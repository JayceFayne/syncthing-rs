use super::PortNumber;
use crate::utils::impl_from_str_and_display;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Write},
    net::SocketAddr,
    num::ParseIntError,
    path::PathBuf,
    str::FromStr,
};

/// The GUI configuration.
/// <https://docs.syncthing.net/users/config.html#gui-element>
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Gui {
    /// If not `true`, the GUI and API will not be started.
    pub enabled: bool,
    /// If set to `true`, TLS (HTTPS) will be enforced. Non-HTTPS requests will be redirected to
    /// HTTPS. When set to `false`, TLS connections are still possible but not required.
    #[serde(rename = "useTLS")]
    pub tls: bool,
    /// This enables Profiling and additional endpoints in the REST API, see Debug Endpoints.
    pub debugging: bool,
    /// The listen address. Allowed address formats are:
    /// - IPv4 address and port (`127.0.0.1:8384`)  
    ///   The address and port are used as given.
    /// - IPv6 address and port (`[::1]:8384`)  
    ///   The address and port are used as given. The address must be enclosed in square brackets.
    /// - Wildcard and port (`0.0.0.0:12345`, `[::]:12345`, `:12345`)  
    ///   These are equivalent and will result in Syncthing listening on all interfaces via both
    ///   IPv4 and IPv6.
    /// - UNIX socket location (/var/run/st.sock)  
    ///   If the address is an absolute path it is interpreted as the path to a UNIX socket.
    pub address: Address,
    /// When `address` is set to a UNIX socket location, set this to an octal value to override the
    /// default permissions of the socket.
    // TODO: Octal
    pub unix_socket_permissions: String,
    /// Set to require authentication.
    pub user: String,
    /// Contains the bcrypt hash of the real password.
    pub password: String,
    /// If set, this is the API key that enables usage of the REST interface.
    pub api_key: String,
    /// If `true`, this allows access to the web GUI from outside (i.e. not localhost) without
    /// authorization. A warning will displayed about this setting on startup.
    pub insecure_admin_access: bool,
    /// When the GUI / API is bound to localhost, we enforce that the `Host` header looks like
    /// localhost. This option bypasses that check.
    pub insecure_skip_hostcheck: bool,
    /// Allow rendering the GUI within an `<iframe>`, `<frame>` or `<object>` by not setting the
    /// `X-Frame-Options: SAMEORIGIN` HTTP header. This may be needed for serving the Syncthing GUI
    /// as part of a website through a proxy.
    pub insecure_allow_frame_loading: bool,
    /// The name of the theme to use.
    pub theme: String,
    /// Authentication mode to use. If not present, the authentication mode (`static`) is controlled
    /// by the presence of `user`/`password` fields for backward compatibility.
    pub auth_mode: AuthMode,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Address {
    Ip(SocketAddr),
    #[serde(with = "serde_with::rust::display_fromstr")]
    Port(WildcardPort),
    Path(PathBuf),
}

impl_from_str_and_display!(Address);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct WildcardPort(PortNumber);

impl WildcardPort {
    pub fn new(port: PortNumber) -> Self {
        Self(port)
    }

    pub fn port(self) -> PortNumber {
        self.0
    }
}

impl From<PortNumber> for WildcardPort {
    fn from(port: PortNumber) -> Self {
        Self::new(port)
    }
}

impl From<WildcardPort> for PortNumber {
    fn from(port: WildcardPort) -> Self {
        port.port()
    }
}

impl FromStr for WildcardPort {
    type Err = ParseWildcardPortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(port) = s.strip_prefix(':') {
            port.parse::<PortNumber>()
                .map(WildcardPort::new)
                .map_err(ParseWildcardPortError::IntError)
        } else {
            Err(ParseWildcardPortError::MissingPrefix)
        }
    }
}

impl Display for WildcardPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(':')?;
        self.port().fmt(f)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ParseWildcardPortError {
    MissingPrefix,
    IntError(ParseIntError),
}

impl Display for ParseWildcardPortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseWildcardPortError::*;
        match self {
            MissingPrefix => f.write_str("missing ':' prefix"),
            IntError(int_error) => int_error.fmt(f),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum AuthMode {
    Static,
    Ldap,
}

impl_from_str_and_display!(AuthMode);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::config::test_helper::{deserialize, serialize};
    use rstest::rstest;
    use serde_json::Value;

    #[rstest]
    #[case::ipv4("127.0.0.1:8384")]
    #[case::ipv6("[::1]:8384")]
    #[case::wildcard4("0.0.0.0:12345")]
    #[case::wildcard6("[::]:12345")]
    #[case::wildcard_port(":12345")]
    #[case::unix_socket("/var/run/st.sock")]
    fn types_of_addresses(#[case] address: &str) {
        let serialized_str = serialize(Value::String(address.to_owned()));
        let deserialized_address = deserialize::<Address>(&serialized_str);
        println!("{:?}", deserialized_address);
        let serialized_address = serialize(deserialized_address);
        assert_eq!(serialized_address, serialized_str)
    }

    #[test]
    fn wildcard_port() {
        let wildcard = ":12345";
        let serialized_str = serialize(Value::String(wildcard.to_owned()));
        let deserialized_address = deserialize::<Address>(&serialized_str);
        assert_eq!(
            deserialized_address,
            Address::Port(WildcardPort::new(12345))
        )
    }

    #[test]
    fn path_serializes_as_address_path() {
        let path = "/var/run/st.sock";
        let serialized_str = serialize(Value::String(path.into()));
        let deserialized_address = deserialize::<Address>(&serialized_str);
        assert_eq!(deserialized_address, Address::Path(path.into()))
    }
}
