use oauthcli::url;
use oauthcli::{ OAuthAuthorizationHeader, OAuthAuthorizationHeaderBuilder, SignatureMethod };
use client::Client;
use data::root::{Root};
use serde_json;
use std::io::Read;
use error::Error;
use hyper::header::Authorization;
use hyper;


/// hack since there is no default token
pub fn default_auth_header() -> OAuthAuthorizationHeader {
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

/// combines `send_request` and `read_response`
pub fn send_and_deserialize (client: &Client, method: hyper::method::Method, url: String, auth_header: OAuthAuthorizationHeader) -> Result<Root, Error> {

    // the raw hyper response
    let response = client.hyper
        .request(method, &url)
        .header(Authorization(auth_header.to_string()))
        .send();
    if response.is_err() { return Err(Error::Hyper) }

    /// get our body as a string
    let mut buf = String::new();
    response.unwrap().read_to_string(&mut buf); 

    /// deserialize from JSON to a Root object
    let result = serde_json::from_str(&buf);

    if let Ok(t) = result {
        Ok(t)
    } else {
        Err(Error::Deserialize)
    }
}