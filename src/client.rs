use hyper;
use oauthcli::{ OAuthAuthorizationHeader, OAuthAuthorizationHeaderBuilder, SignatureMethod };
use hyper::method::Method;
use oauthcli::url;
use user;
use blog;

/// The Client struct that we will use to interface with our API
#[allow(dead_code)]
pub struct Client<'a> {
    pub consumer_key: &'a str,
    pub consumer_secret: &'a str,
    pub oauth_token: &'a str,
    pub oauth_token_secret: &'a str,
    pub hyper: hyper::Client
}

impl<'a> Default for Client<'a> {
    fn default() -> Self {
        Client {
            consumer_key: "",
            consumer_secret: "",
            oauth_token: "",
            oauth_token_secret: "",
            hyper: hyper::Client::new()
        }  
    }
}

impl<'a> Client<'a>{

    /// constructs a get request with Hyper and oauth headers
    ///
    /// * `params` - a vector of tuple parameters to pass into the request
    /// * `method` - the HTTP method - either a GET or POST
    /// * `url` - the base URL for this call, usually based off of `BLOG_PATH` or `USER_PATH`
    pub fn build_request(&self, params: Vec<(String, String)>, method: Method, url: &str) -> OAuthAuthorizationHeader {
        OAuthAuthorizationHeaderBuilder::new(
            method.to_string(),
            &url::Url::parse(url).unwrap(),
            self.consumer_key,
            self.consumer_secret,
            SignatureMethod::HmacSha1
        )
        .token(self.oauth_token, self.oauth_token_secret)
        .request_parameters(params.into_iter())
        .finish()
    }

    /// METHODS
    pub fn user_info(&self) -> user::info::Info {
        user::info::Info::new(&self)
    }

    pub fn blog_info(&self, blog: &'a str) -> blog::info::Info {
        blog::info::Info::new(&self, blog)
    }

    pub fn avatar(&self, blog: &'a str) -> blog::avatar::Avatar {
        blog::avatar::Avatar::new(&self, blog)
    }
}

