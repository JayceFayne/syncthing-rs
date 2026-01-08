use syncthing_async::{Authority, Client, Fallible};

#[tokio::main]
async fn main() -> Fallible<()> {
    let mut clients: Vec<Client> = Vec::new();

    clients.push(Client::with_authority(
        include_str!("../../../api.key"),
        Authority::from_static("localhost:8384"),
    ));

    clients.push(Client::with_authority(
        include_str!("../../../api.key.deb"),
        Authority::from_static("debian-server:8384"),
    ));

    for client in clients {
        println!("client address is {}!", client.authority);

        let _p = client.ping().await?;

        let system = client.get_version_info().await?;
        println!(
            "syncthing {} is running on {:?}!\n>>> ({})!",
            system.version, system.os, system.long_version
        );
        println!("");
    }
    Ok(())
}
