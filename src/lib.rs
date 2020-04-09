mod connection;
mod event_stream;
mod reply;
mod routes;
#[cfg(test)]
mod tests;

pub use connection::Connection;
pub use event_stream::*;
pub use reply::*;

pub type Fallible<T> = Result<T, anyhow::Error>;
