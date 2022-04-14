use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// LDAP configuration options. The mechanism is described in detail under LDAP Authentication.
/// <https://docs.syncthing.net/users/config.html#ldap-element>
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
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

#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Options {
    pub transport: Option<Transport>,
    /// Skip verification (`true` or `false`).
    pub insecure_skip_verify: Option<bool>,
    /// Base DN for user searches.
    #[serde(rename = "searchBaseDN")]
    pub search_base_dn: Option<String>,
    /// Search filter for user searches.
    pub search_filter: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Transport {
    /// Non secure connection.
    NonTls,
    /// TLS secured connection.
    Tls,
    /// StartTLS connection mode.
    StartTls,
}