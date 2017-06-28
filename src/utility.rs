use oauthcli::url;
use oauthcli::{ OAuthAuthorizationHeader, OAuthAuthorizationHeaderBuilder, SignatureMethod };
use client::Client;
use data::root::{Root};
use serde_json;
use std::io::Read;
use error::Error;
use hyper::header::Authorization;
use hyper;
use std::collections::HashMap;


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

    println!("{}", url);

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

    println!("{:?}", result);

    if let Ok(t) = result {
        Ok(t)
    } else {
        Err(Error::Deserialize)
    }
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