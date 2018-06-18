#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
enum PostState {
    published,
    queued,
    draft,
    private,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    id: u32,
    // #[serde(rename="type")]
    // post_type: 			PostType,
    // #[serde(rename="post_url")]
    // url:				String,
    // timestamp:			u64,
    // date:				u64,
    // format:				Format,
    // reblog_key:			String,
    // tags:				Vec<String>,
    // source_url:			String,
    // source_title:		String,
    // liked:				bool,
    // state:				PostState,
    // total_posts:		u32
}
