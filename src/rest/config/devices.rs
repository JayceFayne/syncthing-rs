use super::{FolderId, Kibibytes, KibibytesPerSecond, PortNumber};
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

/// A device participating in the cluster.
///
/// <https://docs.syncthing.net/users/config.html#device-element>
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Device {
    /// The [device ID][1].
    ///
    /// [1]: <https://docs.syncthing.net/dev/device-ids.html#device-ids>
    #[serde(rename = "deviceID")]
    id: String,
    #[serde(flatten)]
    options: Options,
}

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Options {
    /// A friendly name for the device.
    name: Option<String>,
    /// Whether to use protocol compression when sending messages to this device.
    compression: Option<Compression>,
    /// Set to `true` if this device should be trusted as an introducer, i.e. we should copy their
    /// list of devices per folder when connecting.
    ///
    /// See also <https://docs.syncthing.net/users/introducer.html>
    introducer: Option<bool>,
    /// Set to `true` if you wish to follow only introductions and not de-introductions. For
    /// example, if this is set, we would not remove a device that we were introduced to even if the
    /// original introducer is no longer listing the remote device as known.
    skip_introduction_removals: Option<bool>,
    /// Defines which device has introduced us to this device. Used only for following
    /// de-introductions.
    // TODO: Is this a Device ID?
    introduced_by: Option<String>,
    /// The device certificate’s common name, if it is not the default “syncthing”.
    cert_name: Option<String>,
    encryption_password: Option<String>,
    /// Contains an address or host name to use when attempting to connect to this device. Entries
    /// other than `dynamic` need a protocol specific prefix. For the TCP protocol the prefixes
    /// `tcp://` (dual-stack), `tcp4://` (IPv4 only) or `tcp6://` (IPv6 only) can be used. The
    /// prefixes for the QUIC protocol are analogous: `quic://`, `quic4://` and `quic6://` Note that
    /// IP addresses need not use IPv4 or IPv6 prefixes; these are optional.
    // TODO: At least one must be present.
    addresses: Option<Vec<Address>>,
    /// `true` if synchronization with this devices is (temporarily) suspended.
    paused: Option<bool>,
    /// If given, this restricts connections to this device to only this network. The mechanism is
    /// described in detail in a [separate chapter][1]).
    ///
    /// [1]: <https://docs.syncthing.net/advanced/device-allowednetworks.html>
    allowed_networks: Option<Vec<IpNet>>,
    /// If `true`, folders shared from this remote device are automatically added and synced locally
    /// under the default path. For the folder name, Syncthing tries to use the label from the
    /// remote device, and if the same label already exists, it then tries to use the folder’s ID.
    /// If that exists as well, the folder is just offered to accept manually. A local folder
    /// already added with the same ID will just be shared rather than created separately.
    auto_accept_folders: Option<bool>,
    /// Maximum send rate to use for this device. Unit is kibibytes/second, despite the config name
    /// looking like kilobits/second.
    max_send_kbps: Option<KibibytesPerSecond>,
    /// Maximum receive rate to use for this device. Unit is kibibytes/second, despite the config
    /// name looking like kilobits/second.
    max_recv_kbps: Option<KibibytesPerSecond>,
    /// Contains the ID of folders that should be ignored. These folders will always be skipped when
    /// advertised from the containing remote device, i.e. this will be logged, but there will be no
    /// dialog shown in the web GUI.
    ignored_folders: Option<Vec<FolderId>>,
    /// Maximum amount of data to have outstanding in requests towards this device. Unit is
    /// kibibytes.
    #[serde(rename = "maxRequestKiB")]
    max_request_kib: Option<Kibibytes>,
    /// If set to a positive integer, the GUI will display an HTTP link to the IP address which is
    /// currently used for synchronization. Only the TCP port is exchanged for the value specified
    /// here. Note that any port forwarding or firewall settings need to be done manually and the
    /// link will probably not work for link-local IPv6 addresses because of modern browser
    /// limitations.
    #[serde(rename = "remoteGUIPort")]
    remote_gui_port: Option<PortNumber>,
    /// This boolean value marks a particular device as untrusted, which disallows ever sharing any
    /// unencrypted data with it. Every folder shared with that device then needs an encryption
    /// password set, or must already be of the “receive encrypted” type locally. Refer to the
    /// detailed explanation under [Untrusted (Encrypted) Devices][1].
    ///
    /// [1]: <https://docs.syncthing.net/users/untrusted.html>
    untrusted: Option<bool>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum Compression {
    /// Compress metadata packets, such as index information. Metadata is usually very compression
    /// friendly so this is a good default.
    Metadata,
    /// Compress all packets, including file data. This is recommended if the folders contents are
    /// mainly compressible data such as documents or text files.
    Always,
    /// Disable all compression.
    Never,
}

impl Default for Compression {
    fn default() -> Self {
        Self::Metadata
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Address {
    #[serde(with = "strings::dynamic")]
    Dynamic,
    Static(Url),
}

mod strings {
    use crate::utils::named_unit_variant;
    named_unit_variant!(dynamic);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::config::test_helper::*;
    use rstest::rstest;
    use url::Url;

    #[test]
    fn address_dynamic_serializes_as_expected() {
        assert_eq!(serialize(Address::Dynamic), r#""dynamic""#);
        assert_eq!(deserialize::<Address>(r#""dynamic""#), Address::Dynamic)
    }

    #[rstest]
    #[case::tcp("tcp://192.0.2.42")]
    #[case::tcp_port("tcp://192.0.2.42:12345")]
    #[case::tcp_v6("tcp://[2001:db8::23:42]")]
    #[case::hostname("tcp://fileserver")]
    #[case::hostname_port("tcp://fileserver:12345")]
    #[case::tcp4("tcp4://fileserver")]
    #[case::tcp6("tcp6://fileserver")]
    #[case::quic("quic://fileserver")]
    #[case::quic4("quic4://fileserver")]
    #[case::quic6("quic6://fileserver")]
    fn address_roundtrip_testing(#[case] address_str: &str) {
        let address = Address::Static(Url::parse(address_str).unwrap());
        let serialized = serialize(&address);
        assert_eq!(serialized, serialize(address_str));
        let deserialized = deserialize::<Address>(&serialized);
        assert_eq!(address, deserialized)
    }

    #[rstest]
    #[case::simple("192.168.0.0/16")]
    #[case::simple_2("172.16.0.0/12")]
    #[case::v6("2001:db8::/32")]
    #[case::preserves_information("192.168.100.200/16")]
    #[case::disallow_all_v4("0.0.0.0/0")]
    #[case::disallow_all_v6("::/0")]
    fn allowed_network_roundtrip(#[case] network_str: &str) {
        let network = network_str.parse::<IpNet>().unwrap();
        let serialized = serialize(&network);
        assert_eq!(serialized, serialize(network_str));
        let deserialized = deserialize::<IpNet>(&serialized);
        assert_eq!(network, deserialized)
    }

    #[test]
    fn json_normalization_roundtrip() {
        normalization_roundtrip_test::<Device>(DEFAULT_DEVICE);
    }

    const DEFAULT_DEVICE: &str = r#"
    {
        "deviceID": "",
        "name": "",
        "addresses": [
          "dynamic"
        ],
        "compression": "metadata",
        "certName": "",
        "introducer": false,
        "skipIntroductionRemovals": false,
        "introducedBy": "",
        "paused": false,
        "allowedNetworks": [],
        "autoAcceptFolders": false,
        "maxSendKbps": 0,
        "maxRecvKbps": 0,
        "ignoredFolders": [],
        "maxRequestKiB": 0,
        "untrusted": false,
        "remoteGUIPort": 0
      }
    "#;
}
