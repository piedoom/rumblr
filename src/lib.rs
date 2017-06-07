extern crate hyper;
extern crate serde_json;

use std::io::Read;
use serde_json::Value;

const PATH : &str = "/api/read/json?debug=1";

pub struct TumblrClient {
    
}

impl TumblrClient{

    /// Alias for getting the actual JSON file from the V1 api
    fn get(username: &str) -> Value {

        // create a new request
        let client = hyper::Client::new();
        let request_url = format!("http://{}.tumblr.com{}", username, PATH);

        // unwrap and get our request
        let mut response = client.get(&request_url).send().unwrap();

        // return the body
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();
        let v: Value = serde_json::from_str(&body).unwrap();
        v
    }

    pub fn blog_info(username: &str) -> String{
        let info = TumblrClient::get(username);
        format!("{}", info["tumblelog"]["description"])
    }
}