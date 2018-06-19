use data::Post;
use data::Response;
use data::User;

use error::Error;
use hyper;
use hyper::header::Authorization;
use hyper::method::Method;
use oauthcli::url::Url;
use oauthcli::{OAuthAuthorizationHeader, OAuthAuthorizationHeaderBuilder, SignatureMethod};
use request::RequestFactory;
use serde;
use serde_json;
use std::collections::HashMap;
use std::io::Read;
pub const PATH: &str = "http://api.tumblr.com/v2";

/// Allows us to easily interface with the Tumblr API
#[allow(dead_code)]
pub struct Client<'a> {
    pub consumer_key: &'a str,
    pub consumer_secret: &'a str,
    pub oauth_token: &'a str,
    pub oauth_token_secret: &'a str,
    pub hyper: hyper::Client,
}

/// Return an empty default `Client`
impl<'a> Default for Client<'a> {
    fn default() -> Self {
        Client {
            consumer_key: "",
            consumer_secret: "",
            oauth_token: "",
            oauth_token_secret: "",
            hyper: hyper::Client::new(),
        }
    }
}

impl<'a> Client<'a> {
    /// Create an auth header for use in OAuth
    fn get_auth_header(
        &self,
        method: Method,
        url: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<OAuthAuthorizationHeader, Error> {
        let parsed_url = Url::parse(&url)?;
        let mut auth = OAuthAuthorizationHeaderBuilder::new(
            method.to_string(),
            &parsed_url,
            self.consumer_key,
            self.consumer_secret,
            SignatureMethod::HmacSha1,
        );
        auth.token(self.oauth_token, self.oauth_token_secret);
        match params {
            Some(p) => {
                auth.request_parameters(p);
                ()
            }
            _ => {}
        }
        Ok(auth.finish())
    }

    /// Generic get function used by all other client methods
    fn get<T>(&self, endpoint: &str, params: Option<HashMap<&str, &str>>) -> Result<T, Error>
    where
        T: serde::Deserialize<'a>,
    {
        // build our url
        let url = &format!("{}{}", PATH, endpoint);
        // create an oauth header
        let auth = self.get_auth_header(Method::Get, url, params)?.to_string();
        // use hyper to retrieve data
        let mut res = self
            .hyper
            .get(url)
            .header(Authorization(auth))
            .send()
            .map_err(|e| Error::Request(e))?;
        // read and parse our data
        let mut buf = String::new();
        res.read_to_string(&mut buf)?;
        serde_json::from_str::<T>(&buf).map_err(|e| Error::Serde(e))
    }

    // USER METHODS

    //Return the user's data struct
    /*
    pub fn user(&self) -> Result<User, Error> {
        let url = format!("{}/info", USER_PATH);
        let data = RequestFactory::new(self)
            .method(Method::Get)
            .url(url)
            .finalize()
            .send()?;

        match data.response {
            Response::user(user) => Ok(user),
            _ => Err(Error::Unknown),
        }
    }

    // BLOG METHODS

    /// Get posts from a user
    pub fn posts(&self, blog: &'a str) -> Result<Vec<Post>, Error> {
        let url = format!("{}/{}/posts", BLOG_PATH, blog);
        println!("{}", url);
        let data = RequestFactory::new(self)
            .method(Method::Get)
            .url(url)
            .finalize()
            .send()?;

        match data.response {
            Response::posts(posts) => Ok(posts),
            _ => Err(Error::Unknown),
        }
    }
    */
}
