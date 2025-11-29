//! Twitter types and data structures

use serde::{Deserialize, Serialize};

/// Maximum length for a tweet
pub const TWEET_MAX_LENGTH: usize = 280;

/// Input parameters for posting a tweet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterPostInput {
    /// The text content of the tweet (max 280 characters)
    pub text: String,
    /// Optional media URLs to attach
    #[serde(default)]
    pub media_urls: Vec<String>,
    /// Optional tweet ID to reply to
    #[serde(default)]
    pub reply_to: Option<String>,
}

/// Result of posting a tweet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterPostResult {
    /// Whether the post was successful
    pub success: bool,
    /// The ID of the created tweet
    pub tweet_id: Option<String>,
    /// URL of the created tweet
    pub tweet_url: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
}

/// Input parameters for reading a thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterReadThreadInput {
    /// The URL or ID of the tweet to read the thread from
    pub tweet_url_or_id: String,
    /// Maximum number of tweets to retrieve
    #[serde(default = "default_max_tweets")]
    pub max_tweets: usize,
}

fn default_max_tweets() -> usize {
    100
}

/// A single tweet in a thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tweet {
    /// Tweet ID
    pub id: String,
    /// Tweet text content
    pub text: String,
    /// Author username
    pub author: String,
    /// Author display name
    pub author_display_name: String,
    /// Timestamp of the tweet
    pub timestamp: String,
    /// Number of likes
    pub likes: u64,
    /// Number of retweets
    pub retweets: u64,
    /// Number of replies
    pub replies: u64,
    /// Media attachments
    #[serde(default)]
    pub media: Vec<TweetMedia>,
}

/// Media attachment on a tweet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweetMedia {
    /// Type of media (image, video, gif)
    pub media_type: String,
    /// URL of the media
    pub url: String,
    /// Alt text if available
    pub alt_text: Option<String>,
}

/// Result of reading a thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterReadThreadResult {
    /// Whether the read was successful
    pub success: bool,
    /// The tweets in the thread
    pub tweets: Vec<Tweet>,
    /// Total number of tweets found
    pub total_count: usize,
    /// Error message if failed
    pub error: Option<String>,
}

/// Input parameters for searching tweets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterSearchInput {
    /// Search query
    pub query: String,
    /// Maximum number of results
    #[serde(default = "default_search_count")]
    pub count: usize,
    /// Filter by: latest, top, people, photos, videos
    #[serde(default)]
    pub filter: SearchFilter,
}

fn default_search_count() -> usize {
    20
}

/// Search result filter type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchFilter {
    /// Top tweets (default)
    #[default]
    Top,
    /// Latest tweets
    Latest,
    /// People matching query
    People,
    /// Tweets with photos
    Photos,
    /// Tweets with videos
    Videos,
}

/// Result of searching tweets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterSearchResult {
    /// Whether the search was successful
    pub success: bool,
    /// The tweets matching the query
    pub tweets: Vec<Tweet>,
    /// Cursor for pagination
    pub next_cursor: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
}

/// Input parameters for getting timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterGetTimelineInput {
    /// Type of timeline to retrieve
    #[serde(default)]
    pub timeline_type: TimelineType,
    /// Username for user timeline (required if timeline_type is User)
    pub username: Option<String>,
    /// Maximum number of tweets to retrieve
    #[serde(default = "default_timeline_count")]
    pub count: usize,
    /// Cursor for pagination
    pub cursor: Option<String>,
}

fn default_timeline_count() -> usize {
    20
}

/// Type of timeline to retrieve
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimelineType {
    /// Home timeline (for you)
    #[default]
    Home,
    /// User's own timeline
    User,
    /// Following timeline
    Following,
}

/// Result of getting timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterGetTimelineResult {
    /// Whether the retrieval was successful
    pub success: bool,
    /// The tweets in the timeline
    pub tweets: Vec<Tweet>,
    /// Cursor for next page
    pub next_cursor: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
}

/// Twitter session information
///
/// Contains sensitive authentication data. The Debug implementation
/// redacts sensitive fields to prevent accidental logging of credentials.
#[derive(Clone)]
pub struct TwitterSession {
    /// Session cookies
    pub cookies: String,
    /// CSRF token
    pub csrf_token: String,
    /// Bearer token
    pub bearer_token: String,
    /// Authenticated user ID
    pub user_id: Option<String>,
}

impl std::fmt::Debug for TwitterSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwitterSession")
            .field("cookies", &"[REDACTED]")
            .field("csrf_token", &"[REDACTED]")
            .field("bearer_token", &"[REDACTED]")
            .field("user_id", &self.user_id)
            .finish()
    }
}
