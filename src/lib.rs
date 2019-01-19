#[macro_use]
extern crate serde_derive;

#[cfg(test)]
use mockito;

mod client;
pub mod data;
mod error;

pub use client::Client;
pub use error::Error;
