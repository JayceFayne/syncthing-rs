use crate::DeviceID;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Device {
    #[serde(rename = "deviceID")]
    pub id: DeviceID,
    pub name: String,
    pub addresses: Vec<String>,
}
