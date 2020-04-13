use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Version {
    pub arch: String, //FIXME:use enum
    pub long_version: String,
    pub os: String, //FIXME:use enum
    pub version: String,
}
