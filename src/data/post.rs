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
    #[serde(flatten)]
    info: PostInfo,
    title: Option<String>,
    body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    #[serde(flatten)]
    info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    #[serde(flatten)]
    info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Answer {
    #[serde(flatten)]
    info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    #[serde(flatten)]
    info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    #[serde(flatten)]
    info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Photo {
    #[serde(flatten)]
    info: PostInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    #[serde(flatten)]
    info: PostInfo,
}


/// All generic fields guarenteed to be present on every type of post
#[derive(Serialize, Deserialize, Debug)]
pub struct PostInfo {
    id: usize,
    #[serde(rename="post_url")]
    url:				    String,
    timestamp:			    usize,
    date:				    String,
    // format:				Format,
    reblog_key:			    String,
    tags:				    Vec<String>,
    source_url:			    Option<String>,
    source_title:		    Option<String>,
    // liked:				bool,
    // state:				PostState,
    // total_posts:		u32
}
