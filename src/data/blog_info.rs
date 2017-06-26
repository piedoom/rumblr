/// Contains information about a user blog.  
/// Some data is wrapped in an optional as it may not be visible depending on if the current user is the admin
/// of the blog.
pub struct BlogInfo {

	/// These intial fields are accessible publicly.  All `BlogInfo` structures
	/// will have these values, so there is no need for optionals here.

	/// Whether the blog can be asked questions
	ask: 				bool,
	/// Whether the blog allows anonymous questions
	ask_anon: 			bool,
	/// The title of the ask page, if enabled
	ask_page_title: 	String,
	/// Unknown, possibly deprecated field of Tumblr's V1 RSS/Atom API
	can_subscribe: 		bool,
	/// A short user made description of the blog
	description: 		String,
	/// Whether or not the blog is adult.  Note that
	/// `is_nsfw` and `is_adult` seem to do the same thing.
	/// Tumblr pushed updates to their content screening 
	/// recently and staff have confirmed that this is still changing.
	/// Unfortunately, Tumblr does not seem to properly version their API 
	/// so this may break at any time.
	is_adult: 			bool,
	/// Functionally similar to `is_adult` with all the same caveats.
	is_nsfw: 			bool,
	/// The name of this blog.  `https://{name}.tumblr.com`.
	name: 				String,
	/// The number of posts for this blog.
	posts: 				u32,
	/// Unknown.  Most likely an enum of how the user can be replied to on posts.
	/// E.g., 1 = cannot reply at all, 2 = can reply after time, 3 = can reply.
	/// However, replying to posts is not even a documented API feature so this field is largley useless
	/// for the time being.
	reply_conditions: 	String,
	share_likes: 		bool,
	/// Similar to `can_subscribe`.  This is NOT the same as following.
	subscribed: 		bool,
	/// The short user provided title for this blog.
	title: 				String,
	/// Similar to `posts`.  It is unclear how `total_posts` differs from `posts`.
	/// My hypothesis is that `total_posts` counts private, unpublished, or drafted posts.
	/// Tumblr keeps these numbers on record in order to implement their post limiting features.
	total_posts: 		u32,
	/// Unix timestamp of when the blog was last updated.
	/// This will be changed to a regular DateTime object in the future
	updated: 			u64,
	/// The full url of our blog.  If the user has a custom URL, it will be displayed here.
	url: 				String

	/* PRIVATE API FIELDS */

	/// The following fields are dependent on whether or not the user owns the blog in question.
	/// Therefore, they are wrapped in optionals.
	/// Whether or not the accessing user is the admin of this blog.
	admin: 				Optional<bool>,
	/// Whether or not other blogs can send this blog fan mail.
	/// This feature is soft-deprecated in leiu of the messaging feature
	/// that is unfortunately not accessible through a 3rd party application.
	can_send_fan_mail: 	Optional<bool>,
	/// The number of posts a blog has drafted.
	drafts: 			Optional<u32>,
	/// Whether or not this blog also posts to facebook automatically.
	facebook: 			Optional<String>, 					// TODO: change to enum
	/// Whether or not opengraph is enabled.
	facebook_opengraph_enabled: 	Optional<String>,		// TODO: change to enum
	/// Whether or not we follow this blog.
	/// Note: it's totally counter intuitive that this field
	/// is only accessible if you are the administrator, but :shrug emoji:
	followed: 			Optional<bool>,
	/// The amount of followers this blog has.
	followers: 			Optional<u32>,
	/// Whether or not this blog is blocked from the primary account.
	is_blocked_from_primary: 		Optional<bool>,
	/// The amount of messages (unread and read) that this blog has.
	messages: 			Optional<u32>,
	/// Whether or not this is the user's primary blog.  This is generally
	/// unnecessary information as far as API usage goes, but it is important to 
	/// remember that when a user follows/likes/etc., it is the primary blog
	/// that will show as followed/liked/etc.
	primary: 			Optional<bool>,
	/// the number of posts this blog has in its queue.
	queue: 				Optional<u32>,
	/// Whether or not this blog also posts to twitter automatically.
	tweet: 				Optional<String>,					// TODO: change to enum
	/// Unknown.  Same as `tweet`?
	twitter_enabled: 	Optional<bool>,
	/// Unknown.  Same as `tweet`?
	twitter_send: 		Optional<bool>,
	/// The type of this blog.  Can be public or private.
	type: 				Optional<String>,					// TODO: change to enum
}