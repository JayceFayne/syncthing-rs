use crate::{Connection, Fallible};
use futures_util::stream::StreamExt;
use http_client::native::NativeClient;

static KEY: &str = include_str!("../api.key");

#[test]
fn connect() -> Fallible<()> {
    Connection::new(NativeClient::new(), KEY)?;
    Ok(())
}

#[async_std::test]
async fn run_get_system_log() -> Fallible<()> {
    let connection = Connection::new(NativeClient::new(), KEY)?;
    connection.get_system_log().await?;
    Ok(())
}

#[async_std::test]
async fn run_get_system_ping() -> Fallible<()> {
    let connection = Connection::new(NativeClient::new(), KEY)?;
    connection.get_system_ping().await?;
    Ok(())
}

#[async_std::test]
async fn run_get_system_version() -> Fallible<()> {
    let connection = Connection::new(NativeClient::new(), KEY)?;
    connection.get_system_version().await?;
    Ok(())
}

#[async_std::test]
async fn run_get_events() -> Fallible<()> {
    let connection = Connection::new(NativeClient::new(), KEY)?;
    connection.get_events(None, None, None).await?;
    Ok(())
}

#[async_std::test]
async fn event_stream() -> Fallible<()> {
    let connection = Connection::new(NativeClient::new(), KEY)?;
    let mut stream = connection.subscribe(None);
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
