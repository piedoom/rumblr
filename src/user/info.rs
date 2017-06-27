use client::Client;
use data::user::User;
use data::root::{Response};
use hyper::method::Method;
use utility;
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


    	let url = format!("{}/info", USER_PATH);

    	let auth_header = self.client.build_request(
    		vec![], 
    		Method::Get,
    		&url );
        
        let result = utility::send_request(self.client, Method::Get, url, auth_header);

        return match result.response {
            Response::user(user) => Some(user),
            _  => None
        }
    }
}