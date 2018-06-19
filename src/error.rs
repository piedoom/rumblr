use hyper;
use serde_json;
use std;
use std::convert::From;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Unknown,
    IO(std::io::Error),
    Serde(serde_json::Error),
    Request(hyper::Error),
    ParseError(hyper::error::ParseError),
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

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
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

impl From<hyper::error::ParseError> for Error {
    fn from(e: hyper::error::ParseError) -> Self {
        Error::ParseError(e)
    }
}
