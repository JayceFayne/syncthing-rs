use syncthing::{Authority, Client, Fallible};

fn main() -> Fallible<()> {
    let mut clients: Vec<Client> = Vec::new();

    clients.push(Client::with_authority(
        include_str!("../../../api.key"),
        Authority::from_static("localhost:8384"),
    ).unwrap());

    clients.push(Client::with_authority(
        include_str!("../../../api.key.deb"),
        Authority::from_static("debian-server:8384"),
    ).unwrap());

    for client in clients {
        println!("client address is {}!", client.authority);

        let _p = client.ping().unwrap();

        let system = client.get_version_info().unwrap();
        println!(
            "syncthing {} is running on {:?}!\n>>> ({})!",
            system.version, system.os, system.long_version
        );
        println!("");
    }
    Ok(())
}
