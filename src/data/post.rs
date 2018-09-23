#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum PostState {
    Published,
    Queued,
    Draft,
    Private,
}

/// All generic fields guarenteed to be present on every type of post
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
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
    #[serde(flatten)]
    content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Content {
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
    title: Option<String>,
    body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Answer {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Photo {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {

}
