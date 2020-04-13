use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum DebugOption {
    Beacon,
    API,
    App,
    Backend,
    Config,
    Connections,
    DB,
    Dialer,
    Discover,
    Events,
    FS,
    Main,
    Model,
    NAT,
    PMP,
    Protocol,
    Relay,
    Scanner,
    SHA256,
    Stats,
    STUN,
    Sync,
    Upgrade,
    UPnP,
    Ur,
    Versioner,
    WalkFS,
    Watchaggregator,
}

pub type Description = String;

#[derive(Debug, Deserialize)]
pub struct DebugInfo {
    pub enabled: Vec<DebugOption>,
    pub facilities: HashMap<DebugOption, Description>,
}
