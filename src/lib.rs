#[macro_use]
extern crate serde_derive;

#[cfg(test)]
use mockito;

mod client;
pub mod data;
mod error;

pub use client::Client;
pub use error::Error;

// Depending on our build settings, we set the API URL to different
// strings for easy testing via mockito.
fn api_url<'a>() -> String {
    #[cfg(not(test))]
    let url: String = String::from("http://api.tumblr.com/v2");

    #[cfg(test)]
    let url: String = mockito::server_url();

    url
}
