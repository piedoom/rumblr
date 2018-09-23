/// Contains information about a user blog.  
/// Some data is wrapped in an optional as it may not be visible depending on if the current user is the admin
/// of the blog.
#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    /// These intial fields are accessible publicly.  All `BlogInfo` structures
    /// will have these values, so there is no need for optionals here.
    /// Whether the blog can be asked questions
    pub ask: bool,
    /// Whether the blog allows anonymous questions
    pub ask_anon: bool,
    /// The title of the ask page, if enabled
    pub ask_page_title: String,
    /// Unknown, possibly deprecated field of Tumblr's V1 RSS/Atom API
    pub can_subscribe: bool,
    /// A short user made description of the blog
    pub description: String,
    /// Functionally similar to `is_adult` with all the same caveats.
    pub is_nsfw: bool,
    /// The name of this blog.  `https://{name}.tumblr.com`.
    pub name: String,
    /// The number of posts for this blog.
    pub posts: u32,
    pub share_likes: bool,
    /// Similar to `can_subscribe`.  This is NOT the same as following.
    pub subscribed: bool,
    /// The short user provided title for this blog.
    pub title: String,
    /// Similar to `posts`.  It is unclear how `total_posts` differs from `posts`.
    /// My hypothesis is that `total_posts` counts private, unpublished, or drafted posts.
    /// Tumblr keeps these numbers on record in order to implement their post limiting features.
    pub total_posts: u32,
    /// Unix timestamp of when the blog was last updated.
    /// This will be changed to a regular DateTime object in the future
    pub updated: u64,
    /// The full url of our blog.  If the user has a custom URL, it will be displayed here.
    pub url: String,

    /* PRIVATE API FIELDS */
    /// The following fields are dependent on whether or not the user owns the blog in question.
    /// Therefore, they are wrapped in optionals.
    /// Whether or not the accessing user is the admin of this blog.
    pub admin: Option<bool>,
    /// Whether or not other blogs can send this blog fan mail.
    /// This feature is soft-deprecated in leiu of the messaging feature
    /// that is unfortunately not accessible through a 3rd party application.
    pub can_send_fan_mail: Option<bool>,
    /// The number of posts a blog has drafted.
    pub drafts: Option<u32>,
    /// Whether or not this blog also posts to facebook automatically.
    pub facebook: Option<String>, // TODO: change to enum
    /// Whether or not opengraph is enabled.
    pub facebook_opengraph_enabled: Option<String>, // TODO: change to enum
    /// Whether or not we follow this blog.
    /// Note: it's totally counter intuitive that this field
    /// is only accessible if you are the administrator, but :shrug emoji:
    pub followed: Option<bool>,
    /// The amount of followers this blog has.
    pub followers: Option<u32>,
    /// Whether or not this blog is blocked from the primary account.
    pub is_blocked_from_primary: Option<bool>,
    /// The amount of messages (unread and read) that this blog has.
    pub messages: Option<u32>,
    /// Whether or not this is the user's primary blog.  This is generally
    /// unnecessary information as far as API usage goes, but it is important to
    /// remember that when a user follows/likes/etc., it is the primary blog
    /// that will show as followed/liked/etc.
    pub primary: Option<bool>,
    /// the number of posts this blog has in its queue.
    pub queue: Option<u32>,
    /// Whether or not this blog also posts to twitter automatically.
    pub tweet: Option<String>, // TODO: change to enum
    /// Unknown.  Same as `tweet`?
    pub twitter_enabled: Option<bool>,
    /// Unknown.  Same as `tweet`?
    pub twitter_send: Option<bool>,
    /// The visibility of this blog (known as `type` in the JSON API).  
    /// Can be public or private.
    #[serde(rename = "type")]
    pub visibility: Option<BlogVisibility>,
}

/// See `type` field on `BlogInfo` struct.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)] // serde limitation
pub enum BlogVisibility {
    private,
    public,
}
