use hyper;
use oauthcli::{ OAuthAuthorizationHeader, OAuthAuthorizationHeaderBuilder, SignatureMethod };
use http_method::HttpMethod;
use oauthcli::url;
use user_methods;
// use blog_methods;

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
    pub fn build_request(&self, params: Vec<(String, String)>, method: HttpMethod, url: &str) -> OAuthAuthorizationHeader {
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
    pub fn user_info(&self) -> user_methods::info::Info {
        user_methods::info::Info::new(self)
    }
}

