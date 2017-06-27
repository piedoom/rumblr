/// An avatar image for a blog.
#[derive(Serialize, Deserialize, Debug)]
pub struct Avatar {
	#[serde(rename="avatar_url")]
	/// The url of the avatar image.
	pub url: String
}