use crate::*;
use async_std::task::block_on;
use futures_util::stream::StreamExt;
use http_client::native::NativeClient;

static KEY: &str = include_str!("../api.key");

#[test]
fn connect() -> Result<(), Error> {
    Connection::new(NativeClient::new(), KEY)?;
    Ok(())
}
#[test]
fn run_get_system_log() -> Result<(), Error> {
    let connection = Connection::new(NativeClient::new(), KEY)?;
    block_on(connection.get_system_log())?;
    Ok(())
}

#[test]
fn run_get_system_ping() -> Result<(), Error> {
    let connection = Connection::new(NativeClient::new(), KEY)?;
    block_on(connection.get_system_ping())?;
    Ok(())
}

#[test]
fn run_get_system_version() -> Result<(), Error> {
    let connection = Connection::new(NativeClient::new(), KEY)?;
    block_on(connection.get_system_version())?;
    Ok(())
}

#[test]
fn run_get_events() -> Result<(), Error> {
    let connection = Connection::new(NativeClient::new(), KEY)?;
    block_on(connection.get_events(None, None, None))?;
    Ok(())
}

#[test]
fn event_stream() -> Result<(), Error> {
    let connection = Connection::new(NativeClient::new(), KEY)?;
    let mut stream = connection.subscribe(None);
    block_on(async {
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
    })
}
