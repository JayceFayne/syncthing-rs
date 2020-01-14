use crate::reply::*;
use crate::routes::Routes;
use crate::{EventStream, EventType, Fallible, API_HEADER_KEY, API_URL};
use anyhow::format_err;
use http::Method;
use http_client::HttpClient;
use serde::de::DeserializeOwned as Deserialize;
use serde_json::to_string as json_to_string;
use std::convert::TryFrom;
use surf::Request;
use url::{ParseError, Url};

pub struct Connection<C> {
    client: C,
    routes: Routes,
    api_key: String,
}

impl<C> Connection<C> {
    pub fn new(client: C, api_key: impl Into<String>) -> Result<Self, ParseError> {
        Self::new_with_base_url(client, api_key, API_URL)
    }

    pub fn new_with_base_url(
        client: C,
        api_key: impl Into<String>,
        base_url: impl AsRef<str>,
    ) -> Result<Self, ParseError> {
        Ok(Self {
            client,
            api_key: api_key.into(),
            routes: Routes::new_with_base_url(base_url)?,
        })
    }
}

impl<C: HttpClient> Connection<C> {
    pub(crate) async fn request<D: Deserialize>(&self, method: Method, url: Url) -> Fallible<D> {
        let mut resp = Request::with_client(method, url, self.client.clone())
            .set_header(API_HEADER_KEY, &self.api_key)
            .await?;
        let status_code = resp.status().as_u16();
        if status_code < 200 || status_code > 299 {
            Err(format_err!("http status code '{}'", status_code).into())
        } else {
            Ok(resp.body_json().await?)
        }
    }

    pub async fn get_events(
        &self,
        events: Option<&[EventType]>,
        since: Option<u64>,
        limit: Option<u64>,
    ) -> Fallible<Vec<Event>> {
        let mut query = String::new();
        if let Some(events) = events {
            let events = json_to_string(&events)?
                .chars()
                .filter(|e| match e {
                    '\"' => false,
                    '[' => false,
                    ']' => false,
                    _ => true,
                })
                .collect::<String>();
            query.push_str(&format!("&events={}", events));
        }
        if let Some(since) = since {
            query.push_str(&format!("&since={}", since));
        }
        if let Some(limit) = limit {
            query.push_str(&format!("&limit={}", limit));
        }
        let mut url = self.routes.events.clone();
        url.set_query(Some(&query));
        let raw_events: Vec<RawEvent> = self.request(Method::GET, url).await?;
        let mut events = Vec::with_capacity(raw_events.len());
        for raw_event in raw_events {
            events.push(Event::try_from(raw_event)?);
        }
        Ok(events)
    }

    pub fn subscribe(self, events: Option<Vec<EventType>>) -> EventStream<C> {
        EventStream::new(self, events)
    }

    pub async fn get_system_log(&self) -> Fallible<SystemLog> {
        self.request(Method::GET, self.routes.system_log.clone())
            .await
    }

    pub async fn get_system_ping(&self) -> Fallible<SystemPing> {
        self.request(Method::GET, self.routes.system_ping.clone())
            .await
    }

    pub async fn get_system_version(&self) -> Fallible<SystemVersion> {
        self.request(Method::GET, self.routes.system_version.clone())
            .await
    }
}
