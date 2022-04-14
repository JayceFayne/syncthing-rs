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
use hyper::{Client as HyperClient, Method};
use serde::de::DeserializeOwned as Deserialize;

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
        let mut uri_parts = UriParts::default();
        uri_parts.authority = Some(self.authority.clone());
        uri_parts.scheme = Some(Scheme::HTTP);
        uri_parts.path_and_query = Some(PathAndQuery::from_maybe_shared(path_and_query)?);
        let uri = Uri::from_parts(uri_parts)?;
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

    pub async fn get_config_folders(&self) -> Fallible<Vec<config::folders::Folder>> {
        self.request(Method::GET, CONFIG_FOLDERS_PATH).await
    }

    pub async fn get_config_devices(&self) -> Fallible<Vec<config::devices::Device>> {
        self.request(Method::GET, CONFIG_DEVICES_PATH).await
    }
}
