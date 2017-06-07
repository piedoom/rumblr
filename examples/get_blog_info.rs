extern crate rumblr;
use rumblr::TumblrClient;


fn main(){
    let a = TumblrClient::blog_info("doomy");
    println!("{}", a);
}