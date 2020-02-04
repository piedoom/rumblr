use crate::data::Blog;
use crate::data::{Post, Submission};
use crate::data::User;

/// The Tumblr API often gives us data wrapped in an object that we don't necessarily
/// care about.  We need to deserialize the whole thing, anyways, though.
#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub response: Response,
}

#[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    User(Box<User>),
    Blog(Box<Blog>),
    Posts(Vec<Post>),
}

// Because of the way this shit API is set up, submissions and posts are nearly identical. The way Tumblr is set up and the 
// way serde works are kind of incompatible, which means the easiest way to remedy is to create a secondary root struct
// just for submissions.
#[derive(Serialize, Deserialize, Debug)]
pub struct RootSubmission {
    pub response: ResponseSubmission,
}

#[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSubmission {
    pub posts: Vec<Submission>,
}
