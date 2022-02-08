extern crate base64;
extern crate hmac;
extern crate openssl;
extern crate rand;

pub use crate::error::{Error, RequestError};
pub use crate::params::Headers;
pub use crate::resources::*;

pub use self::config::Client;
pub use self::config::Response;

mod client {
    pub mod r#async;
}

mod encryption;
mod error;
pub mod params;
mod resources;

mod config {
    pub type Client = crate::client::r#async::Client;
    pub type Response<T> = crate::client::r#async::Response<T>;
}
