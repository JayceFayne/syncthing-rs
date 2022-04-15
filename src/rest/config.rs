///! Documentation based on v1.19.2-2-gc001c1e
pub mod devices;
pub mod folders;
pub mod gui;
pub mod ldap;
pub mod options;

pub use devices::Device;
pub use folders::Folder;
pub use gui::Gui;
pub use ldap::Ldap;
pub use options::Options;

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

#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct MinDiskFree {
    value: u64,
    unit: DiskFreeUnit,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
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

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Config {
    pub version: u32,
    pub folders: Vec<Folder>,
    pub devices: Vec<Device>,
    pub gui: Gui,
    pub ldap: Ldap,
    pub options: Options,
    // TODO:
    pub remote_ignored_devices: Vec<serde_json::Value>,
    pub defaults: Defaults,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Defaults {
    pub folder: Folder,
    pub device: Device,
    pub ignores: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_roundtrip() {
        test_helper::normalization_roundtrip_test::<Config>(CONFIG)
    }

    const CONFIG: &str = r#"
    {
    "version": 36,
    "folders": [
      {
        "id": "default",
        "label": "Default Folder",
        "filesystemType": "basic",
        "path": "/var/syncthing/Sync",
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
      },
      {
        "id": "new-folder-id",
        "label": "new-folder-label",
        "filesystemType": "basic",
        "path": "~/new-folder",
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
      }
    ],
    "devices": [
      {
        "deviceID": "WKL2I2O-DTY6W24-00005RS-3N70000-YB6QWT7-PTW6IXI-VCP2SXN-BE70000",
        "name": "192e7158f257",
        "addresses": [
          "dynamic"
        ],
        "compression": "metadata",
        "certName": "",
        "introducer": false,
        "skipIntroductionRemovals": false,
        "introducedBy": "",
        "paused": false,
        "allowedNetworks": [],
        "autoAcceptFolders": false,
        "maxSendKbps": 0,
        "maxRecvKbps": 0,
        "ignoredFolders": [],
        "maxRequestKiB": 0,
        "untrusted": false,
        "remoteGUIPort": 0
      }
    ],
    "gui": {
      "enabled": true,
      "address": "127.0.0.1:8384",
      "unixSocketPermissions": "",
      "user": "",
      "password": "",
      "authMode": "static",
      "useTLS": false,
      "apiKey": "MNfwP2uEyxTTtcVDUpmGz00000000000",
      "insecureAdminAccess": false,
      "theme": "default",
      "debugging": false,
      "insecureSkipHostcheck": false,
      "insecureAllowFrameLoading": false
    },
    "ldap": {
      "address": "",
      "bindDN": "",
      "transport": "plain",
      "insecureSkipVerify": false,
      "searchBaseDN": "",
      "searchFilter": ""
    },
    "options": {
      "listenAddresses": [
        "default"
      ],
      "globalAnnounceServers": [
        "default"
      ],
      "globalAnnounceEnabled": true,
      "localAnnounceEnabled": true,
      "localAnnouncePort": 21027,
      "localAnnounceMCAddr": "[ff12::8384]:21027",
      "maxSendKbps": 0,
      "maxRecvKbps": 0,
      "reconnectionIntervalS": 60,
      "relaysEnabled": true,
      "relayReconnectIntervalM": 10,
      "startBrowser": true,
      "natEnabled": true,
      "natLeaseMinutes": 60,
      "natRenewalMinutes": 30,
      "natTimeoutSeconds": 10,
      "urAccepted": -1,
      "urSeen": 3,
      "urUniqueId": "",
      "urURL": "https://data.syncthing.net/newdata",
      "urPostInsecurely": false,
      "urInitialDelayS": 1800,
      "restartOnWakeup": true,
      "autoUpgradeIntervalH": 12,
      "upgradeToPreReleases": false,
      "keepTemporariesH": 24,
      "cacheIgnoredFiles": false,
      "progressUpdateIntervalS": 5,
      "limitBandwidthInLan": false,
      "minHomeDiskFree": {
        "value": 1,
        "unit": "%"
      },
      "releasesURL": "https://upgrades.syncthing.net/meta.json",
      "alwaysLocalNets": [],
      "overwriteRemoteDeviceNamesOnConnect": false,
      "tempIndexMinBlocks": 10,
      "unackedNotificationIDs": [
        "authenticationUserAndPassword"
      ],
      "trafficClass": 0,
      "setLowPriority": true,
      "maxFolderConcurrency": 0,
      "crURL": "https://crash.syncthing.net/newcrash",
      "crashReportingEnabled": true,
      "stunKeepaliveStartS": 180,
      "stunKeepaliveMinS": 20,
      "stunServers": [
        "default"
      ],
      "databaseTuning": "auto",
      "maxConcurrentIncomingRequestKiB": 0,
      "announceLANAddresses": true,
      "sendFullIndexOnUpgrade": false,
      "featureFlags": [],
      "connectionLimitEnough": 0,
      "connectionLimitMax": 0,
      "insecureAllowOldTLSVersions": false
    },
    "remoteIgnoredDevices": [],
    "defaults": {
      "folder": {
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
      },
      "device": {
        "deviceID": "",
        "name": "",
        "addresses": [
          "dynamic"
        ],
        "compression": "metadata",
        "certName": "",
        "introducer": false,
        "skipIntroductionRemovals": false,
        "introducedBy": "",
        "paused": false,
        "allowedNetworks": [],
        "autoAcceptFolders": false,
        "maxSendKbps": 0,
        "maxRecvKbps": 0,
        "ignoredFolders": [],
        "maxRequestKiB": 0,
        "untrusted": false,
        "remoteGUIPort": 0
      },
      "ignores": {
        "lines": []
      }
    }
  }"#;
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
        use serde_json::Value;
        let value: T = deserialize(json);
        let serialized = serialize(value);
        let normalized: Value = deserialize(&serialized);
        let normalized_expected: Value = deserialize(json);
        assert_eq!(normalized, normalized_expected)
    }
}
