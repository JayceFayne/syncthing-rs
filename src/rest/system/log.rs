use crate::rest::system::Entry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Log {
    pub messages: Vec<Entry>,
}
