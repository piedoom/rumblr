use data::blog::Blog;
use data::post::Post;
use data::user::User;

/// The Tumblr API often gives us data wrapped in an object that we don't necessarily
/// care about.  We need to deserialize the whole thing, anyways, though.
#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub response: Response,
}

#[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    User(User),
    Blog(Blog),
    Posts(Vec<Post>),
}
