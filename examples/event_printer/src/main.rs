use futures_util::stream::StreamExt;
use http_client::native::NativeClient;
use syncthing::{Connection, Fallible};

#[async_std::main]
async fn main() -> Fallible<()> {
    let connection = Connection::new(NativeClient::new(), include_str!("../../../api.key"))?;
    let mut stream = connection.subscribe(None);
    while let Some(event) = stream.next().await {
        println!("{:?}", event?);
    }
    Ok(())
}
