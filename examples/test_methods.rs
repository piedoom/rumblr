extern crate rumblr;
use rumblr::client::Client;
use std::env;

fn main() {
    // get our OAUTH variables from env vars
    let consumer_key = &env::var("TUMBLR_CONSUMER_KEY").expect("token");
    let consumer_secret = &env::var("TUMBLR_CONSUMER_SECRET").expect("token");
    let oauth_token = &env::var("TUMBLR_OAUTH_TOKEN").expect("token");
    let oauth_token_secret = &env::var("TUMBLR_OAUTH_TOKEN_SECRET").expect("token");

    let c = Client {
        consumer_key,
        consumer_secret,
        oauth_token,
        oauth_token_secret,
        ..Client::default()
    };

    // get current user primary blog name
    println!("Your blog name is: {}", c.user().unwrap().name);

    // get staff's blog name
	println!("Staff name is: {}", c.blog("staff").unwrap().name);

    // get our dashboard
	println!("Our dashboard: {:?}", c.dashboard().unwrap())
    // let r = c.posts("staff");
    // println!("{:?}", r.unwrap())
}
