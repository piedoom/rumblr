extern crate rumblr;
use rumblr::client::Client;
use std::env;

fn main(){
	// get our OAUTH variables from env vars
	let consumer_key =			&env::var("TUMBLR_CONSUMER_KEY").expect("token");
	let consumer_secret = 		&env::var("TUMBLR_CONSUMER_SECRET").expect("token");
	let oauth_token  = 			&env::var("TUMBLR_OAUTH_TOKEN").expect("token");
	let oauth_token_secret  = 	&env::var("TUMBLR_OAUTH_TOKEN_SECRET").expect("token");

	let c = Client {
		consumer_key,
		consumer_secret,
		oauth_token,
		oauth_token_secret,
		.. Client::default()
	};

	// // get current user primary blog name
	let r = c.user_info().send();
    println!("{}", r.unwrap().name);

    // get staff's blog name
    let r = c.blog_info("staff").send();
    println!("{}", r.unwrap().title);

    

}