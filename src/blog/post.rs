/// Get an array of posts from a specific blog
use client::Client;
use hyper::method::Method;
use data::root::{Response};
use data;
use utility;
use format::Format;
use post_type::*;
use error::Error;
use std::collections::HashMap;
use super::BLOG_PATH;

/// Blog info
pub struct Posts<'a, 'b> {
	client: &'a Client<'a>,
    blog: &'b str,
    post_type: Option<PostType>,
    params: HashMap<String, String>,
}

impl<'a, 'b> Posts<'a, 'b> {

	/// return a new `Info` struct
    /// usually called by our Client object
    pub fn new(client: &'a Client, blog: &'b str) -> Self {
        Posts { 
        	client: client,
            blog: blog,
            post_type: None,
            params: HashMap::new(),
        }
    }

    /// Set the target blog
    pub fn blog(&mut self, blog: &'b str) -> &mut Self {
        self.blog = blog;
        self
    }

    /// Limit the response to posts with the specified tag
    pub fn tag(&mut self, tag: &'b str) -> &mut Self {
    	self.params.insert("tag".to_string(), tag.to_string());
    	self
    }

    /// Set the amount of posts to return, from 1 - 20
    pub fn limit(&mut self, limit: u32) -> &mut Self {
		self.params.insert("limit".to_string(), limit.to_string());
    	self
    }

    /// Set the amount of posts to return, from 1 - 20
    pub fn offset(&mut self, offset: u32) -> &mut Self {
    	self.params.insert("offset".to_string(), offset.to_string());
    	self
    }

    // TODO: pub fn reblog_info
    // TODO: pub fn notes_info

    /// Specifies the post format to return, other than HTML
    pub fn format(&mut self, format: Format) -> &mut Self {
    	self.params.insert("format".to_string(), format.to_string());
    	self
    }

    /// Find post by ID.  If specified, all other fields are superfluous.
    pub fn id(&mut self, id: u32) -> &mut Self {
    	self.params.clear();
		self.params.insert("id".to_string(), id.to_string());
		self
    }

    /// Specify the post type.
    /// this is not considered a parameter and goes in a different part of the URL.
    pub fn post_type(&mut self, post_type: PostType) -> &mut Self {
    	self.post_type = Some(post_type);
    	self
    }

    /// finally send the request and solidify
    pub fn send(&mut self) -> Result<Vec<data::post::Post>, Error> {

    	let url = format!("{}/{}/posts{}{}", 
    			BLOG_PATH, 
    			self.blog, 
    			to_url_string(&self.post_type), 
    			utility::params(&self.params));

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
                    Response::posts(posts) => Ok(posts),
                    _  => Err(Error::Unknown)
                }
            }
            Err(e) => {
                Err(e)
            }
        }        
    }
}