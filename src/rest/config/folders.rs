use super::{Count, FolderId, Kibibytes, MinDiskFree, Seconds};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A counter with -1 meaning "infinite".
pub type CountWithInfinite = i64;

/// <https://docs.syncthing.net/users/config.html#folder-element>
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Folder {
    /// The folder ID, which must be unique.
    pub id: FolderId,
    /// The path to the directory where the folder is stored on this device; not sent to other
    /// devices.
    // We can use [`PathBuf`], serialization and deserialization is assumed to be platform
    // independent, but some kinds of path manipulation may not:
    // <https://udoprog.github.io/rust/2017-11-05/portability-concerns-with-path.html>
    // <https://www.reddit.com/r/rust/comments/ft30mm/why_pathbuffrom_str_can_never_fail/>
    pub path: PathBuf,
    /// The label of a folder is a human readable and descriptive local name. May be different on
    /// each device, empty, and/or identical to other folder labels. (optional)
    pub label: String,
    /// The internal file system implementation used to access this folder, detailed in a [separate
    /// chapter][1].
    ///
    /// [1]: <https://docs.syncthing.net/advanced/folder-filesystem-type.html>
    pub filesystem_type: FilesystemType,
    /// Controls how the folder is handled by Syncthing.
    #[serde(rename = "type")]
    pub ty: Type,
    /// These must have the `id` attribute and can have an `introduced_by` attribute, identifying
    /// the device that introduced us to share this folder with the given device. If the original
    /// introducer unshares this folder with this device, our device will follow and unshare the
    /// folder (subject to `skip_introduction_removals` being `false` on the introducer device).
    ///
    /// All mentioned devices are those that will be sharing the folder in question. Each mentioned
    /// device must have a separate `device` element later in the file. It is customary that the
    /// local device ID is included in all folders. Syncthing will currently add this automatically
    /// if it is not present in the configuration file.
    ///
    /// The `encryption_password` sub-element contains the secret needed to decrypt this folder’s
    /// data on the remote device. If left empty, the data is plainly accessible (but still
    /// protected by the transport encryption). The mechanism and how to set it up is described in a
    /// [separate chapter][1].
    ///
    /// [1]: <https://docs.syncthing.net/users/untrusted.html>
    pub devices: Vec<Device>,
    /// The rescan interval, in seconds. Can be set to `0` to disable when external plugins are used
    /// to trigger rescans.
    pub rescan_interval_s: Seconds,
    /// If set to `true`, this detects changes to files in the folder and scans them.
    pub fs_watcher_enabled: bool,
    /// The duration during which changes detected are accumulated, before a scan is scheduled (only
    /// takes effect if `fs_watcher_enabled` is set to `true`).
    pub fs_watcher_delay_s: Seconds,
    /// If `true`, files originating from this folder will be announced to remote devices with the
    /// “no permission bits” flag. The remote devices will use whatever their default permission
    /// setting is when creating the files. The primary use case is for file systems that do not
    /// support permissions, such as FAT, or environments where changing permissions is impossible.
    pub ignore_perms: bool,
    /// Automatically correct UTF-8 normalization errors found in file names. The mechanism and how
    /// to set it up is described in a [separate chapter][1].
    ///
    /// [1]: <https://docs.syncthing.net/advanced/folder-autonormalize.html>
    pub auto_normalize: bool,
    /// The minimum required free space that should be available on the disk this folder resides.
    /// The folder will be stopped when the value drops below the threshold. The element content is
    /// interpreted according to the given unit attribute. Accepted unit values are `%` (percent of
    /// the disk / volume size), `kB`, `MB`, `GB` and `TB`. Set to zero to disable.
    pub min_disk_free: MinDiskFree,
    /// Specifies a versioning configuration.
    ///
    /// See also [Versioning](<https://docs.syncthing.net/users/versioning.html>)
    pub versioning: Versioning,
    /// The number of copier and hasher routines to use, or `0` for the system determined optimums.
    /// These are low-level performance options for advanced users only; do not change unless
    /// requested to or you’ve actually read and understood the code yourself. :)
    pub copiers: Count,
    pub hashers: Count,
    /// Controls when we stop sending requests to other devices once we’ve got this much unserved
    /// requests. The number of pullers is automatically adjusted based on this desired amount of
    /// outstanding request data.
    #[serde(rename = "pullerMaxPendingKiB")]
    pub puller_max_pending_kib: Kibibytes,
    /// The order in which needed files should be pulled from the cluster. It has no effect when the
    /// folder type is “send only”.
    ///
    /// Note that the scanned files are sent in batches and the sorting is applied only to the
    /// already discovered files. This means the sync might start with a 1 GB file even if there is
    /// 1 KB file available on the source device until the 1 KB becomes known to the pulling device.
    pub order: PullOrder,
    /// When set to `true`, this device will pretend not to see instructions to delete files from
    /// other devices. The mechanism is described in a [separate chapter][1].
    ///
    /// WARNING: Enabling this is highly discouraged - use at your own risk. You have been warned.
    ///
    /// [1]: https://docs.syncthing.net/advanced/folder-ignoredelete.html
    pub ignore_delete: bool,
    /// The interval in seconds with which scan progress information is sent to the GUI. Setting to
    /// `0` will cause Syncthing to use the default value of two.
    pub scan_progress_interval_s: Seconds,
    /// Tweak for rate limiting the puller when it retries pulling files. Don’t change this unless
    /// you know what you’re doing.
    pub puller_pause_s: Seconds,
    /// The maximum number of conflict copies to keep around for any given file. The default, `-1`,
    /// means an unlimited number. Setting this to `0` disables conflict copies altogether.
    pub max_conflicts: CountWithInfinite,
    /// By default, blocks containing all zeros are not written, causing files to be sparse on
    /// filesystems that support this feature. When set to `true`, sparse files will not be created.
    pub disable_sparse_files: bool,
    /// By default, devices exchange information about blocks available in transfers that are still
    /// in progress, which allows other devices to download parts of files that are not yet fully
    /// downloaded on your own device, essentially making transfers more torrent like. When set to
    /// `true`, such information is not exchanged for this folder.
    pub disable_temp_indexes: bool,
    /// `true` if this folder is (temporarily) suspended.
    pub paused: bool,
    /// Use weak hash if more than the given percentage of the file has changed. Set to `-1` to
    /// always use weak hash. Default is `25`.
    pub weak_hash_threshold_pct: CountWithInfinite,
    /// Name of a directory or file in the folder root to be used as [How do I serve a folder from a
    /// read only filesystem?][1]. Default is `.stfolder`.
    ///
    /// [1]: https://docs.syncthing.net/users/faq.html#marker-faq
    pub marker_name: String,
    /// On Unix systems, tries to copy file/folder ownership from the parent directory (the
    /// directory it’s located in). Requires running Syncthing as a privileged user, or granting it
    /// additional capabilities (e.g. `CAP_CHOWN` on Linux).
    pub copy_ownership_from_parent: bool,
    /// Allowed modification timestamp difference when comparing files for equivalence. To be used
    /// on file systems which have unstable modification timestamps that might change after being
    /// recorded during the last write operation. Default is `2` on Android when the folder is
    /// located on a FAT partition, and `0` otherwise.
    pub mod_time_window_s: Seconds,
    /// Maximum number of concurrent write operations while syncing. Increasing this might increase
    /// or decrease disk performance, depending on the underlying storage. Default is `2`.
    pub max_concurrent_writes: Count,
    /// Disables committing file operations to disk before recording them in the database. Disabling
    /// fsync can lead to data corruption. The mechanism is described in a [separate chapter][1].
    ///
    /// WARNING: This is a known insecure option - use at your own risk.
    ///
    /// [1]: <https://docs.syncthing.net/advanced/folder-disable-fsync.html>
    pub disable_fsync: bool,
    /// Order in which the blocks of a file are downloaded. This option controls how quickly
    /// different parts of the file spread between the connected devices, at the cost of causing
    /// strain on the storage.
    pub block_pull_order: BlockPullOrder,
    /// Provides a choice of method for copying data between files. This can be used to optimize
    /// copies on network filesystems, improve speed of large copies or clone the data using
    /// copy-on-write functionality if the underlying filesystem supports it. The mechanism is
    /// described in a [separate chapter][1].
    ///
    /// [1]: <https://docs.syncthing.net/advanced/folder-copyrangemethod.html>
    pub copy_range_method: CopyRangeMethod,
    /// Affects performance by disabling the extra safety checks for case insensitive filesystems.
    /// The mechanism and how to set it up is described in a [separate chapter][1].
    ///
    /// [1]: <https://docs.syncthing.net/advanced/folder-casesensitivefs>
    #[serde(rename = "caseSensitiveFS")]
    pub case_sensitive_fs: bool,
    /// NTFS directory junctions are treated as ordinary directories, if this is set to `true`.
    pub junctions_as_dirs: bool,
}

/// <https://docs.syncthing.net/advanced/folder-filesystem-type.html>
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum FilesystemType {
    Basic,
    Fake,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Type {
    /// The folder is in default mode. Sending local and accepting remote changes. Note that this
    /// type was previously called “readwrite” which is deprecated but still accepted in incoming
    /// configs.
    SendReceive,
    /// The folder is in “send only” mode – it will not be modified by Syncthing on this device.
    /// Note that this type was previously called “readonly” which is deprecated but still accepted
    /// in incoming configs.
    SendOnly,
    /// The folder is in “receive only” mode – it will not propagate changes to other devices.
    ReceiveOnly,
    /// Must be used on untrusted devices, where the data cannot be decrypted because no folder
    /// password was entered. See [Untrusted (Encrypted) Devices][1].
    ///
    /// [1]: <https://docs.syncthing.net/users/untrusted.html>
    ReceiveEncrypted,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Device {
    /// The [device ID][1].
    ///
    /// [1]: <https://docs.syncthing.net/dev/device-ids.html#device-ids>
    #[serde(rename = "deviceID")]
    pub id: String,
    /// Defines which device has introduced us to this device. Used only for following
    /// de-introductions.
    // TODO: Is this a Device ID?
    pub introduced_by: String,
    pub encryption_password: String,
}

// TODO: Where is the documentation for this?
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Versioning {
    #[serde(rename = "type")]
    pub ty: String,
    // TODO
    pub params: serde_json::Value,
    pub cleanup_interval_s: Seconds,
    pub fs_path: String,
    pub fs_type: FilesystemType,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum PullOrder {
    /// Pull files in random order. This optimizes for balancing resources among the devices in a
    /// cluster.
    Random,
    /// Pull files ordered by file name alphabetically.
    Alphabetic,
    /// Pull files ordered by file size; smallest and largest first respectively.
    SmallestFirst,
    LargestFirst,
    /// Pull files ordered by modification time; oldest and newest first respectively.
    OldestFirst,
    NewestFirst,
}

impl Default for PullOrder {
    fn default() -> Self {
        Self::Random
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum BlockPullOrder {
    /// The blocks of a file are split into N equal continuous sequences, where N is the number of
    /// connected devices. Each device starts downloading its own sequence, after which it picks
    /// other devices sequences at random. Provides acceptable data distribution and minimal
    /// spinning disk strain.
    Standard,
    /// The blocks of a file are downloaded in a random order. Provides great data distribution, but
    /// very taxing on spinning disk drives.
    Random,
    /// The blocks of a file are downloaded sequentially, from start to finish. Spinning disk drive
    /// friendly, but provides no improvements to data distribution.
    InOrder,
}

impl Default for BlockPullOrder {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
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
    }"#;
}
