mod connection;
mod event_stream;
pub mod rest;
mod routes;
#[cfg(test)]
mod tests;
mod utils;

pub use connection::Connection;
pub use event_stream::*;

//TODO: add log + feature flag

pub type Fallible<T> = Result<T, anyhow::Error>;
