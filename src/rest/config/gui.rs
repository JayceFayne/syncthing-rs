use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf};

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
    /// - IPv4 address and port (127.0.0.1:8384)  
    ///   The address and port are used as given.
    /// - IPv6 address and port ([::1]:8384)  
    ///   The address and port are used as given. The address must be enclosed in square brackets.
    /// - Wildcard and port (0.0.0.0:12345, [::]:12345, :12345)  
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
    UnixDomainSocket(PathBuf),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum AuthMode {
    Static,
    Ldap,
}
