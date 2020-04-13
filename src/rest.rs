pub mod events;
pub mod system;

use serde::Deserialize;
use std::collections::HashMap;

//TODO: ip type for address, DeviceID/FolderID type with deser
//FIXME: check folder == folderLable inconsistency

type FileName = String;
//TODO: use separate type?
type DeviceID = String;
type FolderName = String;
type Folder = HashMap<FileName, File>;

//TODO: maybe move to events if not used in system
#[derive(Debug, Deserialize)]
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
