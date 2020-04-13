use crate::rest::system::Entry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Error {
    pub errors: Vec<Entry>,
}
