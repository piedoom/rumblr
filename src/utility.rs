use oauthcli::url;
use oauthcli::{ OAuthAuthorizationHeader, OAuthAuthorizationHeaderBuilder, SignatureMethod };
use client::Client;
use data::root::{Root};
use serde_json;
use std::io::Read;
use hyper::header::Authorization;
use hyper;
use std::collections::HashMap;
use error::TumblrError;

/// Create an `OAuthAuthorizationHeader` with blank data
fn default_auth_header() -> OAuthAuthorizationHeader {
	OAuthAuthorizationHeaderBuilder::new(
	    "GET",
	    &url::Url::parse("http://tumblr.com").unwrap(),
	    "",
	    "",
	    SignatureMethod::HmacSha1
	)
	.token("", "")
    .finish()
}



/// convert a `HashMap` of url params to a string
pub fn params(hash: &HashMap<String, String>) -> String{
    let mut result = String::new();
    let mut first_value = true;
    for (key, val) in hash.iter() {
        // figure out what to prepend the param with
        let p = match first_value {
            true => "?",
            _ => "&"
        };
        first_value = false;
        // add our value to the URL string
        result += &format!("{}{}={}", p, key, val);
    }
    result
}