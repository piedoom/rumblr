//! Defines all methods described in the `User` section of Tumblr's API documentation

use client::Client;
use data::root::Response;
use data::user::User;
use error::Error;
use hyper::method::Method;
use utility;

/// Const path strings
pub const USER_PATH: &str = "http://api.tumblr.com/v2/user";

/// User info has no params, so a builder is unessecary.
/// However, we use one anyway to stay consistent.
pub struct Info<'a> {
    client: &'a Client<'a>,
}

impl<'a> Info<'a> {
    /// return a new Info struct
    /// We don't need any params here so a builder
    /// function is kind of useless, but we're doing this
    /// for the sake of sticking to conventions
    pub fn new(client: &'a Client) -> Self {
        Info { client: client }
    }

    /// Return an `Info` struct about the user and blogs that we own.
    pub fn send(&self) -> Result<User, Error> {
        let url = format!("{}/info", USER_PATH);

        // Build the auth header for our request
        let auth_header = self.client.build_request(vec![], Method::Get, &url);

        // get and deserialize our request
        let result = utility::send_and_deserialize(self.client, Method::Get, url, auth_header);

        match result {
            Ok(t) => match t.response {
                Response::user(user) => Ok(user),
                _ => Err(Error::Parse),
            },
            Err(_) => Err(Error::Parse),
        }
    }
}
