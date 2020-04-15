mod client;
mod event_stream;
pub mod rest;
mod routes;
#[cfg(test)]
mod tests;
mod utils;

pub use client::Client;
pub use event_stream::*;

//TODO: add log + feature flag

pub type Fallible<T> = Result<T, anyhow::Error>;
