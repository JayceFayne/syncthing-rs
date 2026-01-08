use crate::Fallible;
use crate::event_stream::EventStream;
use http::header::HeaderValue;
use http::uri::Authority;
use http::{Method, Request};
use serde::Serialize;
use serde::de::DeserializeOwned as Deserialize;
use std::collections::HashMap;
use syncthing_types::events::{Event, EventType};
use syncthing_types::utils::construct_uri;
use syncthing_types::{API_DEFAULT_AUTHORITY, FolderID, Timestamp};
use syncthing_types::{API_HEADER_KEY, routes::*};
use syncthing_types::{EMPTY_EVENT_SUBSCRIPTION, system};
use syncthing_types::{cluster, config, db, utils};
use ureq::Agent;

pub struct Client {
    agent: Agent,
    authority: Authority,
    api_key: String,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            agent: Agent::new_with_defaults(),
            authority: Authority::from_static(API_DEFAULT_AUTHORITY),
            api_key: api_key.into(),
        }
    }

    pub fn with_authority(api_key: impl Into<String>, authority: Authority) -> Fallible<Self> {
        Ok(Self {
            agent: Agent::new_with_defaults(),
            authority,
            api_key: api_key.into(),
        })
    }

    pub(crate) fn get<D: Deserialize, T: AsRef<[u8]> + 'static>(
        &self,
        path_and_query: T,
    ) -> Fallible<D> {
        let uri = construct_uri(&self.authority, path_and_query)?;
        let mut request = Request::new(());
        *request.uri_mut() = uri;
        *request.method_mut() = Method::GET;
        request
            .headers_mut()
            .insert(API_HEADER_KEY, HeaderValue::from_str(&self.api_key)?);
        let mut resp = self.agent.run(request)?;
        Ok(serde_json::from_reader(resp.body_mut().as_reader())?)
    }

    pub(crate) fn post<S: Serialize, T: AsRef<[u8]> + 'static>(
        &self,
        path_and_query: T,
        body: &S,
    ) -> Fallible<()> {
        let uri = construct_uri(&self.authority, path_and_query)?;
        let mut request = Request::new(serde_json::to_string(body)?);
        *request.uri_mut() = uri;
        *request.method_mut() = Method::POST;
        request
            .headers_mut()
            .insert(API_HEADER_KEY, HeaderValue::from_str(&self.api_key)?);
        self.agent.run(request)?;
        Ok(())
    }

    pub fn get_all_events(&self, since: Option<u64>, limit: Option<u64>) -> Fallible<Vec<Event>> {
        self.get_events(since, limit, &EMPTY_EVENT_SUBSCRIPTION)
    }

    pub fn get_events(
        &self,
        since: Option<u64>,
        limit: Option<u64>,
        events: impl AsRef<[EventType]>,
    ) -> Fallible<Vec<Event>> {
        let path_and_query = utils::construct_event_path_and_query(since, limit, events)?;
        self.get(path_and_query)
    }

    pub fn subscribe_to(self, events: impl Into<Vec<EventType>>) -> EventStream {
        EventStream::new(self, events.into())
    }

    pub fn subscribe_to_all(self) -> EventStream {
        EventStream::new(self, EMPTY_EVENT_SUBSCRIPTION.clone())
    }

    pub fn browse(&self, pattern: Option<String>) -> Fallible<Vec<String>> {
        if let Some(pattern) = pattern {
            self.get(format!("{}?current={}", SYSTEM_BROWSE, pattern))
        } else {
            self.get(SYSTEM_BROWSE)
        }
    }

    pub fn get_connections(&self) -> Fallible<system::Connections> {
        self.get(SYSTEM_CONNECTIONS)
    }

    pub fn get_discovery_cache(&self) -> Fallible<system::Discovery> {
        self.get(SYSTEM_DISCOVERY)
    }

    pub fn get_log(&self, since: Option<Timestamp>) -> Fallible<system::Log> {
        if let Some(since) = since {
            self.get(format!("{}?since={}", SYSTEM_LOG, since.to_rfc3339()))
        } else {
            self.get(SYSTEM_LOG)
        }
    }

    pub fn get_errors(&self) -> Fallible<system::Error> {
        self.get(SYSTEM_ERROR)
    }

    pub fn clear_errors(&self) -> Fallible<()> {
        self.post(SYSTEM_ERROR_CLEAR, &())
    }

    pub fn get_loglevels_info(&self) -> Fallible<system::LogLevelsInfo> {
        self.get(SYSTEM_LOGLEVELS)
    }

    pub fn get_paths(&self) -> Fallible<HashMap<String, String>> {
        self.get(SYSTEM_PATHS)
    }

    pub fn ping(&self) -> Fallible<system::Ping> {
        self.get(SYSTEM_PING)
    }

    pub fn status(&self) -> Fallible<system::Status> {
        self.get(SYSTEM_STATUS)
    }

    pub fn get_upgrade_info(&self) -> Fallible<system::UpgradeInfo> {
        self.get(SYSTEM_UPGRADE)
    }

    pub fn get_version_info(&self) -> Fallible<system::VersionInfo> {
        self.get(SYSTEM_VERSION)
    }

    pub fn get_cluster_pending_devices(&self) -> Fallible<cluster::PendingDevices> {
        self.get(CLUSTER_PENDING_DEVICES)
    }

    pub fn get_config_folders(&self) -> Fallible<config::Folder> {
        self.get(CONFIG_FOLDERS)
    }

    pub fn get_config_devices(&self) -> Fallible<config::Device> {
        self.get(CONFIG_DEVICES)
    }

    pub fn get_db_status(&self, folder_id: &FolderID) -> Fallible<db::Status> {
        let mut string = DB_STATUS.to_string();
        string.push_str(folder_id);
        self.get(string)
    }
}
