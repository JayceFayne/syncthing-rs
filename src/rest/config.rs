///! Documentation based on v1.19.2-2-gc001c1e
pub mod devices;
pub mod folders;
pub mod gui;
pub mod ldap;
pub mod options;

use devices::Device;
use folders::Folder;
use serde::{Deserialize, Serialize};

pub type FolderId = String;
pub type Version = u32;
pub type PortNumber = u16;
pub type Kibibytes = u64;
pub type KibibytesPerSecond = u64;
/// Positive number of seconds.
pub type Seconds = u32;
pub type Minutes = u32;
pub type Hours = u32;
/// Counter starting at zero.
pub type Count = u64;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct MinDiskFree {
    value: u64,
    unit: DiskFreeUnit,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DiskFreeUnit {
    #[serde(rename = "0")]
    Disabled,
    #[serde(rename = "%")]
    Percentage,
    #[serde(rename = "kB")]
    Kilobyte,
    #[serde(rename = "MB")]
    Megabyte,
    #[serde(rename = "GB")]
    Gigabyte,
    #[serde(rename = "TB")]
    Terrabyte,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Config {
    pub version: u32,
    pub folders: Vec<Folder>,
    pub devices: Vec<Device>,
    // TODO
    pub gui: serde_json::Value,
    // TODO
    pub ldap: serde_json::Value,
    // TODO
    pub options: serde_json::Value,
    // TODO
    pub remote_ignored_devices: Vec<serde_json::Value>,
    pub defaults: Defaults,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Defaults {
    pub folder: Folder,
    pub device: Device,
    pub ignores: serde_json::Value,
}

#[cfg(test)]
mod test_helper {
    use serde::{Deserialize, Serialize};

    pub fn deserialize<'a, T: Deserialize<'a>>(json: &'a str) -> T {
        serde_json::from_str(json).unwrap()
    }

    pub fn serialize<T: Serialize>(value: T) -> String {
        serde_json::to_string(&value).unwrap()
    }

    /// Deserialize `json` into `T`, then serialize it, deserialize it again into a
    /// [`serde_json::Value`] and compare it with a directly [`serde_json::Value`] normalized
    /// representation.
    pub fn normalization_roundtrip_test<T: Serialize + for<'a> Deserialize<'a>>(json: &str) {
        let value: T = deserialize(json);
        let serialized = serialize(value);
        let normalized: serde_json::Value = deserialize(&serialized);
        let normalized_expected: serde_json::Value = deserialize(json);
        assert_eq!(normalized, normalized_expected)
    }
}
