use client::Client;
use http_method::HttpMethod;
use hyper::header::Authorization;
use std::io::Read;
use serde_json;
use data::user::User;
use data::root::{Root,Response};
use super::USER_PATH;

/// User info has no params, so a builder is unessecary.
/// However, we use one anyway to stay consistent.
pub struct Info<'a> {
	client: &'a Client<'a>
}

impl<'a> Info<'a> {

	/// return a new Info struct
	/// We don't need any params here so a builder
	/// function is kind of useless, but we're doing this
	/// for the sake of sticking to conventions
    pub fn new(client: &'a Client) -> Self {
        Info { 
        	client: client
        }
    }

    /// finally send the request
    pub fn send(&self) -> Option<User> {


    	let info_path = format!("{}/info", USER_PATH);

    	let auth_header = self.client.build_request(
    		vec![], 
    		HttpMethod::Get,
    		&info_path );

    	let mut response = String::new();

    	self.client.hyper
    		.get(&info_path)
    		.header(Authorization(auth_header.to_string()))
    		.send()
    		.unwrap()
    		.read_to_string(&mut response);

        
        

    	let result: Root = serde_json::from_str(&response).unwrap();

        return match result.response {
            Response::user(user) => Some(user),
            _  => None
        }
    }
}