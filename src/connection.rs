use crate::event_stream::EventStream;
use crate::reply::*;
use crate::routes::*;
use crate::{EventType, Fallible};
use anyhow::bail;
use bytes::buf::BufExt as _;
use http::header::HeaderValue;
use http::request::Request;
use http::uri::{Authority, Parts as UriParts, PathAndQuery, Scheme, Uri};
use hyper::client::HttpConnector;
use hyper::{Client, Method};
use serde::de::DeserializeOwned as Deserialize;

static API_HEADER_KEY: &str = "X-API-Key";
static API_DEFAULT_AUTHORITY: &str = "127.0.0.1:8384";
static EMPTY_EVENT_SUBSCRIPTION: Vec<EventType> = Vec::new();

pub struct Connection {
    client: Client<HttpConnector>,
    authority: Authority,
    api_key: String,
}

impl Connection {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            authority: Authority::from_static(API_DEFAULT_AUTHORITY),
        }
    }

    pub fn new_with_client(client: Client<HttpConnector>, api_key: impl Into<String>) -> Self {
        Self {
            client,
            api_key: api_key.into(),
            authority: Authority::from_static(API_DEFAULT_AUTHORITY),
        }
    }

    pub fn new_with_authority(api_key: impl Into<String>, authority: Authority) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            authority,
        }
    }

    pub fn new_with_client_and_authority(
        client: Client<HttpConnector>,
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
        std::mem::replace(request.uri_mut(), uri);
        std::mem::replace(request.method_mut(), method);
        request
            .headers_mut()
            .insert(API_HEADER_KEY, HeaderValue::from_str(&self.api_key)?);
        let resp = self.client.request(request).await?;
        let status_code = resp.status().as_u16();
        if status_code < 200 || status_code > 299 {
            bail!("got http status code '{}'", status_code)
        } else {
            let body = hyper::body::aggregate(resp).await?;
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
        if !events.is_empty() {
            let events = serde_json::to_string(&events)?
                .chars()
                .filter(|e| match e {
                    '\"' => false,
                    '[' => false,
                    ']' => false,
                    _ => true,
                })
                .collect::<String>();
            path_and_query.push_str("&events=");
            path_and_query.push_str(events.as_ref());
        }
        if let Some(since) = since {
            path_and_query.push_str("&since=");
            path_and_query.push_str(since.to_string().as_ref());
        }
        if let Some(limit) = limit {
            path_and_query.push_str("&limit=");
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

    pub async fn get_system_log(&self) -> Fallible<SystemLog> {
        self.request(Method::GET, SYSTEM_LOG_PATH).await
    }

    pub async fn get_system_errors(&self) -> Fallible<SystemError> {
        self.request(Method::GET, SYSTEM_ERROR_PATH).await
    }

    pub async fn get_system_ping(&self) -> Fallible<SystemPing> {
        self.request(Method::GET, SYSTEM_PING_PATH).await
    }

    pub async fn get_system_version(&self) -> Fallible<SystemVersion> {
        self.request(Method::GET, SYSTEM_VERSION_PATH).await
    }
}
