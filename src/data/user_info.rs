use data::blog_info::BlogInfo;

/// Contains all properties relevant to the UserInfo API call
pub struct UserInfo {
	pub name: String,
	pub likes: u32,
	pub following: u32,
	pub default_post_format: String, // TODO: change to enum
	/// the array of blogs a user owns
	pub blogs: Vec<BlogInfo>
}

