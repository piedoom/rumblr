use client::Client;
use hyper::method::Method;
use data::blog::Blog;
use data::root::{Response};
use utility;
use super::BLOG_PATH;

/// Blog info
pub struct Info<'a, 'b> {
	client: &'a Client<'a>,
    blog: &'b str
}

impl<'a, 'b> Info<'a, 'b> {

	/// return a new `Info` struct
    /// usually called by our Client object
    pub fn new(client: &'a Client) -> Self {
        Info { 
        	client: client,
            blog: ""
        }
    }

    /// Set the target blog
    pub fn blog(&mut self, blog: &'b str) -> &mut Self {
        self.blog = blog;
        self
    }

    /// finally send the request and solidify the `Info` struct
    pub fn send(&mut self) -> Option<Blog> {

    	let url = format!("{}/{}/info", BLOG_PATH, self.blog);

    	let auth_header = self.client.build_request(
    		vec![], 
    		Method::Get,
    		&url );

    	let result = utility::send_request(self.client, Method::Get, url, auth_header);

        return match result.response {
            Response::blog(blog) => Some(blog),
            _  => None
        }
    }
}