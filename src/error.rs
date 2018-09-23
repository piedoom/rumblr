use reqwest;
use serde_json;
use std;
use std::convert::From;
use std::fmt::{Display, Formatter, Result};
use oauthcli;

#[derive(Debug)]
pub enum Error {
    Unknown,
    IO(std::io::Error),
    Serde(serde_json::Error),
    Request(reqwest::Error),
    ParseError(oauthcli::url::ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::Unknown => f.write_str("Unknown Error"),
            Error::Serde(e) => e.fmt(f),
            Error::Request(e) => e.fmt(f),
            Error::IO(e) => e.fmt(f),
            Error::ParseError(e) => e.fmt(f),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Request(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<oauthcli::url::ParseError> for Error {
    fn from(e: oauthcli::url::ParseError) -> Self {
        Error::ParseError(e)
    }
}
