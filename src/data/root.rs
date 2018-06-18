use data::blog::Blog;
use data::post::Post;
use data::user::User;

/// The Tumblr API often gives us data wrapped in an object that we don't necessarily
/// care about.  We need to deserialize the whole thing, anyways, though.
#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub response: Response,
}

#[allow(non_camel_case_types)] // serde enum limitations
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    user(User),
    blog(Blog),
    posts(Vec<Post>),
}
