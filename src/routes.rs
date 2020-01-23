use url::{ParseError, Url};

#[derive(Clone)]
pub(crate) struct Routes {
    pub events: Url,
    pub system_log: Url,
    pub system_error: Url,
    pub system_ping: Url,
    pub system_version: Url,
}

impl Routes {
    pub fn new_with_base_url(base_url: impl AsRef<str>) -> Result<Self, ParseError> {
        let base_url = Url::parse(base_url.as_ref())?.join("rest/")?;
        Ok(Self {
            events: base_url.join("events")?,
            system_log: base_url.join("system/log")?,
            system_error: base_url.join("system/error")?,
            system_ping: base_url.join("system/ping")?,
            system_version: base_url.join("system/version")?,
        })
    }
}
