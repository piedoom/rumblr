use crate::data::*;
use crate::Error;
use oauthcli::{
    url::Url, OAuthAuthorizationHeader, OAuthAuthorizationHeaderBuilder, SignatureMethod,
};
use reqwest;
use reqwest::{header::AUTHORIZATION, Method};
use serde;
use std::collections::HashMap;

const API_URL: &str = "http://api.tumblr.com/v2";

/// Allows us to easily interface with the Tumblr API
#[allow(dead_code)]
#[derive(Debug)]
pub struct Client<'a> {
    pub consumer_key: &'a str,
    pub consumer_secret: &'a str,
    pub oauth_token: &'a str,
    pub oauth_token_secret: &'a str,
    pub http: reqwest::Client,
    pub url: String,
}

/// Return an empty default `Client`
impl<'a> Default for Client<'a> {
    fn default() -> Self {
        Client {
            consumer_key: "",
            consumer_secret: "",
            oauth_token: "",
            oauth_token_secret: "",
            http: reqwest::Client::new(),
            url: String::from(API_URL),
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
        if let Some(p) = params {
            auth.request_parameters(p);
        }
        Ok(auth.finish())
    }

    /// Set the API endpoint to an arbitrary string.
    pub fn set_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    /// Generic get function used by all other client methods
    fn get<T>(&self, endpoint: &str, params: Option<HashMap<&str, &str>>) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        // build our url
        let url = &format!("{}{}", self.url, endpoint);
        // create an oauth header
        let auth = self
            .get_auth_header(Method::GET, &url.to_string(), params)?
            .to_string();

        let req = self
            .http
            .get(url)
            .header(AUTHORIZATION, auth)
            .header("Content-Type", "application/json")
            .send()?
            .json();

        match req {
            Ok(data) => Ok(data),
            Err(e) => Err(Error::Request(e)),
        }
    }

    // USER METHODS

    /// Get our user data
    pub fn user(&self) -> Result<User, Error> {
        let root: Root = self.get("/user/info", None)?;
        match root.response {
            Response::User(u) => Ok(*u),
            _ => Err(Error::Unknown),
        }
    }

    /// Get our dashboard
    pub fn dashboard(&self) -> Result<Vec<Post>, Error> {
        let root: Root = self.get("/user/dashboard", None)?;
        match root.response {
            Response::Posts(p) => Ok(p),
            _ => Err(Error::Unknown),
        }
    }

    // BLOG METHODS
    /// Get another user's information
    pub fn blog(&self, blog: &'a str) -> Result<Blog, Error> {
        let root: Root = self.get(&format!("/blog/{}/info", blog), None)?;
        match root.response {
            Response::Blog(b) => Ok(*b),
            _ => Err(Error::Unknown),
        }
    }

    /// Get asks, known as submissions. Tumblr has a really interesting history with asks that probably lead to this nomenclature. I believe it includes both
    /// asks, submissions, and possibly "fan mail", if that is still a thing.
    pub fn submissions(&self, blog: &'a str) -> Result<Vec<Submission>, Error> {
        let root: RootSubmission = self.get(&format!("/blog/{}/posts/submission", blog), None)?;
        Ok(root.response.posts)
    }
}
