use crate::{Connection, Fallible};
use futures_util::stream::StreamExt;

static API_KEY: &str = include_str!("../api.key");

#[tokio::test]
async fn run_get_system_log() -> Fallible<()> {
    let connection = Connection::new(API_KEY);
    connection.get_system_log().await?;
    Ok(())
}

#[tokio::test]
async fn run_get_system_ping() -> Fallible<()> {
    let connection = Connection::new(API_KEY);
    connection.get_system_ping().await?;
    Ok(())
}

#[tokio::test]
async fn run_get_system_version() -> Fallible<()> {
    let connection = Connection::new(API_KEY);
    connection.get_system_version().await?;
    Ok(())
}

#[tokio::test]
async fn run_get_events() -> Fallible<()> {
    let connection = Connection::new(API_KEY);
    connection.get_all_events(None, None).await?;
    Ok(())
}

#[tokio::test]
async fn event_stream() -> Fallible<()> {
    let connection = Connection::new(API_KEY);
    let mut stream = connection.subscribe_to_all();
    let mut last = 0;
    let mut i = 0;
    while let Some(event) = stream.next().await {
        if i > 3 {
            return Ok(());
        }
        let event = event?;
        if last == 0 {
            last = event.id;
        } else {
            i += 1;
            assert_eq!(last + 1, event.id);
            last = event.id;
        }
    }
    Ok(())
}
