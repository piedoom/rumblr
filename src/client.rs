use hyper;
use hyper::method::Method;
use data::user::User;
use error::TumblrError;
use data::root::Response;
use request::{RequestFactory};

pub const USER_PATH : &str = "http://api.tumblr.com/v2/user";

/// Allows us to easily interface with the Tumblr API
#[allow(dead_code)]
pub struct Client<'a> {
    pub consumer_key: &'a str,
    pub consumer_secret: &'a str,
    pub oauth_token: &'a str,
    pub oauth_token_secret: &'a str,
    pub hyper: hyper::Client
}

/// Return an empty default `Client`
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
    

    // pub fn generic_request<T>(&self) -> 

    // METHODS

    /// Return the user's data struct
    pub fn user(&self) -> Result<User, TumblrError> {

        let url = format!("{}/info", USER_PATH);
        let request = RequestFactory::new(self).method(Method::Get).url(url).finalize().send();

        match request {
            Ok(t) => match t.response {
                Response::user(user) => Ok(user),
                _ => Err(TumblrError::Parse)
            },
            _ => Err(TumblrError::Parse)            
        }
    }

// 
//     // pub fn posts(&self, blog: &'a str) -> blog::post::Posts {
//     //     blog::post::Posts::new(&self, blog)
    // }

}
