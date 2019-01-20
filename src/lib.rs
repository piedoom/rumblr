#[macro_use]
extern crate serde_derive;

mod client;
pub mod data;
mod error;

pub use client::Client;
pub use error::Error;
