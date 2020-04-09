use crate::{Event, EventData, EventType, RawEvent};
use serde_json::{from_str, Error};
use std::convert::TryFrom;

impl TryFrom<RawEvent> for Event {
    type Error = Error;

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
                EventType::ConfigSaved => ConfigSaved(from_str(data)?),
                EventType::DeviceConnected => DeviceConnected(from_str(data)?),
                EventType::DeviceDisconnected => DeviceDisconnected(from_str(data)?),
                EventType::DeviceDiscovered => DeviceDiscovered(from_str(data)?),
                EventType::DevicePaused => DevicePaused(from_str(data)?),
                EventType::DeviceRejected => DeviceRejected(from_str(data)?),
                EventType::DeviceResumed => DeviceResumed(from_str(data)?),
                EventType::DownloadProgress => DownloadProgress(from_str(data)?),
                EventType::FolderCompletion => FolderCompletion(from_str(data)?),
                EventType::FolderErrors => FolderErrors(from_str(data)?),
                EventType::FolderRejected => FolderRejected(from_str(data)?),
                EventType::FolderScanProgress => FolderScanProgress(from_str(data)?),
                EventType::FolderSummary => FolderSummary(from_str(data)?),
                EventType::ItemFinished => ItemFinished(from_str(data)?),
                EventType::ItemStarted => ItemStarted(from_str(data)?),
                EventType::ListenAddressesChanged => ListenAddressesChanged(from_str(data)?),
                EventType::LocalChangeDetected => LocalChangeDetected(from_str(data)?),
                EventType::LocalIndexUpdated => LocalIndexUpdated(from_str(data)?),
                EventType::LoginAttempt => LoginAttempt(from_str(data)?),
                EventType::RemoteChangeDetected => RemoteChangeDetected(from_str(data)?),
                EventType::RemoteDownloadProgress => RemoteDownloadProgress(from_str(data)?),
                EventType::RemoteIndexUpdated => RemoteIndexUpdated(from_str(data)?),
                EventType::Starting => Starting(from_str(data)?),
                EventType::StartupComplete => StartupComplete,
                EventType::StateChanged => StateChanged(from_str(data)?),
            },
        })
    }
}
