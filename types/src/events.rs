use crate::{DeviceID, FolderID, Timestamp};
use crate::{FileName, Folder, FolderName};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::collections::HashMap;

//FIXME: complete
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ConfigSavedEvent {
    pub version: u64,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DeviceConnectedEvent {
    pub addr: String,
    #[serde(rename = "id")]
    pub device_id: DeviceID,
    pub device_name: String,
    pub client_name: String,
    pub client_version: String,
    #[serde(rename = "type")]
    pub client_type: String, //FIXME: use enum
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DeviceDisconnectedEvent {
    #[serde(rename = "id")]
    pub device_id: DeviceID,
    pub error: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DeviceDiscoveredEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    #[serde(default)]
    pub addrs: Vec<String>,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DevicePausedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DeviceRejectedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    pub name: String,
    pub address: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DeviceResumedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ClusterConfigReceivedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct FolderCompletionEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    #[serde(rename = "folder")]
    pub folder_id: FolderID,
    pub completion: f64,
    pub global_bytes: u64,
    pub need_bytes: u64,
    pub need_deletes: u64,
    pub need_items: u64,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FolderErrorsEvent {
    pub folder: String,
    #[serde(default)]
    pub errors: Vec<FolderError>,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FolderError {
    pub error: String,
    pub path: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FolderRejectedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    #[serde(rename = "folder")]
    pub folder_id: String,
    #[serde(rename = "folderLabel")]
    pub folder_label: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FolderScanProgressEvent {
    pub total: u64,
    pub rate: u64,
    pub current: u64,
    #[serde(rename = "folder")]
    pub folder_id: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FolderSummaryEvent {
    pub folder: String,
    pub summary: FolderSummaryData,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum ItemAction {
    Update,
    Metadata,
    Delete,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ItemFinishedEvent {
    pub item: String,
    pub folder: String,
    pub error: Option<String>,
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
    pub action: ItemAction,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ItemStartedEvent {
    pub item: String,
    pub folder: String,
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
    pub action: ItemAction,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListenAddressesChangedEvent {}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LocalChangeDetectedEvent {
    pub action: String, //FIXME: use enum
    #[serde(rename = "folderID")]
    pub folder_id: String,
    pub label: String,
    pub path: String,
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LocalIndexUpdatedEvent {
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub items: u64,
    pub version: u64,
    #[serde(default)]
    pub filenames: Vec<FileName>,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LoginAttemptEvent {
    pub username: String,
    pub success: bool,
}
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RemoteDownloadProgressEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    pub folder: String,
    pub state: HashMap<FileName, u64>,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RemoteIndexUpdatedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub items: u64,
    pub version: u64,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StartingEvent {
    #[serde(rename = "myID")]
    pub device_id: DeviceID,
    pub home: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum FolderState {
    Idle,
    Scanning,
    ScanWaiting,
    SyncWaiting,
    SyncPreparing,
    Syncing,
    Cleaning,
    CleanWaiting,
    Error,
    Unknown,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StateChangedEvent {
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub duration: Option<f64>,
    pub from: FolderState,
    pub to: FolderState,
    pub error: Option<String>,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum EventData {
    ConfigSaved(ConfigSavedEvent),
    DeviceConnected(DeviceConnectedEvent),
    DeviceDisconnected(DeviceDisconnectedEvent),
    DeviceDiscovered(DeviceDiscoveredEvent),
    DevicePaused(DevicePausedEvent),
    DeviceRejected(DeviceRejectedEvent),
    DeviceResumed(DeviceResumedEvent),
    ClusterConfigReceived(ClusterConfigReceivedEvent),
    DownloadProgress(HashMap<FolderName, Folder>),
    FolderCompletion(FolderCompletionEvent),
    FolderErrors(FolderErrorsEvent),
    FolderRejected(FolderRejectedEvent),
    FolderScanProgress(FolderScanProgressEvent),
    FolderSummary(Box<FolderSummaryEvent>),
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
pub(super) struct RawEvent {
    pub id: u64,
    #[serde(rename = "globalID")]
    pub global_id: u64,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub time: Timestamp,
    pub data: Box<RawValue>,
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub enum EventType {
    ConfigSaved,
    DeviceConnected,
    DeviceDisconnected,
    DeviceDiscovered,
    DevicePaused,
    DeviceRejected,
    DeviceResumed,
    ClusterConfigReceived,
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

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "RawEvent")]
pub struct Event {
    pub id: u64,
    pub global_id: u64,
    pub time: Timestamp,
    pub data: EventData,
}

impl core::convert::TryFrom<RawEvent> for Event {
    type Error = serde_json::Error;

    fn try_from(raw_event: RawEvent) -> Result<Self, Self::Error> {
        use EventData::*;
        let RawEvent {
            id,
            global_id,
            event_type,
            time,
            data,
        } = raw_event;
        let data = data.get();
        Ok(Event {
            id,
            global_id,
            time,
            data: match event_type {
                EventType::ConfigSaved => ConfigSaved(serde_json::from_str(data)?),
                EventType::DeviceConnected => DeviceConnected(serde_json::from_str(data)?),
                EventType::DeviceDisconnected => DeviceDisconnected(serde_json::from_str(data)?),
                EventType::DeviceDiscovered => DeviceDiscovered(serde_json::from_str(data)?),
                EventType::DevicePaused => DevicePaused(serde_json::from_str(data)?),
                EventType::DeviceRejected => DeviceRejected(serde_json::from_str(data)?),
                EventType::DeviceResumed => DeviceResumed(serde_json::from_str(data)?),
                EventType::DownloadProgress => DownloadProgress(serde_json::from_str(data)?),
                EventType::FolderCompletion => FolderCompletion(serde_json::from_str(data)?),
                EventType::FolderErrors => FolderErrors(serde_json::from_str(data)?),
                EventType::FolderRejected => FolderRejected(serde_json::from_str(data)?),
                EventType::FolderScanProgress => FolderScanProgress(serde_json::from_str(data)?),
                EventType::FolderSummary => FolderSummary(serde_json::from_str(data)?),
                EventType::ItemFinished => ItemFinished(serde_json::from_str(data)?),
                EventType::ItemStarted => ItemStarted(serde_json::from_str(data)?),
                EventType::ListenAddressesChanged => {
                    ListenAddressesChanged(serde_json::from_str(data)?)
                }
                EventType::LocalChangeDetected => LocalChangeDetected(serde_json::from_str(data)?),
                EventType::LocalIndexUpdated => LocalIndexUpdated(serde_json::from_str(data)?),
                EventType::LoginAttempt => LoginAttempt(serde_json::from_str(data)?),
                EventType::RemoteChangeDetected => {
                    RemoteChangeDetected(serde_json::from_str(data)?)
                }
                EventType::RemoteDownloadProgress => {
                    RemoteDownloadProgress(serde_json::from_str(data)?)
                }
                EventType::RemoteIndexUpdated => RemoteIndexUpdated(serde_json::from_str(data)?),
                EventType::Starting => Starting(serde_json::from_str(data)?),
                EventType::StartupComplete => StartupComplete,
                EventType::StateChanged => StateChanged(serde_json::from_str(data)?),
                EventType::ClusterConfigReceived => {
                    ClusterConfigReceived(serde_json::from_str(data)?)
                }
            },
        })
    }
}
