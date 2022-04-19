use serde::{Deserialize, Serialize};

pub type IgnorePattern = String;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Ignores {
    pub lines: Vec<IgnorePattern>,
}
