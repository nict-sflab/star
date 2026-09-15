use std::error::Error;

pub use config::ProxyConfig;
pub use proxy::build_server;
pub use upstream::StarUpstream;

pub type BoxError = Box<dyn Error + Send + Sync>;

mod auth;
mod config;
mod http;
mod proxy;
mod upstream;
