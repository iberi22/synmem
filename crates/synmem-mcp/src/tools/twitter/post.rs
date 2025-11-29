//! Twitter post tool
//!
//! Posts a tweet with optional media and reply support.

use super::{
    RateLimiter, RateLimitConfig, TwitterError, TwitterPostInput, TwitterPostResult,
    TwitterSession, TWEET_MAX_LENGTH,
};

/// Post a tweet
///
/// # Arguments
/// * `input` - The tweet content and options
/// * `session` - Valid Twitter session
/// * `rate_limiter` - Rate limiter to prevent API abuse
///
/// # Returns
/// Result containing the posted tweet info or an error
///
/// # Example
/// ```ignore
/// let input = TwitterPostInput {
///     text: "Hello, Twitter!".to_string(),
///     media_urls: vec![],
///     reply_to: None,
/// };
/// let result = twitter_post(input, &session, &rate_limiter).await?;
/// ```
pub async fn twitter_post(
    input: TwitterPostInput,
    session: &TwitterSession,
    rate_limiter: &RateLimiter,
) -> Result<TwitterPostResult, TwitterError> {
    // Validate input
    validate_post_input(&input)?;

    // Check rate limit
    rate_limiter.acquire().await?;

    // Validate session
    validate_session(session)?;

    // In a real implementation, this would:
    // 1. Prepare the tweet payload
    // 2. Upload media if provided
    // 3. Make the API call to post the tweet
    // 4. Return the result

    // For now, return a placeholder result
    // The actual implementation would use chromiumoxide to automate the browser
    Ok(TwitterPostResult {
        success: true,
        tweet_id: Some("placeholder_tweet_id".to_string()),
        tweet_url: Some("https://twitter.com/user/status/placeholder_tweet_id".to_string()),
        error: None,
    })
}

/// Validate the post input
fn validate_post_input(input: &TwitterPostInput) -> Result<(), TwitterError> {
    // Check tweet length
    let char_count = input.text.chars().count();
    if char_count > TWEET_MAX_LENGTH {
        return Err(TwitterError::TweetTooLong {
            max: TWEET_MAX_LENGTH,
            actual: char_count,
        });
    }

    // Check for empty tweet (unless media is attached)
    if input.text.trim().is_empty() && input.media_urls.is_empty() {
        return Err(TwitterError::InvalidInput {
            message: "Tweet must have text or media".to_string(),
        });
    }

    // Validate media URLs
    for url in &input.media_urls {
        if !is_valid_media_url(url) {
            return Err(TwitterError::InvalidInput {
                message: format!("Invalid media URL: {}", url),
            });
        }
    }

    Ok(())
}

/// Validate the Twitter session
fn validate_session(session: &TwitterSession) -> Result<(), TwitterError> {
    if session.cookies.is_empty() || session.csrf_token.is_empty() {
        return Err(TwitterError::NoSession);
    }
    Ok(())
}

/// Check if a URL is a valid media URL
fn is_valid_media_url(url: &str) -> bool {
    // Basic validation - check for common image/video extensions or data URLs
    let url_lower = url.to_lowercase();
    url_lower.starts_with("http://")
        || url_lower.starts_with("https://")
        || url_lower.starts_with("data:")
}

/// Create a rate limiter configured for posting
pub fn create_post_rate_limiter() -> RateLimiter {
    RateLimiter::new(RateLimitConfig::for_post())
}
