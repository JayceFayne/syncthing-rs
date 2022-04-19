use crate::event_stream::EventStream;
use crate::rest::events::{Event, EventType};
use crate::rest::{config, system};
use crate::routes::*;
use crate::utils::QueryChars;
use crate::Fallible;
use anyhow::bail;
use bytes::buf::BufExt as _;
use bytes::Buf;
use http::header::HeaderValue;
use http::request::Request;
use http::uri::{Authority, Parts as UriParts, PathAndQuery, Scheme, Uri};
use hyper::client::HttpConnector;
use hyper::{Body, Client as HyperClient, Method};
use serde::de::DeserializeOwned as Deserialize;
use serde::Serialize;

static API_HEADER_KEY: &str = "X-API-Key";
static API_DEFAULT_AUTHORITY: &str = "127.0.0.1:8384";
static EMPTY_EVENT_SUBSCRIPTION: Vec<EventType> = Vec::new();

pub struct Client {
    client: HyperClient<HttpConnector>,
    authority: Authority,
    api_key: String,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: HyperClient::new(),
            api_key: api_key.into(),
            authority: Authority::from_static(API_DEFAULT_AUTHORITY),
        }
    }

    pub fn new_with_hyper_client(
        client: HyperClient<HttpConnector>,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            client,
            api_key: api_key.into(),
            authority: Authority::from_static(API_DEFAULT_AUTHORITY),
        }
    }

    pub fn new_with_authority(api_key: impl Into<String>, authority: Authority) -> Self {
        Self {
            client: HyperClient::new(),
            api_key: api_key.into(),
            authority,
        }
    }

    pub fn new_with_hyper_client_and_authority(
        client: HyperClient<HttpConnector>,
        api_key: impl Into<String>,
        authority: Authority,
    ) -> Self {
        Self {
            client,
            api_key: api_key.into(),
            authority,
        }
    }

    pub(crate) async fn request<D: Deserialize, T: AsRef<[u8]> + 'static>(
        &self,
        method: Method,
        path_and_query: T,
    ) -> Fallible<D> {
        let uri = self.request_uri(path_and_query)?;
        let mut request = Request::new(Default::default());
        *request.uri_mut() = uri;
        *request.method_mut() = method;
        request
            .headers_mut()
            .insert(API_HEADER_KEY, HeaderValue::from_str(&self.api_key)?);
        let resp = self.client.request(request).await?;
        let status_code = resp.status().as_u16();
        let body = hyper::body::aggregate(resp).await?;
        if !(200..=299).contains(&status_code) {
            bail!(
                "got http status code '{}' with following msg:\n {}",
                status_code,
                String::from_utf8_lossy(body.bytes())
            )
        } else {
            Ok(serde_json::from_reader(body.reader())?)
        }
    }

    pub(crate) async fn send<P: AsRef<[u8]> + 'static, V: Serialize>(
        &self,
        method: Method,
        path_and_query: P,
        value: V,
    ) -> Fallible<()> {
        let uri = self.request_uri(path_and_query)?;
        let json = serde_json::to_string_pretty(&value)?;
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header(API_HEADER_KEY, HeaderValue::from_str(&self.api_key)?)
            .header("content-type", "application/json")
            .body(Body::from(json))?;
        let response = self.client.request(request).await?;
        let status_code = response.status().as_u16();
        let body = hyper::body::aggregate(response).await?;
        if !(200..=299).contains(&status_code) {
            bail!(
                "got http status code '{}' with following msg:\n {}",
                status_code,
                String::from_utf8_lossy(body.bytes())
            )
        }

        Ok(())
    }

    pub(crate) async fn delete<P: AsRef<[u8]> + 'static>(&self, path_and_query: P) -> Fallible<()> {
        let uri = self.request_uri(path_and_query)?;
        let request = Request::builder()
            .method(Method::DELETE)
            .uri(uri)
            .header(API_HEADER_KEY, HeaderValue::from_str(&self.api_key)?)
            .body(Body::empty())?;
        let response = self.client.request(request).await?;
        let status_code = response.status().as_u16();
        let body = hyper::body::aggregate(response).await?;
        if !(200..=299).contains(&status_code) {
            bail!(
                "got http status code '{}' with following msg:\n {}",
                status_code,
                String::from_utf8_lossy(body.bytes())
            )
        }
        Ok(())
    }

    fn request_uri<P: AsRef<[u8]> + 'static>(&self, path_and_query: P) -> Fallible<Uri> {
        let mut uri_parts = UriParts::default();
        uri_parts.authority = Some(self.authority.clone());
        uri_parts.scheme = Some(Scheme::HTTP);
        uri_parts.path_and_query = Some(PathAndQuery::from_maybe_shared(path_and_query)?);
        Ok(Uri::from_parts(uri_parts)?)
    }

    pub async fn get_all_events(
        &self,
        since: Option<u64>,
        limit: Option<u64>,
    ) -> Fallible<Vec<Event>> {
        self.get_events(since, limit, &EMPTY_EVENT_SUBSCRIPTION)
            .await
    }

    pub async fn get_events(
        &self,
        since: Option<u64>,
        limit: Option<u64>,
        events: impl AsRef<[EventType]>,
    ) -> Fallible<Vec<Event>> {
        let mut path_and_query = EVENTS_PATH.to_owned();
        let events = events.as_ref();
        let mut query_chars = QueryChars::new();
        if !events.is_empty() {
            let events = serde_json::to_string(&events)?
                .chars()
                .filter(|e| !matches!(e, '\"' | '[' | ']'))
                .collect::<String>();
            path_and_query.push(query_chars.next_char());
            path_and_query.push_str("events=");
            path_and_query.push_str(events.as_ref());
        }
        if let Some(since) = since {
            path_and_query.push(query_chars.next_char());
            path_and_query.push_str("since=");
            path_and_query.push_str(since.to_string().as_ref());
        }
        if let Some(limit) = limit {
            path_and_query.push(query_chars.next_char());
            path_and_query.push_str("limit=");
            path_and_query.push_str(limit.to_string().as_ref());
        }
        self.request(Method::GET, path_and_query).await
    }

    pub fn subscribe_to(self, events: impl Into<Vec<EventType>>) -> EventStream {
        EventStream::new(self, events.into())
    }

    pub fn subscribe_to_all(self) -> EventStream {
        EventStream::new(self, EMPTY_EVENT_SUBSCRIPTION.clone())
    }

    pub async fn get_system_connections(&self) -> Fallible<system::connections::Connections> {
        self.request(Method::GET, SYSTEM_CONNECTIONS_PATH).await
    }

    pub async fn get_system_debug(&self) -> Fallible<system::debug::DebugInfo> {
        self.request(Method::GET, SYSTEM_DEBUG_PATH).await
    }

    pub async fn get_system_discovery(&self) -> Fallible<system::discovery::Discovery> {
        self.request(Method::GET, SYSTEM_DISCOVERY_PATH).await
    }

    pub async fn get_system_log(&self) -> Fallible<system::log::Log> {
        self.request(Method::GET, SYSTEM_LOG_PATH).await
    }

    pub async fn get_system_error(&self) -> Fallible<system::error::Error> {
        self.request(Method::GET, SYSTEM_ERROR_PATH).await
    }

    pub async fn get_system_ping(&self) -> Fallible<system::ping::Ping> {
        self.request(Method::GET, SYSTEM_PING_PATH).await
    }

    pub async fn get_system_upgrade(&self) -> Fallible<system::upgrade::UpgradeInfo> {
        self.request(Method::GET, SYSTEM_UPGRADE_PATH).await
    }

    pub async fn get_system_version(&self) -> Fallible<system::version::Version> {
        self.request(Method::GET, SYSTEM_VERSION_PATH).await
    }

    // /rest/config

    pub async fn get_config(&self) -> Fallible<config::Config> {
        self.request(Method::GET, CONFIG_PATH).await
    }

    pub async fn get_config_version(&self) -> Fallible<config::Version> {
        let version: config::VersionOnly = self.request(Method::GET, CONFIG_PATH).await?;
        Ok(version.version)
    }

    pub async fn put_config(&self, config: &config::Config) -> Fallible<()> {
        self.send(Method::PUT, CONFIG_PATH, config).await
    }

    pub async fn is_restart_required(&self) -> Fallible<bool> {
        let restart_required: config::RestartRequired = self
            .request(Method::GET, CONFIG_RESTART_REQUIRED_PATH)
            .await?;
        Ok(restart_required.requires_restart)
    }

    // config/folders

    pub async fn get_config_folders(&self) -> Fallible<Vec<config::Folder>> {
        self.request(Method::GET, CONFIG_FOLDERS_PATH).await
    }

    pub async fn put_config_folders(&self, folders: &[config::Folder]) -> Fallible<()> {
        self.send(Method::PUT, CONFIG_FOLDERS_PATH, folders).await
    }

    pub async fn post_config_folders(&self, folder: &config::Folder) -> Fallible<()> {
        self.send(Method::POST, CONFIG_FOLDERS_PATH, folder).await
    }

    // config/folders/*id*

    pub async fn get_config_folder(&self, id: impl AsRef<str>) -> Fallible<config::Folder> {
        let id = id.as_ref();
        self.request(Method::GET, format!("{CONFIG_FOLDERS_PATH}/{id}"))
            .await
    }

    pub async fn put_config_folder(&self, folder: &config::Folder) -> Fallible<()> {
        let id = &folder.id;
        self.send(Method::PUT, format!("{CONFIG_FOLDERS_PATH}/{id}"), folder)
            .await
    }

    pub async fn delete_config_folder(&self, id: impl AsRef<str>) -> Fallible<()> {
        let id = id.as_ref();
        self.delete(format!("{CONFIG_FOLDERS_PATH}/{id}")).await
    }

    // config/devices

    pub async fn get_config_devices(&self) -> Fallible<Vec<config::Device>> {
        self.request(Method::GET, CONFIG_DEVICES_PATH).await
    }

    pub async fn put_config_devices(&self, devices: &[config::Device]) -> Fallible<()> {
        self.send(Method::PUT, CONFIG_DEVICES_PATH, devices).await
    }

    pub async fn post_config_devices(&self, device: &config::Device) -> Fallible<()> {
        self.send(Method::POST, CONFIG_DEVICES_PATH, device).await
    }

    // config/devices/*id*

    pub async fn get_config_device(&self, id: impl AsRef<str>) -> Fallible<config::Device> {
        let id = id.as_ref();
        self.request(Method::GET, format!("{CONFIG_DEVICES_PATH}/{id}"))
            .await
    }

    pub async fn put_config_device(&self, device: &config::Device) -> Fallible<()> {
        let id = &device.id;
        self.send(Method::PUT, format!("{CONFIG_DEVICES_PATH}/{id}"), device)
            .await
    }

    pub async fn delete_config_device(&self, id: impl AsRef<str>) -> Fallible<()> {
        let id = id.as_ref();
        self.delete(format!("{CONFIG_DEVICES_PATH}/{id}")).await
    }

    // config/defaults/folder

    pub async fn get_config_defaults_folder(&self) -> Fallible<config::Folder> {
        self.request(Method::GET, CONFIG_DEFAULTS_FOLDER_PATH).await
    }

    pub async fn put_config_defaults_folder(&self, folder: &config::Folder) -> Fallible<()> {
        self.send(Method::PUT, CONFIG_DEFAULTS_FOLDER_PATH, folder)
            .await
    }

    // config/defaults/device

    pub async fn get_config_defaults_device(&self) -> Fallible<config::Device> {
        self.request(Method::GET, CONFIG_DEFAULTS_DEVICE_PATH).await
    }

    pub async fn put_config_defaults_device(&self, device: &config::Device) -> Fallible<()> {
        self.send(Method::PUT, CONFIG_DEFAULTS_DEVICE_PATH, device)
            .await
    }

    // config/defaults/ignores

    pub async fn get_config_defaults_ignores(&self) -> Fallible<config::Ignores> {
        self.request(Method::GET, CONFIG_DEFAULTS_IGNORES_PATH)
            .await
    }

    pub async fn put_config_defaults_ignores(&self, ignores: &config::Ignores) -> Fallible<()> {
        self.send(Method::PUT, CONFIG_DEFAULTS_IGNORES_PATH, ignores)
            .await
    }

    // config/options

    pub async fn get_config_options(&self) -> Fallible<config::Options> {
        self.request(Method::GET, CONFIG_OPTIONS_PATH).await
    }

    pub async fn put_config_options(&self, options: &config::Options) -> Fallible<()> {
        self.send(Method::PUT, CONFIG_OPTIONS_PATH, options).await
    }

    // config/ldap

    pub async fn get_config_ldap(&self) -> Fallible<config::Ldap> {
        self.request(Method::GET, CONFIG_LDAP_PATH).await
    }

    pub async fn put_config_ldap(&self, ldap: &config::Ldap) -> Fallible<()> {
        self.send(Method::PUT, CONFIG_LDAP_PATH, ldap).await
    }

    // config/gui

    pub async fn get_config_gui(&self) -> Fallible<config::Gui> {
        self.request(Method::GET, CONFIG_GUI_PATH).await
    }

    pub async fn put_config_gui(&self, gui: &config::Gui) -> Fallible<()> {
        self.send(Method::PUT, CONFIG_GUI_PATH, gui).await
    }
}
