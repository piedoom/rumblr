use std;

/// Specifies the format to return
/// The default is HTML
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum Format {
	/// Plain text, no HTML
	text,
	/// As entered by the user
	raw,
	/// Formatted HTML
	html
}

impl std::string::ToString for Format {
	fn to_string(&self) -> String {
		return match self {
			&Format::text => "text".to_string(),
			&Format::raw => "raw".to_string(),
			&Format::html => "html".to_string()
		}
	}
}