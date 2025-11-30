//! Twitter/X Automation Tools
//!
//! This module provides tools for Twitter/X automation including:
//! - Posting tweets
//! - Reading threads
//! - Searching tweets
//! - Getting timelines
//!
//! All tools require a valid Twitter session and implement rate limiting
//! to avoid account suspension.

mod error;
mod post;
mod rate_limiter;
mod read_thread;
mod search;
mod timeline;
mod types;

pub use error::TwitterError;
pub use post::{create_post_rate_limiter, twitter_post};
pub use rate_limiter::{RateLimitConfig, RateLimiter};
pub use read_thread::{create_read_rate_limiter, twitter_read_thread};
pub use search::{create_search_rate_limiter, twitter_search};
pub use timeline::{create_timeline_rate_limiter, twitter_get_timeline};
pub use types::*;

#[cfg(test)]
mod tests;
