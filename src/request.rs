//! Constructor for requests to be built without repetition

use hyper::method::Method;
use error::TumblrError;
use client::Client;
use oauthcli::{ OAuthAuthorizationHeader, OAuthAuthorizationHeaderBuilder, SignatureMethod };
use oauthcli::url;
use utility;
use data::root::Root;
use serde_json;
use std::io::Read;
use hyper::header::Authorization;
use hyper;
use std::collections::HashMap;


/// Request
#[derive(Clone)]
pub struct Request<'a> {
	method: Method,
	url: String,
	params: Vec<(String,String)>,
	client: &'a Client<'a>
}

impl<'a> Request<'a> {
	/// get a particular request and return the outcome
	pub fn go(&self) -> Result<Root, TumblrError> {
		let header = self.build_request();

		// Get back the root element
		let result = self.send_and_deserialize(header);

		// match and return root object
		match result {
			// First make sure we had no errors
			Ok(t) => Ok(t),
			Err(e) => Err(TumblrError::Parse)
		}
	}

	/// Constructs a request with Hyper and oauth headers
    fn build_request(&self) -> OAuthAuthorizationHeader {
    	let req = self.clone();
        OAuthAuthorizationHeaderBuilder::new(
            req.method.to_string(),
            // TODO: error on invalid URL
            &url::Url::parse(&req.url).unwrap(),
            req.client.consumer_key,
            req.client.consumer_secret,
            SignatureMethod::HmacSha1
        )
        .token(req.client.oauth_token, req.client.oauth_token_secret)
        .request_parameters(req.params.into_iter())
        .finish()
    }

    /// Send a given request and deserialize the response
	pub fn send_and_deserialize(&self, header: OAuthAuthorizationHeader) -> Result<Root, TumblrError> {

		let req = self.clone();

	    // Send the request
	    let response = req.client.hyper
	        .request(req.method, &req.url)
	        .header(Authorization(header.to_string()))
	        .send();

	    // Return early if the request failed
	    if response.is_err() { return Err(TumblrError::Network) }

	    // Read our response and write to a buffer.
	    // We can just `unwrap` here since we already error checked.
	    let mut buf = String::new();
	    response.unwrap().read_to_string(&mut buf); 

	    // deserialize from JSON to a `Root` object
	    let result = serde_json::from_str(&buf);

	    match result {
	        Ok(result) => Ok(result),
	        Err(_) => Err(TumblrError::Parse)
	    }
	}
}

/// Request factory
#[derive(Clone)]
pub struct RequestFactory<'a> {
	method: Method,
	url: String,
	params: Vec<(String,String)>,
	client: &'a Client<'a>
}

impl<'a> RequestFactory<'a> {
	/// Initialize a new default `RequestFactory`
	pub fn new(client: &'a Client) -> RequestFactory<'a> {
		RequestFactory {
			url: "".to_string(),
			method: Method::Get,
			params: vec![],
			client: client
		}
	}

	/// Change the HTTP method of the request
	pub fn method(mut self, method: Method) -> Self {
		self.method = method;
		self
	}

	/// Set the endpoint url for the request
	pub fn url(mut self, url: String) -> Self {
		self.url = url;
		self
	}

	/// Set the parameters of the request as a vector of tuple pairs.
	/// This will replace any already set data in this field.
	pub fn params(mut self, params: Vec<(String, String)>) -> Self {
		self.params = params;
		self
	}

	/// Add a sing parameter to the request.  This does not replace
	/// any already set data.
	pub fn add_param(mut self, param: (String, String)) -> Self {
		self.params.push(param);
		self
	}

	/// Finalize the factory and return a `Request`
	pub fn finalize(mut self) -> Request<'a> {
		Request {
			method: self.method,
			url: self.url,
			params: self.params,
			client: self.client
		}
	}
}