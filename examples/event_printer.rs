use futures_util::stream::StreamExt;
use syncthing::{Connection, Fallible};

#[tokio::main]
async fn main() -> Fallible<()> {
    let connection = Connection::new(include_str!("../api.key"));
    let mut stream = connection.subscribe_to_all();
    while let Some(event) = stream.next().await {
        println!("{:?}", event?);
    }
    Ok(())
}
