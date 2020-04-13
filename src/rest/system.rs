pub mod connections;
pub mod debug;
pub mod discovery;
pub mod error;
pub mod log;
pub mod ping;
pub mod upgrade;
pub mod version;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub when: String,
    pub message: String,
}
