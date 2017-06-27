/// The image avatar that belongs to a blog
use client::Client;
use hyper::method::Method;
use data;
use data::root::{Response};
use utility;
use error::Error;
use super::BLOG_PATH;

/// Avatar info
pub struct Avatar<'a, 'b> {
	client: &'a Client<'a>,
    blog: &'b str,
    size: usize
}

impl<'a, 'b> Avatar<'a, 'b> {

	/// return a new `Info` struct
    /// usually called by our Client object
    pub fn new(client: &'a Client, blog: &'b str) -> Self {
        Avatar { 
        	client: client,
            blog: blog,
            // default value
            size: 64
        }
    }

    /// Set the target blog
    pub fn blog(&mut self, blog: &'b str) -> &mut Self {
        self.blog = blog;
        self
    }

    /// Set the desired avatar size.
    /// The available sizes are 16, 24, 30, 40, 48, 64, 96, and 512.
    /// If an "invalid" value is provided, it will floor itself to the nearest valid value
    pub fn size(&mut self, size: usize) -> &mut Self {
    	// check the size and floor invalid values
    	self.size = match size {
    		0...23 => 16,
    		24...29 => 24,
    		30...39 => 30,
    		40...47 => 40,
    		48...63 => 48,
    		64...95 => 64,
    		96...511 => 96,
    		512...1024 => 512,
    		// default
    		_ => 64
    	};
    	self
    }

    /// finally send the request and solidify the `Info` struct
    pub fn send(&mut self) -> Result<data::avatar::Avatar, Error> {

    	let url = format!("{}/{}/avatar/{}", BLOG_PATH, self.blog, self.size);

		println!("{}", url);

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
                    Response::avatar(avatar) => Ok(avatar),
                    _  => Err(Error::Unknown)
                }
            }
            Err(e) => {
                Err(e)
            }
        }

    }
}
