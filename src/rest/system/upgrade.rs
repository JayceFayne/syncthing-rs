use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpgradeInfo {
    //TODO: version enum
    pub latest: String,
    pub major_newer: bool,
    pub newer: bool,
    //TODO: version enum
    pub running: String,
}
