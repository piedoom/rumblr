use crate::data::Blog;
use crate::data::{Post, Submission, User, Edit, Meta};
/// The Tumblr API often gives us data wrapped in an object that we don't necessarily
/// care about.  We need to deserialize the whole thing, anyways, though.
#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub meta: Meta,
    //#[serde(serialize_with = "is_some_empty_vec")]
    pub response: Option<Response>,
    pub errors: Option<Error>,
}

#[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    User(Box<User>),
    Blog(Box<Blog>),
    Posts(Vec<Post>),
    Edit(Box<Edit>),
    Submissions(Vec<Submission>),
}

// I don't know why it needs to be like this to work. I think it has to do with the fact that the signature for submissions can match that of posts.
#[derive(Serialize, Deserialize, Debug)]
pub struct RootSubmission {
    pub meta: Meta,
    pub response: Option<ResponseSubmission>,
}

#[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSubmission {
    pub posts: Vec<Submission>,
}
#[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub title: String,
    pub code: usize,
    pub detail: String,
}
