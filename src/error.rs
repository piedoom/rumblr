use std::error;
use std::fmt;

#[derive(Debug)]
pub enum TumblrError {
    Network,
    Parse,
}

impl error::Error for TumblrError {
    fn description(&self) -> &str {
        match *self {
            // TODO: Make more descriptive
            TumblrError::Network => "Network error.",
            TumblrError::Parse => "Parsing error.",
        }
    }
}

impl fmt::Display for TumblrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TumblrError::Network => f.write_str("NetworkError"),
            TumblrError::Parse => f.write_str("ParseError"),
        }
    }
}
