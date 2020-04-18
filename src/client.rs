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
        body: Option<HashMap<&str, &str>>,
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
        if let Some(b) = body {
            auth.request_parameters(b);
        }
        Ok(auth.finish())
    }

    /// Set the API endpoint to an arbitrary string.
    pub fn set_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    /// Generic get function used by all other client methods
    fn http<T>(&self, method: Method, endpoint: &str, params: Option<HashMap<&str, &str>>, body: Option<HashMap<&str, &str>>) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        // build our url
        let url = &format!("{}{}", self.url, endpoint);

        // create an oauth header
        let auth = self
            .get_auth_header(method.clone(), &url.to_string(), params.clone(), body.clone())?
            .to_string();
        
        // build the request
        let mut req = self
            .http
            .request(method.clone(), url)
            .header(AUTHORIZATION, auth)
            .header("Content-Type", "application/json");

        // Add URL params or a form body to the request
        if params.is_some() {
            req = req.query(&params.unwrap());
        }

        if body.is_some() {
            req = req.form(&body.unwrap());
        }

        dbg!(&req);

        // send the request
        let mut response = req.send()?;
        dbg!(&response);

        let json = response.json();
        match json {
            Ok(data) => Ok(data),
            Err(e) => Err(Error::Request(e)),
        }
    }

    // USER METHODS

    /// Get our user data
    pub fn user(&self) -> Result<User, Error> {
        let root: Root = self.http(Method::GET, "/user/info", None, None)?;
        if let Some(r) = root.response {
            if let Response::User(u) = r {
                return Ok(*u)
            } 
        }
        Err(Error::Unknown)        
    }

    /// Get our dashboard
    pub fn dashboard(&self, params: Option<HashMap<&str, &str>>) -> Result<Vec<Post>, Error> {
        let root: Root = self.http(Method::GET, "/user/dashboard", params, None)?;

        if let Some(r) = root.response {
            if let Response::Posts(p) = r {
                return Ok(p)
            } 
        }
        Err(Error::Unknown)        
    }

    // BLOG METHODS
    /// Get another user's information
    pub fn blog(&self, blog: &'a str, params: Option<HashMap<&str, &str>>) -> Result<Blog, Error> {
        let root: Root = self.http(Method::GET, &format!("/blog/{}/info", blog), params, None)?;
        if let Some(r) = root.response {
            if let Response::Blog(b) = r {
                return Ok(*b)
            } 
        }
        Err(Error::Unknown)   
    }

    /// Get asks, known as submissions. Tumblr has a really interesting history with asks that probably lead to this nomenclature. I believe it includes both
    /// asks, submissions, and possibly "fan mail", if that is still a thing.
    pub fn submissions(&self, blog: &'a str, params: Option<HashMap<&str, &str>>) -> Result<Vec<Submission>, Error> {
        let root: RootSubmission = self.http(Method::GET, &format!("/blog/{}/posts/submission", blog), params, None)?;
        match root.response {
            Some(r) => Ok(r.posts),
            None => Err(Error::Unknown),
        }
    }

    pub fn edit(&self, blog: &'a str, params: Option<HashMap<&str, &str>>, body: Option<HashMap<&str, &str>>) -> Result<Root, Error> {
        let root: Root = self.http(Method::POST, &format!("/blog/{}/post/edit", blog), params, body)?;
        Ok(root)
    }
}
