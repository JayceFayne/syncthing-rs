use crate::utils::impl_from_str_and_display;
use serde::{Deserialize, Serialize};

/// LDAP configuration options. The mechanism is described in detail under LDAP Authentication.
/// <https://docs.syncthing.net/users/config.html#ldap-element>
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Ldap {
    /// LDAP server address (`server:port`).
    // TODO: Is there a type for `server:port` representations?
    pub address: String,
    /// BindDN for user authentication. Special `%s` variable should be used to pass username to
    /// LDAP.
    #[serde(rename = "bindDN")]
    pub bind_dn: String,
    #[serde(flatten)]
    pub options: Options,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Options {
    pub transport: Transport,
    /// Skip verification (`true` or `false`).
    pub insecure_skip_verify: bool,
    /// Base DN for user searches.
    #[serde(rename = "searchBaseDN")]
    pub search_base_dn: String,
    /// Search filter for user searches.
    pub search_filter: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Transport {
    /// Non secure connection.
    Plain,
    /// TLS secured connection.
    Tls,
    /// StartTLS connection mode.
    StartTls,
}

impl_from_str_and_display!(Transport);
