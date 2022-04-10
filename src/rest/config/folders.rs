use super::{devices::Device, FolderId};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::path::{Path, PathBuf};

/// Positive number of seconds.
pub type Seconds = u64;
/// Counter starting at zero.
pub type Count = u64;
/// A counter with -1 meaning "infinite".
pub type CountWithInfinite = i64;

/// https://docs.syncthing.net/users/config.html#folder-element
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Folder {
    pub id: FolderId,
    // We can use [`PathBuf`], serialization and deserialization is assumed to be platform
    // independent, but some kinds of path manipulation may not:
    // <https://udoprog.github.io/rust/2017-11-05/portability-concerns-with-path.html>
    // <https://www.reddit.com/r/rust/comments/ft30mm/why_pathbuffrom_str_can_never_fail/>
    pub path: PathBuf,
    #[serde(flatten)]
    pub parameters: Parameters,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Parameters {
    pub label: Option<String>,
    pub filesystem_type: Option<FilesystemType>,
    #[serde(rename = "type")]
    pub ty: Option<Type>,
    pub devices: Option<Vec<Device>>,
    pub rescan_interval_s: Option<Seconds>,
    pub fs_watcher_enabled: Option<bool>,
    pub fs_watcher_delay_s: Option<Seconds>,
    pub ignore_perms: Option<bool>,
    pub auto_normalize: Option<bool>,
    pub min_disk_free: Option<MinDiskFree>,
    pub versioning: Option<Versioning>,
    pub copiers: Option<Count>,
    pub hashers: Option<Count>,
    pub puller_max_pending_ki_b: Option<usize>,
    pub order: Option<PullOrder>,
    pub ignore_delete: Option<bool>,
    pub scan_progress_interval_s: Option<Seconds>,
    pub puller_pause_s: Option<Seconds>,
    pub max_conflicts: Option<CountWithInfinite>,
    pub disable_sparse_files: Option<bool>,
    pub disable_temp_indexes: Option<bool>,
    pub paused: Option<bool>,
    pub weak_hash_threshold_pct: Option<CountWithInfinite>,
    pub marker_name: Option<String>,
    pub copy_ownership_from_parent: Option<bool>,
    pub mod_time_window_s: Option<Seconds>,
    pub max_concurrent_writes: Option<Count>,
    pub disable_fsync: Option<bool>,
    pub block_pull_order: Option<BlockPullOrder>,
    pub copy_range_method: Option<CopyRangeMethod>,
    #[serde(rename = "caseSensitiveFS")]
    pub case_sensitive_fs: Option<bool>,
    pub junctions_as_dirs: Option<bool>,
}

impl Folder {
    pub fn new(id: &str, path: &Path) -> Self {
        Self {
            id: id.to_owned(),
            path: path.to_owned(),
            parameters: Default::default(),
        }
    }
}

/// https://docs.syncthing.net/advanced/folder-filesystem-type.html
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum FilesystemType {
    Basic,
    Fake,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Type {
    SendReceive,
    SendOnly,
    ReceiveOnly,
    ReceiveEncrypted,
}

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

// TODO: Where is the spec for this?
#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Versioning {
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub params: Option<VersioningParams>,
    pub cleanup_interval_s: Option<Seconds>,
    pub fs_path: Option<String>,
    pub fs_type: Option<FilesystemType>,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct VersioningParams {
    // TODO
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum PullOrder {
    Random,
    Alphabetic,
    SmallestFirst,
    LargestFirst,
    OldestFirst,
    NewestFirst,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum BlockPullOrder {
    Standard,
    Random,
    InOrder,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "snake_case", serialize = "snake_case"))]
pub enum CopyRangeMethod {
    Standard,
    CopyFileRange,
    Ioctl,
    SendFile,
    DuplicateExtents,
    All,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::config::test_helper::*;

    #[test]
    fn can_deserialize_folder_with_id_and_path_only() {
        assert_eq!(
            deserialize::<Folder>(r#"{"id": "folder_id", "path": "folder_path"}"#),
            Folder::new("folder_id", Path::new("folder_path"))
        );

        assert_eq!(
            serialize(Folder::new("folder_id", Path::new("folder_path"))),
            r#"{"id":"folder_id","path":"folder_path"}"#
        )
    }

    #[test]
    fn copy_range_method_naming() {
        assert_eq!(serialize(CopyRangeMethod::Ioctl), "\"ioctl\"");
        assert_eq!(
            serialize(CopyRangeMethod::DuplicateExtents),
            "\"duplicate_extents\""
        );
    }

    #[test]
    fn json_normalization_roundtrip() {
        normalization_roundtrip_test::<Folder>(DEFAULT_FOLDER);
    }

    const DEFAULT_FOLDER: &str = r#"
    {
        "id": "",
        "label": "",
        "filesystemType": "basic",
        "path": "~",
        "type": "sendreceive",
        "devices": [
          {
            "deviceID": "WKL2I2O-DTY6W24-00005RS-3N70000-YB6QWT7-PTW6IXI-VCP2SXN-BE70000",
            "introducedBy": "",
            "encryptionPassword": ""
          }
        ],
        "rescanIntervalS": 3600,
        "fsWatcherEnabled": true,
        "fsWatcherDelayS": 10,
        "ignorePerms": false,
        "autoNormalize": true,
        "minDiskFree": {
          "value": 1,
          "unit": "%"
        },
        "versioning": {
          "type": "",
          "params": {},
          "cleanupIntervalS": 3600,
          "fsPath": "",
          "fsType": "basic"
        },
        "copiers": 0,
        "pullerMaxPendingKiB": 0,
        "hashers": 0,
        "order": "random",
        "ignoreDelete": false,
        "scanProgressIntervalS": 0,
        "pullerPauseS": 0,
        "maxConflicts": 10,
        "disableSparseFiles": false,
        "disableTempIndexes": false,
        "paused": false,
        "weakHashThresholdPct": 25,
        "markerName": ".stfolder",
        "copyOwnershipFromParent": false,
        "modTimeWindowS": 0,
        "maxConcurrentWrites": 2,
        "disableFsync": false,
        "blockPullOrder": "standard",
        "copyRangeMethod": "standard",
        "caseSensitiveFS": false,
        "junctionsAsDirs": false
      } "#;
}
