use crate::DeviceID;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub errors: u32,
    pub pull_errors: u32,
    pub invalid: String,

    pub global_files: u64,
    pub global_directories: u64,
    pub global_symlinks: u64,
    pub global_deleted: u64,
    pub global_bytes: u64,
    pub global_total_items: u64,

    pub local_files: u64,
    pub local_directories: u64,
    pub local_symlinks: u64,
    pub local_deleted: u64,
    pub local_bytes: u64,
    pub local_total_items: u64,

    pub need_files: u64,
    pub need_directories: u64,
    pub need_symlinks: u64,
    pub need_deletes: u64,
    pub need_bytes: u64,
    pub need_total_items: u64,

    pub receive_only_changed_files: u64,
    pub receive_only_changed_directories: u64,
    pub receive_only_changed_symlinks: u64,
    pub receive_only_changed_deletes: u64,
    pub receive_only_changed_bytes: u64,
    pub receive_only_total_items: u64,

    pub in_sync_files: u64,
    pub in_sync_bytes: u64,

    pub state: FolderState,
    pub state_changed: String,

    pub error: String,

    pub version: u64,
    pub sequence: u64,

    pub remote_sequence: HashMap<DeviceID, u64>,

    pub ignore_patterns: bool,
    pub watch_error: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FolderState {
    Idle,
    Syncing,
    Scanning,
    Error,
    Unknown,
}
