use hyper;
use std::convert::From;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum TumblrError {
    Network,
    Parse,
    Request(hyper::Error),
}

impl Display for TumblrError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TumblrError::Network => f.write_str("NetworkError"),
            TumblrError::Parse => f.write_str("ParseError"),
            TumblrError::Request(e) => e.fmt(f),
        }
    }
}

impl From<hyper::Error> for TumblrError {
    fn from(err: hyper::Error) -> Self {
        TumblrError::Request(err)
    }
}
