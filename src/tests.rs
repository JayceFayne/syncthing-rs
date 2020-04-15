use crate::{Client, Fallible};
use futures_util::stream::StreamExt;

static API_KEY: &str = include_str!("../api.key");

#[tokio::test]
async fn get_system_connections() -> Fallible<()> {
    let client = Client::new(API_KEY);
    client.get_system_connections().await?;
    Ok(())
}

#[tokio::test]
async fn get_system_debug() -> Fallible<()> {
    let client = Client::new(API_KEY);
    client.get_system_debug().await?;
    Ok(())
}

#[tokio::test]
async fn get_system_discovery() -> Fallible<()> {
    let client = Client::new(API_KEY);
    client.get_system_discovery().await?;
    Ok(())
}

#[tokio::test]
async fn get_system_log() -> Fallible<()> {
    let client = Client::new(API_KEY);
    client.get_system_log().await?;
    Ok(())
}

#[tokio::test]
async fn get_system_ping() -> Fallible<()> {
    let client = Client::new(API_KEY);
    client.get_system_ping().await?;
    Ok(())
}

#[tokio::test]
async fn get_system_error() -> Fallible<()> {
    let client = Client::new(API_KEY);
    client.get_system_error().await?;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn get_system_upgrade() -> Fallible<()> {
    let client = Client::new(API_KEY);
    client.get_system_upgrade().await?;
    Ok(())
}

#[tokio::test]
async fn get_system_version() -> Fallible<()> {
    let client = Client::new(API_KEY);
    client.get_system_version().await?;
    Ok(())
}

#[tokio::test]
async fn get_events() -> Fallible<()> {
    let client = Client::new(API_KEY);
    client.get_all_events(None, None).await?;
    Ok(())
}

#[tokio::test]
async fn event_stream() -> Fallible<()> {
    let client = Client::new(API_KEY);
    let mut stream = client.subscribe_to_all();
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
