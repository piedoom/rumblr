#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum PostState {
    Published,
    Queued,
    Draft,
    Private,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Post {
    Text(Text),
    Photo(Photo),
    Quote(Quote),
    Link(Link),
    Answer(Answer),
    Video(Video),
    Audio(Audio),
    Chat(Chat),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    // info: PostInfo,
    title: Option<String>,
    body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    //info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    //info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Answer {
    //info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    //info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    //info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Photo {
    //info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    //info: PostInfo,
}


/// All generic fields guarenteed to be present on every type of post
#[derive(Serialize, Deserialize, Debug)]
pub struct PostInfo {
    id: usize,
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
