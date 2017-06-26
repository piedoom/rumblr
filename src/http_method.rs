pub enum HttpMethod {
	Get,
	Post
}

impl HttpMethod {
	/// convert enum to a string for use in other libraries
	pub fn to_string(&self) -> &str {
		match self {
			&HttpMethod::Get => return "GET",
			&HttpMethod::Post => return "POST"
		}
	}
}