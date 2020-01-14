use serde_derive::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SystemVersion {
    pub arch: String, //FIXME:use enum
    pub long_version: String,
    pub os: String, //FIXME:use enum
    pub version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Pong {
    Pong,
}

#[derive(Debug, Deserialize)]
pub struct SystemPing {
    pub ping: Pong,
}

#[derive(Debug, Deserialize)]
pub struct SystemLog {
    pub messages: Vec<SystemLogEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SystemLogEntry {
    pub when: String,
    pub message: String,
}

//TODO: ip type for address, DeviceID/FolderID type with deser
//FIXME: check folder == folderLable inconsistency

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum EventType {
    ConfigSaved,
    DeviceConnected,
    DeviceDisconnected,
    DeviceDiscovered,
    DevicePaused,
    DeviceRejected,
    DeviceResumed,
    DownloadProgress,
    FolderCompletion,
    FolderErrors,
    FolderRejected,
    FolderScanProgress,
    FolderSummary,
    ItemFinished,
    ItemStarted,
    ListenAddressesChanged,
    LocalChangeDetected,
    LocalIndexUpdated,
    LoginAttempt,
    RemoteChangeDetected,
    RemoteDownloadProgress,
    RemoteIndexUpdated,
    Starting,
    StartupComplete,
    StateChanged,
}

type FileName = String;
type FolderName = String;
type Folder = HashMap<FileName, File>;

#[derive(Debug, Deserialize)]
pub enum EventData {
    ConfigSaved(ConfigSavedEvent),
    DeviceConnected(DeviceConnectedEvent),
    DeviceDisconnected(DeviceDisconnectedEvent),
    DeviceDiscovered(DeviceDiscoveredEvent),
    DevicePaused(DevicePausedEvent),
    DeviceRejected(DeviceRejectedEvent),
    DeviceResumed(DeviceResumedEvent),
    DownloadProgress(HashMap<FolderName, Folder>),
    FolderCompletion(FolderCompletionEvent),
    FolderErrors(FolderErrorsEvent),
    FolderRejected(FolderRejectedEvent),
    FolderScanProgress(FolderScanProgressEvent),
    FolderSummary(FolderSummaryEvent),
    ItemFinished(ItemFinishedEvent),
    ItemStarted(ItemStartedEvent),
    ListenAddressesChanged(ListenAddressesChangedEvent),
    LocalChangeDetected(LocalChangeDetectedEvent),
    LocalIndexUpdated(LocalIndexUpdatedEvent),
    LoginAttempt(LoginAttemptEvent),
    RemoteChangeDetected(RemoteChangeDetectedEvent),
    RemoteDownloadProgress(RemoteDownloadProgressEvent),
    RemoteIndexUpdated(RemoteIndexUpdatedEvent),
    Starting(StartingEvent),
    StartupComplete,
    StateChanged(StateChangedEvent),
}

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

//FIXME: complete
#[derive(Debug, Deserialize)]
pub struct ConfigSavedEvent {
    #[serde(rename = "Version")]
    pub version: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DeviceConnectedEvent {
    pub addr: String,
    #[serde(rename = "id")]
    pub device_id: String,
    pub device_name: String,
    pub client_name: String,
    pub client_version: String,
    #[serde(rename = "type")]
    pub client_type: String, //FIXME: use enum
}

#[derive(Debug, Deserialize)]
pub struct DeviceDisconnectedEvent {
    #[serde(rename = "id")]
    pub device_id: String,
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceDiscoveredEvent {
    #[serde(rename = "device")]
    pub device_id: String,
    pub addrs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DevicePausedEvent {
    #[serde(rename = "device")]
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceRejectedEvent {
    #[serde(rename = "device")]
    device_id: String,
    pub name: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceResumedEvent {
    #[serde(rename = "device")]
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct FolderCompletionEvent {
    #[serde(rename = "device")]
    pub device_id: String,
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub completion: f64,
    pub global_bytes: u64,
    pub need_bytes: u64,
    pub need_deletes: u64,
    pub need_items: u64,
}

#[derive(Debug, Deserialize)]
pub struct FolderErrorsEvent {
    pub folder: String,
    pub errors: Vec<FolderError>,
}

#[derive(Debug, Deserialize)]
pub struct FolderError {
    pub error: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct FolderRejectedEvent {
    #[serde(rename = "device")]
    pub device_id: String,
    #[serde(rename = "folder")]
    pub folder_id: String,
    #[serde(rename = "folderLabel")]
    pub folder_label: String,
}

#[derive(Debug, Deserialize)]
pub struct FolderScanProgressEvent {
    pub total: u64,
    pub rate: u64,
    pub current: u64,
    #[serde(rename = "folder")]
    pub folder_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FolderSummaryEvent {
    pub folder: String,
    pub summary: FolderSummaryData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct FolderSummaryData {
    pub global_bytes: u64,
    pub global_deleted: u64,
    pub global_directories: u64,
    pub global_files: u64,
    pub global_symlinks: u64,
    pub global_total_items: u64,
    pub ignore_patterns: bool,
    pub in_sync_bytes: u64,
    pub in_sync_files: u64,
    pub invalid: Option<String>,
    pub local_bytes: u64,
    pub local_deleted: u64,
    pub local_directories: u64,
    pub local_files: u64,
    pub local_symlinks: u64,
    pub local_total_items: u64,
    pub need_bytes: u64,
    pub need_deletes: u64,
    pub need_directories: u64,
    pub need_files: u64,
    pub need_symlinks: u64,
    pub need_total_items: u64,
    pub pull_errors: u64,
    pub sequence: u64,
    pub state: String,
    pub state_changed: String, //FIXME: use enum
    pub version: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum ItemAction {
    Update,
    Metadata,
    Delete,
}

#[derive(Debug, Deserialize)]
pub struct ItemFinishedEvent {
    pub item: String,
    pub folder: String,
    pub error: Option<String>,
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
    pub action: ItemAction,
}

#[derive(Debug, Deserialize)]
pub struct ItemStartedEvent {
    pub item: String,
    pub folder: String,
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
    pub action: ItemAction,
}

#[derive(Debug, Deserialize)]
pub struct ListenAddressesChangedEvent {}

#[derive(Debug, Deserialize)]
pub struct LocalChangeDetectedEvent {
    pub action: String, //FIXME: use enum
    #[serde(rename = "folderID")]
    pub folder_id: String,
    pub label: String,
    path: String,
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
}

#[derive(Debug, Deserialize)]
pub struct LocalIndexUpdatedEvent {
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub items: u64,
    pub version: u64,
    pub filenames: Vec<FileName>,
}

#[derive(Debug, Deserialize)]
pub struct LoginAttemptEvent {
    pub username: String,
    pub success: bool,
}
#[derive(Debug, Deserialize)]
pub struct RemoteChangeDetectedEvent {
    pub action: String,
    #[serde(rename = "folderID")]
    pub folder_id: String,
    pub label: String,
    pub path: String,
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
    #[serde(rename = "modifiedBy")]
    pub modified_by: String,
}

#[derive(Debug, Deserialize)]
pub struct RemoteDownloadProgressEvent {
    #[serde(rename = "device")]
    pub device_id: String,
    pub folder: String,
    pub state: HashMap<FileName, u64>,
}

#[derive(Debug, Deserialize)]
pub struct RemoteIndexUpdatedEvent {
    #[serde(rename = "device")]
    pub device_id: String,
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub items: u64,
    pub version: u64,
}

#[derive(Debug, Deserialize)]
pub struct StartingEvent {
    #[serde(rename = "myID")]
    pub device_id: String,
    pub home: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum FolderState {
    Idle,
    Scanning,
    ScanWaiting,
    SyncPreparing,
    Syncing,
    Error,
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct StateChangedEvent {
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub duration: Option<f64>,
    pub from: FolderState,
    pub to: FolderState,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawEvent {
    pub id: u64,
    #[serde(rename = "globalID")]
    pub global_id: u64,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub time: String,
    pub data: Box<RawValue>,
}

#[derive(Debug)]
pub struct Event {
    pub id: u64,
    pub global_id: u64,
    pub time: String,
    pub data: EventData,
}
