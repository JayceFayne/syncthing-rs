mod client;
mod event_stream;
#[cfg(test)]
mod tests;

pub use client::Client;
pub use http::uri::Authority;

pub type Fallible<T> = Result<T, anyhow::Error>;
pub use syncthing_types::*;
