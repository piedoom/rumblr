use data::Post;
use data::Response;
use data::User;

use error::Error;
use hyper;
use hyper::method::Method;
use request::RequestFactory;

pub const BLOG_PATH: &str = "http://api.tumblr.com/v2/blog";
pub const USER_PATH: &str = "http://api.tumblr.com/v2/user";

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
    // USER METHODS

    /// Return the user's data struct
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
}
