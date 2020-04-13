use crate::rest::events::{Event, EventType};
use crate::{Connection, Fallible};
use futures_core::future::BoxFuture;
use futures_core::ready;
use futures_core::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

async fn receive(
    connection: Connection,
    since: Option<u64>,
    limit: Option<u64>,
    events: Vec<EventType>,
) -> (Connection, Vec<EventType>, Fallible<Vec<Event>>) {
    let data = connection.get_events(since, limit, &events).await;
    (connection, events, data)
}

#[allow(clippy::large_enum_variant)]
enum State {
    Buffer(Option<(Connection, Vec<EventType>)>, Vec<Event>),
    Future(BoxFuture<'static, (Connection, Vec<EventType>, Fallible<Vec<Event>>)>),
}

//TODO:self correction mechanism see: https://docs.syncthing.net/rest/events-get.html#events-get
pub struct EventStream {
    state: State,
    since: Option<u64>,
}

impl EventStream {
    pub(crate) fn new(connection: Connection, events: Vec<EventType>) -> Self {
        Self {
            state: State::Future(Box::pin(receive(connection, None, None, events))),
            since: None,
        }
    }
}

impl Stream for EventStream {
    type Item = Fallible<Event>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut self.state {
            State::Future(fut) => match ready!(fut.as_mut().poll(cx)) {
                (connection, events, Ok(mut data)) => {
                    data.reverse();
                    self.state = State::Buffer(Some((connection, events)), data);
                    Poll::Pending
                }
                (connection, events, Err(err)) => {
                    self.state =
                        State::Future(Box::pin(receive(connection, self.since, None, events)));
                    Poll::Ready(Some(Err(err)))
                }
            },
            State::Buffer(connection_events, data) => {
                if let Some(event) = data.pop() {
                    self.since = Some(event.id);
                    Poll::Ready(Some(Ok(event)))
                } else {
                    let (connection, events) = connection_events.take().unwrap();
                    self.state =
                        State::Future(Box::pin(receive(connection, self.since, None, events)));
                    Poll::Pending
                }
            }
        }
    }
}
