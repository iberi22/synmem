//! Tests for Twitter automation tools

use super::*;

/// Create a mock Twitter session for testing
fn mock_session() -> TwitterSession {
    TwitterSession {
        cookies: "auth_token=mock_token; ct0=mock_csrf".to_string(),
        csrf_token: "mock_csrf_token".to_string(),
        bearer_token: "mock_bearer".to_string(),
        user_id: Some("123456789".to_string()),
    }
}

/// Create an invalid session for testing error cases
fn invalid_session() -> TwitterSession {
    TwitterSession {
        cookies: String::new(),
        csrf_token: String::new(),
        bearer_token: String::new(),
        user_id: None,
    }
}

mod post_tests {
    use super::*;

    #[tokio::test]
    async fn test_post_valid_tweet() {
        let input = TwitterPostInput {
            text: "Hello, Twitter!".to_string(),
            media_urls: vec![],
            reply_to: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_post());

        let result = twitter_post(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert!(result.tweet_id.is_some());
    }

    #[tokio::test]
    async fn test_post_tweet_too_long() {
        let long_text = "a".repeat(281);
        let input = TwitterPostInput {
            text: long_text,
            media_urls: vec![],
            reply_to: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_post());

        let result = twitter_post(input, &session, &rate_limiter).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            TwitterError::TweetTooLong { max, actual } => {
                assert_eq!(max, 280);
                assert_eq!(actual, 281);
            }
            _ => panic!("Expected TweetTooLong error"),
        }
    }

    #[tokio::test]
    async fn test_post_empty_tweet_no_media() {
        let input = TwitterPostInput {
            text: "   ".to_string(),
            media_urls: vec![],
            reply_to: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_post());

        let result = twitter_post(input, &session, &rate_limiter).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            TwitterError::InvalidInput { message } => {
                assert!(message.contains("text or media"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[tokio::test]
    async fn test_post_no_session() {
        let input = TwitterPostInput {
            text: "Hello!".to_string(),
            media_urls: vec![],
            reply_to: None,
        };
        let session = invalid_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_post());

        let result = twitter_post(input, &session, &rate_limiter).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TwitterError::NoSession));
    }

    #[tokio::test]
    async fn test_post_with_media() {
        let input = TwitterPostInput {
            text: "Check out this image!".to_string(),
            media_urls: vec!["https://example.com/image.jpg".to_string()],
            reply_to: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_post());

        let result = twitter_post(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_reply() {
        let input = TwitterPostInput {
            text: "This is a reply".to_string(),
            media_urls: vec![],
            reply_to: Some("123456789".to_string()),
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_post());

        let result = twitter_post(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_max_length_tweet() {
        let text = "a".repeat(280);
        let input = TwitterPostInput {
            text,
            media_urls: vec![],
            reply_to: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_post());

        let result = twitter_post(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
    }
}

mod read_thread_tests {
    use super::*;

    #[tokio::test]
    async fn test_read_thread_by_url() {
        let input = TwitterReadThreadInput {
            tweet_url_or_id: "https://twitter.com/user/status/123456789".to_string(),
            max_tweets: 50,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_read_thread(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_read_thread_by_id() {
        let input = TwitterReadThreadInput {
            tweet_url_or_id: "123456789".to_string(),
            max_tweets: 50,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_read_thread(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_read_thread_x_url() {
        let input = TwitterReadThreadInput {
            tweet_url_or_id: "https://x.com/user/status/123456789".to_string(),
            max_tweets: 50,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_read_thread(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_read_thread_invalid_url() {
        let input = TwitterReadThreadInput {
            tweet_url_or_id: "https://example.com/not-twitter".to_string(),
            max_tweets: 50,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_read_thread(input, &session, &rate_limiter).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TwitterError::InvalidInput { .. }
        ));
    }

    #[tokio::test]
    async fn test_read_thread_no_session() {
        let input = TwitterReadThreadInput {
            tweet_url_or_id: "123456789".to_string(),
            max_tweets: 50,
        };
        let session = invalid_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_read_thread(input, &session, &rate_limiter).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TwitterError::NoSession));
    }
}

mod search_tests {
    use super::*;

    #[tokio::test]
    async fn test_search_valid_query() {
        let input = TwitterSearchInput {
            query: "rust programming".to_string(),
            count: 20,
            filter: SearchFilter::Top,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_search());

        let result = twitter_search(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_search_empty_query() {
        let input = TwitterSearchInput {
            query: "".to_string(),
            count: 20,
            filter: SearchFilter::Top,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_search());

        let result = twitter_search(input, &session, &rate_limiter).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TwitterError::InvalidInput { .. }
        ));
    }

    #[tokio::test]
    async fn test_search_zero_count() {
        let input = TwitterSearchInput {
            query: "test".to_string(),
            count: 0,
            filter: SearchFilter::Top,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_search());

        let result = twitter_search(input, &session, &rate_limiter).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_count_too_high() {
        let input = TwitterSearchInput {
            query: "test".to_string(),
            count: 101,
            filter: SearchFilter::Top,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_search());

        let result = twitter_search(input, &session, &rate_limiter).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_different_filters() {
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_search());

        for filter in [
            SearchFilter::Top,
            SearchFilter::Latest,
            SearchFilter::People,
            SearchFilter::Photos,
            SearchFilter::Videos,
        ] {
            let input = TwitterSearchInput {
                query: "test".to_string(),
                count: 10,
                filter,
            };
            let result = twitter_search(input, &session, &rate_limiter).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_search_no_session() {
        let input = TwitterSearchInput {
            query: "test".to_string(),
            count: 20,
            filter: SearchFilter::Top,
        };
        let session = invalid_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_search());

        let result = twitter_search(input, &session, &rate_limiter).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TwitterError::NoSession));
    }
}

mod timeline_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_home_timeline() {
        let input = TwitterGetTimelineInput {
            timeline_type: TimelineType::Home,
            username: None,
            count: 20,
            cursor: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_get_timeline(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_get_following_timeline() {
        let input = TwitterGetTimelineInput {
            timeline_type: TimelineType::Following,
            username: None,
            count: 20,
            cursor: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_get_timeline(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_timeline() {
        let input = TwitterGetTimelineInput {
            timeline_type: TimelineType::User,
            username: Some("testuser".to_string()),
            count: 20,
            cursor: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_get_timeline(input, &session, &rate_limiter).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_timeline_no_username() {
        let input = TwitterGetTimelineInput {
            timeline_type: TimelineType::User,
            username: None,
            count: 20,
            cursor: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_get_timeline(input, &session, &rate_limiter).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TwitterError::InvalidInput { .. }
        ));
    }

    #[tokio::test]
    async fn test_get_timeline_invalid_username() {
        let input = TwitterGetTimelineInput {
            timeline_type: TimelineType::User,
            username: Some("invalid username with spaces".to_string()),
            count: 20,
            cursor: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_get_timeline(input, &session, &rate_limiter).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_timeline_zero_count() {
        let input = TwitterGetTimelineInput {
            timeline_type: TimelineType::Home,
            username: None,
            count: 0,
            cursor: None,
        };
        let session = mock_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_get_timeline(input, &session, &rate_limiter).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_timeline_no_session() {
        let input = TwitterGetTimelineInput {
            timeline_type: TimelineType::Home,
            username: None,
            count: 20,
            cursor: None,
        };
        let session = invalid_session();
        let rate_limiter = RateLimiter::new(RateLimitConfig::for_read());

        let result = twitter_get_timeline(input, &session, &rate_limiter).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TwitterError::NoSession));
    }
}

mod rate_limiter_tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_acquire() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 5,
            window_seconds: 60,
            min_delay_ms: 0,
        });

        for _ in 0..5 {
            let result = limiter.acquire().await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_exhausted() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 2,
            window_seconds: 60,
            min_delay_ms: 0,
        });

        // Exhaust the rate limit
        assert!(limiter.acquire().await.is_ok());
        assert!(limiter.acquire().await.is_ok());

        // Should fail now
        let result = limiter.acquire().await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TwitterError::RateLimited { .. }
        ));
    }

    #[test]
    fn test_rate_limiter_remaining_tokens() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 10,
            window_seconds: 60,
            min_delay_ms: 0,
        });

        assert_eq!(limiter.remaining_tokens(), 10);
    }

    #[test]
    fn test_rate_limiter_can_proceed() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 10,
            window_seconds: 60,
            min_delay_ms: 0,
        });

        assert!(limiter.can_proceed());
    }

    #[test]
    fn test_rate_limit_configs() {
        let post_config = RateLimitConfig::for_post();
        let read_config = RateLimitConfig::for_read();
        let search_config = RateLimitConfig::for_search();

        // Post should be most restrictive
        assert!(post_config.max_requests < read_config.max_requests);
        assert!(post_config.min_delay_ms > read_config.min_delay_ms);

        // Search should be between post and read
        assert!(search_config.max_requests <= read_config.max_requests);
        assert!(search_config.max_requests >= post_config.max_requests);
    }
}

mod error_tests {
    use super::*;

    #[test]
    fn test_error_is_recoverable() {
        assert!(TwitterError::RateLimited { wait_seconds: 60 }.is_recoverable());
        assert!(TwitterError::NetworkError {
            message: "timeout".to_string()
        }
        .is_recoverable());
        assert!(!TwitterError::NoSession.is_recoverable());
        assert!(!TwitterError::InvalidInput {
            message: "test".to_string()
        }
        .is_recoverable());
    }

    #[test]
    fn test_error_retry_after() {
        assert_eq!(
            TwitterError::RateLimited { wait_seconds: 60 }.retry_after(),
            Some(60)
        );
        assert_eq!(
            TwitterError::NetworkError {
                message: "timeout".to_string()
            }
            .retry_after(),
            Some(5)
        );
        assert_eq!(TwitterError::NoSession.retry_after(), None);
    }

    #[test]
    fn test_error_display() {
        let error = TwitterError::TweetTooLong {
            max: 280,
            actual: 300,
        };
        assert!(error.to_string().contains("280"));
        assert!(error.to_string().contains("300"));

        let error = TwitterError::NoSession;
        assert!(error.to_string().contains("session"));
    }
}

mod types_tests {
    use super::*;

    #[test]
    fn test_tweet_max_length() {
        assert_eq!(TWEET_MAX_LENGTH, 280);
    }

    #[test]
    fn test_search_filter_default() {
        let filter: SearchFilter = Default::default();
        assert!(matches!(filter, SearchFilter::Top));
    }

    #[test]
    fn test_timeline_type_default() {
        let timeline_type: TimelineType = Default::default();
        assert!(matches!(timeline_type, TimelineType::Home));
    }

    #[test]
    fn test_twitter_post_input_serialization() {
        let input = TwitterPostInput {
            text: "Hello, world!".to_string(),
            media_urls: vec!["https://example.com/image.jpg".to_string()],
            reply_to: Some("123".to_string()),
        };

        let json = serde_json::to_string(&input).unwrap();
        let deserialized: TwitterPostInput = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.text, input.text);
        assert_eq!(deserialized.media_urls, input.media_urls);
        assert_eq!(deserialized.reply_to, input.reply_to);
    }

    #[test]
    fn test_tweet_serialization() {
        let tweet = Tweet {
            id: "123456789".to_string(),
            text: "Test tweet".to_string(),
            author: "testuser".to_string(),
            author_display_name: "Test User".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            likes: 100,
            retweets: 50,
            replies: 25,
            media: vec![],
        };

        let json = serde_json::to_string(&tweet).unwrap();
        let deserialized: Tweet = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, tweet.id);
        assert_eq!(deserialized.text, tweet.text);
        assert_eq!(deserialized.likes, tweet.likes);
    }

    #[test]
    fn test_twitter_session_debug_redacts_sensitive_data() {
        let session = TwitterSession {
            cookies: "super_secret_cookie".to_string(),
            csrf_token: "secret_csrf_token".to_string(),
            bearer_token: "secret_bearer_token".to_string(),
            user_id: Some("123456".to_string()),
        };

        let debug_output = format!("{:?}", session);

        // Ensure sensitive data is redacted
        assert!(!debug_output.contains("super_secret_cookie"));
        assert!(!debug_output.contains("secret_csrf_token"));
        assert!(!debug_output.contains("secret_bearer_token"));

        // Ensure user_id is still visible (not sensitive)
        assert!(debug_output.contains("123456"));

        // Ensure REDACTED placeholder is present
        assert!(debug_output.contains("[REDACTED]"));
    }
}
