use std;

/// enum for type of post.
/// Specific for getting posts as data structs will have additional data.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum PostType {
	text,
	quote,
	link,
	answer,
	video,
	audio,
	photo,
	chat
}

impl std::string::ToString for PostType {
	fn to_string(&self) -> String {
		match self {
			&PostType::text => "text".to_string(),
			&PostType::quote => "quote".to_string(),
			&PostType::link => "link".to_string(),
			&PostType::answer => "answer".to_string(),
			&PostType::video => "video".to_string(),
			&PostType::audio => "audio".to_string(),
			&PostType::photo => "photo".to_string(),
			&PostType::chat => "chat".to_string()
		}
	}
}

/// Converts an option used on an info struct to be used in a URL parameter
pub fn to_url_string (option_type: &Option<PostType>) -> String {
	match option_type {
		&None => "".to_string(),
		&Some(ref e) => format!("/{}", e.to_string()).to_string()
	}
}