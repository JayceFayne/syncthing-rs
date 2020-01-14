use crate::{Connection, Event, EventData, EventType, Fallible, RawEvent};
use futures_core::ready;
use futures_core::stream::Stream;
use http_client::HttpClient;
use serde_json::{from_str, Error};
use std::cell::UnsafeCell;
use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

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

//TODO:self correction mechanism see: https://docs.syncthing.net/rest/events-get.html#events-get
pub struct EventStream<C> {
    connection: UnsafeCell<Connection<C>>,
    events: Option<UnsafeCell<Vec<EventType>>>,
    state: State,
    since: Option<u64>,
}

enum State {
    Init,
    Buffer(Vec<Event>),
    Future(Pin<Box<dyn Future<Output = Fallible<Vec<Event>>>>>),
}

impl<C> EventStream<C> {
    pub(crate) fn new(connection: Connection<C>, events: Option<Vec<EventType>>) -> Self {
        Self {
            connection: UnsafeCell::new(connection),
            events: events.map(UnsafeCell::new),
            state: State::Init,
            since: None,
        }
    }
}

impl<C: HttpClient> EventStream<C> {
    fn queue_next_state(&mut self, limit: Option<u64>) {
        let connection = unsafe { &*self.connection.get() };
        let events = self.events.as_ref().map(|e| &unsafe { &*e.get() }[..]);
        self.state = State::Future(Box::pin(connection.get_events(events, self.since, limit)));
    }
}

impl<C: HttpClient> Stream for EventStream<C> {
    type Item = Fallible<Event>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut self.state {
            State::Future(fut) => match ready!(fut.as_mut().poll(cx)) {
                Ok(mut data) => {
                    data.reverse();
                    if let Some(event) = data.pop() {
                        self.since = Some(event.id);
                        self.state = State::Buffer(data);
                        Poll::Ready(Some(Ok(event)))
                    } else {
                        self.queue_next_state(None);
                        Poll::Pending
                    }
                }
                Err(err) => {
                    self.queue_next_state(None);
                    Poll::Ready(Some(Err(err)))
                }
            },
            State::Buffer(data) => {
                if let Some(event) = data.pop() {
                    self.since = Some(event.id);
                    Poll::Ready(Some(Ok(event)))
                } else {
                    self.queue_next_state(None);
                    Poll::Pending
                }
            }
            State::Init => {
                self.queue_next_state(Some(1));
                Poll::Pending
            }
        }
    }
}
