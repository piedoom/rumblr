#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PostState {
    Published,
    Queued,
    Draft,
    Private,
    Submission,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Html,
    Markdown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    Text,
    Raw
}

/// All generic fields guarenteed to be present on every type of post
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    id: usize,
    #[serde(rename = "post_url")]
    url: String,
    timestamp: usize,
    date: String,
    format: Format,
    reblog_key: String,
    tags: Vec<String>,
    source_url: Option<String>,
    source_title: Option<String>,
    liked: bool,
    state: PostState,
    #[serde(flatten)]
    content: Content,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Submission {
    pub id: usize,
    #[serde(rename = "post_url")]
    pub url: String,
    pub state: PostState,
    pub question: String,
    pub answer: String,
    pub timestamp: usize,
    pub date: String,
    pub format: Format,
    pub reblog_key: String,
    pub tags: Vec<String>,
    pub source_url: Option<String>,
    pub source_title: Option<String>,
    pub liked: bool,
    pub asking_name: String,
    pub asking_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    title: Option<String>,
    body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
    /// The actual quote
    title: String,
    /// The source of the quote (optional)
    source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    /// The optional title of the page the link points to
    title: String,
    /// The link itself
    url: String,
    /// The author of the article that the link points to
    author: Option<String>,
    /// An excerpt from the article that the link points to
    excerpt: Option<String>,
    /// The publisher of the article that the link points to
    publisher: String,
    // photos:
    /// An optional user-supplied description
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    /// The blog that sent this ask, or answered it if it was privately answered
    asking_name: String,
    /// The blog URL that sent this ask, or answered it if it was privately answered
    asking_url: String,
    /// The question being asked
    question: String,
    /// The answer given
    answer: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    /// Optional user supplied caption
    caption: Option<String>,
    // TODO: player
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Audio {
    /// Optional user supplied caption
    caption: Option<String>,
    /// HTML embed for player
    player: String,
    /// Number of times an audio post has been played
    plays: usize,
    /// Optional album art URL
    album_art: Option<String>,
    /// Optional Artist Value
    artist: Option<String>,
    /// Optional Album Value
    album: Option<String>,
    /// Optional Track Name
    track_name: Option<String>,
    /// Optional Track Number
    track_number: Option<String>,
    /// Optional year
    year: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Photo {
    // TODO:
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    /// Optional title of the post
    title: Option<String>,
    /// The full chat body
    body: String,
    // TODO: Array of chat items
}
