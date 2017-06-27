use client::Client;
use hyper::method::Method;
use data::blog::Blog;
use data::root::{Response};
use utility;
use error::Error;
use super::BLOG_PATH;

/// Blog info
pub struct Info<'a, 'b> {
	client: &'a Client<'a>,
    blog: &'b str
}

impl<'a, 'b> Info<'a, 'b> {

	/// return a new `Info` struct
    /// usually called by our Client object
    pub fn new(client: &'a Client, blog: &'b str) -> Self {
        Info { 
        	client: client,
            blog: blog
        }
    }

    /// Set the target blog
    pub fn blog(&mut self, blog: &'b str) -> &mut Self {
        self.blog = blog;
        self
    }

    /// finally send the request and solidify the `Info` struct
    pub fn send(&mut self) -> Result<Blog, Error> {

    	let url = format!("{}/{}/info", BLOG_PATH, self.blog);

    	let auth_header = self.client.build_request(
    		vec![], 
    		Method::Get,
    		&url );

    	let result = utility::send_and_deserialize(self.client, Method::Get, url, auth_header);

        // error check
        return match result {
            // return our data 
            Ok(t) => {
                match t.response {
                    Response::blog(blog) => Ok(blog),
                    _  => Err(Error::Unknown)
                }
            }
            Err(e) => {
                Err(e)
            }
        }        
    }
}