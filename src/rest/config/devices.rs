use super::{FolderId, Kibibytes, KibibytesPerSecond, PortNumber};
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

/// <https://docs.syncthing.net/users/config.html#device-element>
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Device {
    #[serde(rename = "deviceID")]
    id: String,
    #[serde(flatten)]
    options: Options,
}

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Options {
    name: Option<String>,
    compression: Option<Compression>,
    introducer: Option<bool>,
    skip_introduction_removals: Option<bool>,
    introduced_by: Option<String>,
    cert_name: Option<String>,
    encryption_password: Option<String>,
    // TODO: At least one must be present.
    addresses: Option<Vec<Address>>,
    paused: Option<bool>,
    allowed_networks: Option<Vec<IpNet>>,
    auto_accept_folders: Option<bool>,
    max_send_kbps: Option<KibibytesPerSecond>,
    max_recv_kbps: Option<KibibytesPerSecond>,
    ignored_folders: Option<Vec<FolderId>>,
    #[serde(rename = "maxRequestKiB")]
    max_request_kib: Option<Kibibytes>,
    #[serde(rename = "remoteGUIPort")]
    remote_gui_port: Option<PortNumber>,
    untrusted: Option<bool>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum Compression {
    Metadata,
    Always,
    Never,
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
