use syncthing::{Connection, Fallible};

#[tokio::main]
async fn main() -> Fallible<()> {
    let connection = Connection::new(include_str!("../api.key"));
    let system = connection.get_system_version().await?;
    println!("syncthing {} is running on {}!", system.version, system.os);
    Ok(())
}
