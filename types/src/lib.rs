pub mod cluster;
pub mod config;
pub mod db;
pub mod events;
pub mod routes;
pub mod system;
#[cfg(feature = "utils")]
pub mod utils;

use chrono::DateTime;
use chrono::FixedOffset;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static API_HEADER_KEY: &str = "X-API-Key";
pub static API_DEFAULT_AUTHORITY: &str = "127.0.0.1:8384";
pub static EMPTY_EVENT_SUBSCRIPTION: Vec<crate::events::EventType> = Vec::new();

//TODO: ip type for address, DeviceID/FolderID type with deser
//FIXME: check folder == folderLable inconsistency

type FileName = String;
//TODO: use separate type?
type DeviceID = String;
pub type FolderID = String;
type FolderName = String;
type Folder = HashMap<FileName, File>;
pub type Timestamp = DateTime<FixedOffset>;

//TODO: maybe move to events if not used in system
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct File {
    pub total: u64,
    pub pulling: u64,
    pub copied_from_origin: u64,
    pub reused: u64,
    pub copied_from_elsewhere: u64,
    pub pulled: u64,
    pub bytes_total: u64,
    pub bytes_done: u64,
}
