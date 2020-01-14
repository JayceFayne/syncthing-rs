mod connection;
mod event;
mod reply;
mod routes;
#[cfg(test)]
mod tests;

pub use connection::Connection;
pub use event::*;
pub use reply::*;

pub use surf::Exception as Error;
pub type Fallible<T> = Result<T, Error>;

pub static API_URL: &str = "http://localhost:8384";
pub static API_HEADER_KEY: &str = "X-API-Key";
