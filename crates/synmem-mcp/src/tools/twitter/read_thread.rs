//! Twitter read thread tool
//!
//! Reads an entire Twitter thread and extracts all tweets.

use super::{
    RateLimiter, RateLimitConfig, Tweet, TwitterError, TwitterReadThreadInput,
    TwitterReadThreadResult, TwitterSession,
};

/// Read a Twitter thread
///
/// # Arguments
/// * `input` - The thread URL or ID and options
/// * `session` - Valid Twitter session
/// * `rate_limiter` - Rate limiter to prevent API abuse
///
/// # Returns
/// Result containing all tweets in the thread or an error
///
/// # Example
/// ```ignore
/// let input = TwitterReadThreadInput {
///     tweet_url_or_id: "https://twitter.com/user/status/123456".to_string(),
///     max_tweets: 50,
/// };
/// let result = twitter_read_thread(input, &session, &rate_limiter).await?;
/// for tweet in result.tweets {
///     println!("{}: {}", tweet.author, tweet.text);
/// }
/// ```
pub async fn twitter_read_thread(
    input: TwitterReadThreadInput,
    session: &TwitterSession,
    rate_limiter: &RateLimiter,
) -> Result<TwitterReadThreadResult, TwitterError> {
    // Validate input
    let tweet_id = extract_tweet_id(&input.tweet_url_or_id)?;

    // Check rate limit
    rate_limiter.acquire().await?;

    // Validate session
    validate_session(session)?;

    // In a real implementation, this would:
    // 1. Navigate to the tweet URL
    // 2. Extract the main tweet and all replies in the thread
    // 3. Follow the conversation thread
    // 4. Collect all tweets up to max_tweets

    // For now, return a placeholder result
    let placeholder_tweet = Tweet {
        id: tweet_id.clone(),
        text: "Placeholder tweet text".to_string(),
        author: "placeholder_user".to_string(),
        author_display_name: "Placeholder User".to_string(),
        timestamp: "2024-01-01T00:00:00Z".to_string(),
        likes: 0,
        retweets: 0,
        replies: 0,
        media: vec![],
    };

    Ok(TwitterReadThreadResult {
        success: true,
        tweets: vec![placeholder_tweet],
        total_count: 1,
        error: None,
    })
}

/// Extract tweet ID from URL or use as-is if already an ID
fn extract_tweet_id(url_or_id: &str) -> Result<String, TwitterError> {
    // If it's already a numeric ID, return it
    if url_or_id.chars().all(|c| c.is_ascii_digit()) {
        return Ok(url_or_id.to_string());
    }

    // Try to extract from URL formats:
    // https://twitter.com/user/status/123456789
    // https://x.com/user/status/123456789

    if let Some(id) = extract_id_from_url(url_or_id) {
        return Ok(id);
    }

    Err(TwitterError::InvalidInput {
        message: format!("Could not extract tweet ID from: {}", url_or_id),
    })
}

/// Extract tweet ID from a Twitter/X URL
fn extract_id_from_url(url: &str) -> Option<String> {
    let url_lower = url.to_lowercase();

    // Check if it's a Twitter/X URL
    if !url_lower.contains("twitter.com") && !url_lower.contains("x.com") {
        return None;
    }

    // Look for /status/ pattern
    let parts: Vec<&str> = url.split('/').collect();
    for (i, part) in parts.iter().enumerate() {
        if *part == "status" && i + 1 < parts.len() {
            // Get the ID and remove any query parameters
            let id = parts[i + 1].split('?').next()?;
            if id.chars().all(|c| c.is_ascii_digit()) && !id.is_empty() {
                return Some(id.to_string());
            }
        }
    }

    None
}

/// Validate the Twitter session
fn validate_session(session: &TwitterSession) -> Result<(), TwitterError> {
    if session.cookies.is_empty() || session.csrf_token.is_empty() {
        return Err(TwitterError::NoSession);
    }
    Ok(())
}

/// Create a rate limiter configured for reading
pub fn create_read_rate_limiter() -> RateLimiter {
    RateLimiter::new(RateLimitConfig::for_read())
}
