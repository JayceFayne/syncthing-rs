use crate::Fallible;
use crate::event_stream::EventStream;
use http::Method;
use http::header::HeaderValue;
use http::uri::Authority;
use reqwest::Client as HttpClient;
use serde::Serialize;
use serde::de::DeserializeOwned as Deserialize;
use std::collections::HashMap;
use syncthing_types::FolderID;
use syncthing_types::events::{Event, EventType};
use syncthing_types::utils::construct_uri;
use syncthing_types::{API_DEFAULT_AUTHORITY, Timestamp};
use syncthing_types::{API_HEADER_KEY, routes::*};
use syncthing_types::{EMPTY_EVENT_SUBSCRIPTION, system};
use syncthing_types::{cluster, config, db, utils};

pub struct Client {
    client: HttpClient,
    authority: Authority,
    api_key: String,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: HttpClient::new(),
            authority: Authority::from_static(API_DEFAULT_AUTHORITY),
            api_key: api_key.into(),
        }
    }

    pub fn with_authority(api_key: impl Into<String>, authority: Authority) -> Self {
        Self {
            client: HttpClient::new(),
            authority,
            api_key: api_key.into(),
        }
    }

    pub fn authority(&self) -> &Authority {
        &self.authority
    }

    pub(crate) async fn get<D: Deserialize, T: AsRef<[u8]> + 'static>(
        &self,
        path_and_query: T,
    ) -> Fallible<D> {
        let url = construct_uri(&self.authority, path_and_query)?.to_string();
        let resp = self
            .client
            .request(Method::GET, url)
            .header(API_HEADER_KEY, HeaderValue::from_str(&self.api_key)?)
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    pub(crate) async fn post<S: Serialize, T: AsRef<[u8]> + 'static>(
        &self,
        path_and_query: T,
        body: &S,
    ) -> Fallible<()> {
        let url = construct_uri(&self.authority, path_and_query)?.to_string();
        self.client
            .request(Method::POST, url)
            .body(serde_json::to_string(body)?)
            .header(API_HEADER_KEY, HeaderValue::from_str(&self.api_key)?)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
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
        let path_and_query = utils::construct_event_path_and_query(since, limit, events)?;
        self.get(path_and_query).await
    }

    pub fn subscribe_to(self, events: impl Into<Vec<EventType>>) -> EventStream {
        EventStream::new(self, events.into())
    }

    pub fn subscribe_to_all(self) -> EventStream {
        EventStream::new(self, EMPTY_EVENT_SUBSCRIPTION.clone())
    }

    pub async fn browse(&self, pattern: Option<String>) -> Fallible<Vec<String>> {
        if let Some(pattern) = pattern {
            self.get(format!("{}?current={}", SYSTEM_BROWSE, pattern))
                .await
        } else {
            self.get(SYSTEM_BROWSE).await
        }
    }

    pub async fn get_connections(&self) -> Fallible<system::Connections> {
        self.get(SYSTEM_CONNECTIONS).await
    }

    pub async fn get_discovery_cache(&self) -> Fallible<system::Discovery> {
        self.get(SYSTEM_DISCOVERY).await
    }

    pub async fn get_log(&self, since: Option<Timestamp>) -> Fallible<system::Log> {
        if let Some(since) = since {
            self.get(format!("{}?since={}", SYSTEM_LOG, since.to_rfc3339()))
                .await
        } else {
            self.get(SYSTEM_LOG).await
        }
    }

    pub async fn get_errors(&self) -> Fallible<system::Error> {
        self.get(SYSTEM_ERROR).await
    }

    pub async fn clear_errors(&self) -> Fallible<()> {
        self.post(SYSTEM_ERROR_CLEAR, &()).await
    }

    pub async fn get_loglevels_info(&self) -> Fallible<system::LogLevelsInfo> {
        self.get(SYSTEM_LOGLEVELS).await
    }

    pub async fn get_paths(&self) -> Fallible<HashMap<String, String>> {
        self.get(SYSTEM_PATHS).await
    }

    pub async fn ping(&self) -> Fallible<system::Ping> {
        self.get(SYSTEM_PING).await
    }

    pub async fn status(&self) -> Fallible<system::Status> {
        self.get(SYSTEM_STATUS).await
    }

    pub async fn get_upgrade_info(&self) -> Fallible<system::UpgradeInfo> {
        self.get(SYSTEM_UPGRADE).await
    }

    pub async fn get_version_info(&self) -> Fallible<system::VersionInfo> {
        self.get(SYSTEM_VERSION).await
    }

    pub async fn get_cluster_pending_devices(&self) -> Fallible<cluster::PendingDevices> {
        self.get(CLUSTER_PENDING_DEVICES).await
    }

    pub async fn get_config_folders(&self) -> Fallible<Vec<config::Folder>> {
        self.get(CONFIG_FOLDERS).await
    }

    pub async fn get_config_devices(&self) -> Fallible<Vec<config::Device>> {
        self.get(CONFIG_DEVICES).await
    }

    pub async fn get_db_status(&self, folder_id: &FolderID) -> Fallible<db::Status> {
        let mut string: String = DB_STATUS.to_string();
        string.push_str(folder_id);
        self.get(string).await
    }
}
