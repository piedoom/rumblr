use crate::data::Blog;

/// Contains all properties relevant to the UserInfo API call
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub likes: u32,
    pub following: u32,
    pub default_post_format: String, // TODO: change to enum
    /// The array of blogs a user owns
    pub blogs: Vec<Blog>,
}
