use crate::{DeviceID, FolderID};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Folder {
    pub id: FolderID,
    pub label: String,
    pub path: String,
    pub paused: bool,
    pub devices: Vec<FolderDevice>,
}

#[derive(Deserialize, Debug)]
struct FolderDevice {
    #[serde(rename = "deviceID")]
    device_id: DeviceID,
}
