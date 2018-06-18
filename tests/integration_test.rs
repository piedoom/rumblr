extern crate rumblr;
use rumblr::TumblrClient;

#[test]
fn get_blog_name() {
    let result = TumblrClient::blog_info("staff");
    assert_eq!(result, "staff");
}
