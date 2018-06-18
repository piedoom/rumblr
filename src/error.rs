use hyper;
use std::convert::From;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Network,
    Parse,
    Request(hyper::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::Network => f.write_str("NetworkError"),
            Error::Parse => f.write_str("ParseError"),
            Error::Request(e) => e.fmt(f),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Request(err)
    }
}
