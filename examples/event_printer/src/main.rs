use syncthing::{Connection, Fallible};
use async_std::task::block_on;
use http_client::native::NativeClient;
use futures_util::stream::StreamExt;

fn main() -> Fallible<()> {
    let connection = Connection::new(NativeClient::new(), include_str!("../../../api.key"))?;
    let mut stream = connection.subscribe(None);
    block_on(async {
        while let Some(event) = stream.next().await {
         println!("{:?}", event?);
     }
     Ok(())
    })
}
