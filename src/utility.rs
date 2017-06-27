use oauthcli::url;
use oauthcli::{ OAuthAuthorizationHeader, OAuthAuthorizationHeaderBuilder, SignatureMethod };
use client::Client;
use data::root::{Root, Response};
use serde_json;
use std::io::Read;
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

/// send the request and get back a root object
pub fn send_request(client: &Client, method: hyper::method::Method, url: String, auth_header: OAuthAuthorizationHeader) -> Root {
        
    // the raw hyper response
    let response = client.hyper
        .request(method, &url)
        .header(Authorization(auth_header.to_string()))
        .send();

    // the new result response
    read_response(response)    
}

/// read the hyper response to a JSON root object
pub fn read_response(response: Result<hyper::client::Response, hyper::Error>) -> Root {

    /// get our body as a string
    let mut buf = String::new();
    response.unwrap().read_to_string(&mut buf);

    /// deserialize from JSON to a Root object
    let result: Root = serde_json::from_str(&buf).unwrap();
    result
}