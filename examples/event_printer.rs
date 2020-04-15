use futures_util::stream::StreamExt;
use syncthing::{Client, Fallible};

#[tokio::main]
async fn main() -> Fallible<()> {
    let client = Client::new(include_str!("../api.key"));
    let mut stream = client.subscribe_to_all();
    while let Some(event) = stream.next().await {
        println!("{:?}", event?);
    }
    Ok(())
}
