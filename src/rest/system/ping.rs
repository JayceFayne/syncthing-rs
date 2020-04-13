use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Pong {
    Pong,
}

#[derive(Debug, Deserialize)]
pub struct Ping {
    pub ping: Pong,
}
