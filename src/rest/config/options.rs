use super::{
    Count, Hours, Kibibytes, KibibytesPerSecond, MinDiskFree, Minutes, PortNumber, Seconds,
};
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

pub type UsageReportingVersion = u32;
pub type TrafficClass = u8;

/// The options element contains all other global configuration options.
/// <https://docs.syncthing.net/users/config.html#options-element>
#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Options {
    /// The listen address for incoming sync connections. See [`ListenAddress`] for the allowed
    /// syntax.
    listen_addresses: Option<Vec<ListenAddress>>,
    /// URIs to global announce (discovery) servers. Any number of global announce servers may be
    /// present.See [`AnnounceServer`] for the allowed syntax.
    global_announce_servers: Option<Vec<AnnounceServer>>,
    /// Whether to announce this device to the global announce (discovery) server, and also use it
    /// to look up other devices.
    global_announce_enabled: Option<bool>,
    /// Whether to send announcements to the local LAN, also use such announcements to find other
    /// devices.
    local_announce_enabled: Option<bool>,
    /// The port on which to listen and send IPv4 broadcast announcements to.
    local_announce_port: Option<PortNumber>,
    /// The group address and port to join and send IPv6 multicast announcements on.
    // TODO: Type for Address:Port
    #[serde(rename = "localAnnounceMCAddr")]
    local_announce_mc_addr: Option<String>,
    /// Outgoing data rate limit, in kibibytes per second.
    max_send_kbps: Option<KibibytesPerSecond>,
    /// Incoming data rate limits, in kibibytes per second.
    max_recv_kbps: Option<KibibytesPerSecond>,
    /// The number of seconds to wait between each attempt to connect to currently unconnected
    /// devices.
    reconnection_interval_s: Option<Seconds>,
    /// When `true`, relays will be connected to and potentially used for device to device
    /// connections.
    relays_enabled: Option<bool>,
    /// Sets the interval, in minutes, between relay reconnect attempts.
    relay_reconnect_interval_m: Option<Minutes>,
    /// Whether to attempt to start a browser to show the GUI when Syncthing starts.
    start_browser: Option<bool>,
    /// Whether to attempt to perform a UPnP and NAT-PMP port mapping for incoming sync connections.
    nat_enabled: Option<bool>,
    /// Request a lease for this many minutes; zero to request a permanent lease.
    nat_lease_minutes: Option<Minutes>,
    /// Attempt to renew the lease after this many minutes.
    nat_renewal_minutes: Option<Minutes>,
    /// When scanning for UPnP devices, wait this long for responses.
    nat_timeout_seconds: Option<Seconds>,
    /// Whether the user has accepted to submit anonymous usage data. The default, 0, mean the user
    /// has not made a choice, and Syncthing will ask at some point in the future. -1 means no, a
    /// number above zero means that that version of usage reporting has been accepted.
    // TODO: Type
    ur_accepted: Option<i32>,
    /// The highest usage reporting version that has already been shown in the web GUI.
    ur_seen: Option<UsageReportingVersion>,
    /// The unique ID sent together with the usage report. Generated when usage reporting is enabled.
    ur_unique_id: Option<String>,
    /// The URL to post usage report data to, when enabled.
    #[serde(rename = "urURL")]
    ur_url: Option<Url>,
    /// When `true`, the UR URL can be http instead of https, or have a self-signed certificate. The
    /// default is `false`.
    ur_post_insecurely: Option<bool>,
    /// The time to wait from startup for the first usage report to be sent. Allows the system to
    /// stabilize before reporting statistics.
    ur_initial_delay_s: Option<Seconds>,
    /// Whether to perform a restart of Syncthing when it is detected that we are waking from sleep
    /// mode (i.e. an unfolding laptop).
    restart_on_wakeup: Option<bool>,
    /// Check for a newer version after this many hours. Set to `0` to disable automatic upgrades.
    auto_upgrade_interval_h: Option<Hours>,
    /// If `true`, automatic upgrades include release candidates (see Versions & Releases).
    upgrade_to_pre_releases: Option<bool>,
    /// Keep temporary failed transfers for this many hours. While the temporaries are kept, the
    /// data they contain need not be transferred again.
    keep_temporaries_h: Option<Hours>,
    /// Whether to cache the results of ignore pattern evaluation. Performance at the price of
    /// memory. Defaults to `false` as the cost for evaluating ignores is usually not significant.
    cache_ignored_files: Option<bool>,
    /// How often in seconds the progress of ongoing downloads is made available to the GUI.
    progress_update_interval_s: Option<Seconds>,
    /// Whether to apply bandwidth limits to devices in the same broadcast domain as the local
    /// device.
    limit_bandwidth_in_lan: Option<bool>,
    /// The minimum required free space that should be available on the partition holding the
    /// configuration and index. The element content is interpreted according to the given unit
    /// attribute. Accepted unit values are % (percent of the disk / volume size), kB, MB, GB and
    /// TB. Set to zero to disable.
    min_home_disk_free: Option<MinDiskFree>,
    /// The URL from which release information is loaded, for automatic upgrades.
    #[serde(rename = "releasesURL")]
    releases_url: Option<Url>,
    /// Networks that should be considered as local given in CIDR notation.
    always_local_nets: Option<Vec<IpNet>>,
    /// If set, device names will always be overwritten with the name given by remote on each
    /// connection. By default, the name that the remote device announces will only be adopted when
    /// a name has not already been set.
    overwrite_remote_device_names_on_connect: Option<bool>,
    /// When exchanging index information for incomplete transfers, only take into account files
    /// that have at least this many blocks.
    temp_index_min_blocks: Option<Count>,
    /// ID of a notification to be displayed in the web GUI. Will be removed once the user
    /// acknowledged it (e.g. an transition notice on an upgrade).
    #[serde(rename = "unackedNotificationIDs")]
    unacked_notification_ids: Option<Vec<String>>,
    /// Specify a type of service (TOS)/traffic class of outgoing packets.
    traffic_class: Option<TrafficClass>,
    /// Syncthing will attempt to lower its process priority at startup. Specifically: on Linux, set
    /// itself to a separate process group, set the niceness level of that process group to nine and
    /// the I/O priority to best effort level five; on other Unixes, set the process niceness level
    /// to nine; on Windows, set the process priority class to below normal. To disable this
    /// behavior, for example to control process priority yourself as part of launching Syncthing,
    /// set this option to `false`.
    set_low_priority: Option<bool>,
    /// This option controls how many folders may concurrently be in I/O-intensive operations such
    /// as syncing or scanning. The mechanism is described in detail in a separate chapter.
    max_folder_concurrency: Option<Count>,
    #[serde(rename = "crURL")]
    /// Server URL where automatic crash reports will be sent if enabled.
    cr_url: Option<Url>,
    /// Switch to opt out from the automatic crash reporting feature. Set false to keep Syncthing
    /// from sending panic logs on serious troubles. Defaults to true, to help the developers
    /// troubleshoot.
    crash_reporting_enabled: Option<bool>,
    /// Interval in seconds between contacting a STUN server to maintain NAT mapping. Default is 24
    /// and you can set it to 0 to disable contacting STUN servers. The interval is automatically
    /// reduced if needed, down to a minimum of [`stun_keepalive_min_s`].
    stun_keepalive_start_s: Option<Seconds>,
    /// Minimum for the `stun_keepalive_start_s` interval, in seconds.
    stun_keepalive_min_s: Option<Seconds>,
    /// Servers to be used for STUN, given as ip:port. The keyword `default` gets expanded to
    /// `stun.callwithus.com:3478`, `stun.counterpath.com:3478`, `stun.counterpath.net:3478`,
    /// `stun.ekiga.net:3478`, `stun.ideasip.com:3478`, `stun.internetcalls.com:3478`,
    /// `stun.schlund.de:3478`, `stun.sipgate.net:10000`, `stun.sipgate.net:3478`,
    /// `stun.voip.aebc.com:3478`, `stun.voiparound.com:3478`, `stun.voipbuster.com:3478`,
    /// `stun.voipstunt.com:3478` and s`tun.xten.com:3478` (this is the default).
    stun_servers: Option<Vec<StunServer>>,
    /// Controls how Syncthing uses the backend key-value database that stores the index data and
    /// other persistent data it needs. The available options and implications are explained in a
    /// separate chapter.
    database_tuning: Option<DatabaseTuning>,
    /// This limits how many bytes we have “in the air” in the form of response data being read and
    /// processed.
    #[serde(rename = "maxConcurrentIncomingRequestKiB")]
    max_concurrent_incoming_request_kib: Option<Kibibytes>,
    /// Enable (the default) or disable announcing private (RFC1918) LAN IP addresses to global
    /// discovery.
    #[serde(rename = "announceLANAddresses")]
    announce_lan_addresses: Option<bool>,
    /// Controls whether all index data is resent when an upgrade has happened, equivalent to
    /// starting Syncthing with `--reset-deltas`. This used to be the default behavior in older
    /// versions, but is mainly useful as a troubleshooting step and causes high database churn. The
    /// default is now `false`.
    send_full_index_on_upgrade: Option<bool>,
    /// Feature flags are simple strings that, when added to the configuration, may unleash
    /// unfinished or still-in-development features to allow early user testing. Any supported value
    /// will be separately announced with the feature, so that regular users do not enable it by
    /// accident.
    feature_flags: Option<Vec<String>>,
    /// The number of connections at which we stop trying to connect to more devices, zero meaning
    /// no limit. Does not affect incoming connections. The mechanism is described in detail in a
    /// separate chapter.
    // TODO: Type for which 0 means no limit
    connection_limit_enough: Option<Count>,
    /// The maximum number of connections which we will allow in total, zero meaning no limit.
    /// Affects incoming connections and prevents attempting outgoing connections. The mechanism is
    /// described in detail in a separate chapter.
    connection_limit_max: Option<Count>,
    /// Only for compatibility with old versions of Syncthing on remote devices.
    ///
    /// Syncthing by default allows only TLS 1.3 or higher for sync connections. Setting this option
    /// makes Syncthing also allow TLS 1.2. Use only for compatibility with very old Syncthing
    /// instances, or other implementations as required.
    ///
    /// This option does not affect the GUI/API connections; those always allow TLS 1.2 or higher.
    ///
    /// The default value is `false`.
    #[serde(rename = "insecureAllowOldTLSVersions")]
    insecure_allow_old_tls_versions: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
enum ListenAddress {
    #[serde(with = "strings::default")]
    Default,
    Address(Url),
}

/// A URI to a global announce (discovery) server, or the word default to include the default
/// servers. The syntax for non-default entries is that of an HTTP or HTTPS URL. A number of options
/// may be added as query options to the URL: `insecure` to prevent certificate validation (required
/// for HTTP URLs) and `id=<device ID>` to perform certificate pinning. The device ID to use is
/// printed by the discovery server on startup.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
enum AnnounceServer {
    #[serde(with = "strings::default")]
    Default,
    Address(Url),
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
enum StunServer {
    #[serde(with = "strings::default")]
    Default,
    // TODO: ip:port
    Address(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
enum DatabaseTuning {
    Small,
    Large,
    Auto,
}

mod strings {
    use crate::utils::named_unit_variant;
    named_unit_variant!(default);
}