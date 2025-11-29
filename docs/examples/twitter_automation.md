# 🐦 Twitter Automation with SynMem

This guide shows you how to use SynMem to automate Twitter (X) interactions using your authenticated session.

## Prerequisites

1. **Authenticated Twitter session** - You must be logged into Twitter in Chrome
2. **SynMem extension installed** - With permission for twitter.com
3. **Claude Desktop configured** - With SynMem MCP server

---

## Reading Tweets

### Read a Single Tweet

```
User: Read the tweet at https://twitter.com/user/status/123456789
```

SynMem will extract:
- Tweet content
- Author information
- Engagement metrics
- Media attachments

**MCP Tool:**

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://twitter.com/user/status/123456789",
    "wait_for": "[data-testid='tweet']"
  }
}

{
  "tool": "scrape_page",
  "arguments": {
    "selectors": {
      "content": "[data-testid='tweetText']",
      "author": "[data-testid='User-Name']",
      "time": "time",
      "likes": "[data-testid='like'] span",
      "retweets": "[data-testid='retweet'] span",
      "replies": "[data-testid='reply'] span"
    }
  }
}
```

**Result:**

```json
{
  "success": true,
  "data": {
    "content": "Just released v2.0 of my project! Check it out...",
    "author": "@developer",
    "time": "2024-01-15T14:30:00Z",
    "likes": "1.2K",
    "retweets": "342",
    "replies": "89"
  }
}
```

---

### Read a Thread

```
User: Read the entire thread starting from this tweet
```

**MCP Tool:**

```json
{
  "tool": "twitter_read_thread",
  "arguments": {
    "url": "https://twitter.com/user/status/123456789",
    "include_replies": false,
    "max_tweets": 50
  }
}
```

**Result:**

```json
{
  "success": true,
  "thread": {
    "author": {
      "handle": "@developer",
      "displayName": "Developer Name",
      "verified": true
    },
    "tweets": [
      {
        "id": "123456789",
        "content": "1/ Here's a thread about building with Rust...",
        "timestamp": "2024-01-15T14:30:00Z"
      },
      {
        "id": "123456790",
        "content": "2/ First, let's talk about memory safety...",
        "timestamp": "2024-01-15T14:31:00Z"
      },
      {
        "id": "123456791",
        "content": "3/ The ownership system ensures...",
        "timestamp": "2024-01-15T14:32:00Z"
      }
    ],
    "total_tweets": 15
  }
}
```

---

### Read Timeline

```
User: Get the latest 10 tweets from my timeline
```

**MCP Tool:**

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://twitter.com/home",
    "wait_for": "[data-testid='tweet']"
  }
}

{
  "tool": "scrape_page",
  "arguments": {
    "selectors": {
      "tweets": "[data-testid='tweet']",
      "authors": "[data-testid='User-Name']",
      "contents": "[data-testid='tweetText']"
    }
  }
}
```

---

## Posting Tweets

### Simple Tweet

```
User: Post a tweet saying "Hello, Twitter! 👋"
```

**MCP Tool:**

```json
{
  "tool": "twitter_post",
  "arguments": {
    "text": "Hello, Twitter! 👋"
  }
}
```

**Result:**

```json
{
  "success": true,
  "tweet_id": "123456792",
  "url": "https://twitter.com/you/status/123456792"
}
```

### Reply to a Tweet

```
User: Reply to that tweet with "Thanks for sharing!"
```

**MCP Tool:**

```json
{
  "tool": "twitter_post",
  "arguments": {
    "text": "Thanks for sharing!",
    "reply_to": "123456789"
  }
}
```

### Tweet with Media

```
User: Post a tweet with this image: [image URL]
```

**MCP Tool:**

```json
{
  "tool": "twitter_post",
  "arguments": {
    "text": "Check out this chart! 📊",
    "media": ["https://example.com/chart.png"]
  }
}
```

---

## Engagement Actions

### Like a Tweet

```
User: Like the tweet at https://twitter.com/user/status/123456789
```

**MCP Tool sequence:**

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://twitter.com/user/status/123456789",
    "wait_for": "[data-testid='like']"
  }
}

{
  "tool": "click",
  "arguments": {
    "selector": "[data-testid='like']"
  }
}
```

### Retweet

```
User: Retweet this tweet
```

```json
{
  "tool": "click",
  "arguments": {
    "selector": "[data-testid='retweet']"
  }
}

{
  "tool": "click",
  "arguments": {
    "text": "Repost"
  }
}
```

### Quote Tweet

```
User: Quote tweet this with my thoughts
```

```json
{
  "tool": "click",
  "arguments": {
    "selector": "[data-testid='retweet']"
  }
}

{
  "tool": "click",
  "arguments": {
    "text": "Quote"
  }
}

{
  "tool": "type_text",
  "arguments": {
    "selector": "[data-testid='tweetTextarea_0']",
    "text": "This is a great point! Here's my take..."
  }
}

{
  "tool": "click",
  "arguments": {
    "text": "Post"
  }
}
```

---

## Profile Actions

### View Profile

```
User: Show me the profile for @developer
```

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://twitter.com/developer",
    "wait_for": "[data-testid='UserName']"
  }
}

{
  "tool": "scrape_page",
  "arguments": {
    "selectors": {
      "name": "[data-testid='UserName']",
      "bio": "[data-testid='UserDescription']",
      "followers": "a[href$='/followers'] span",
      "following": "a[href$='/following'] span",
      "location": "[data-testid='UserLocation']"
    }
  }
}
```

### Follow/Unfollow

```
User: Follow @developer
```

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://twitter.com/developer"
  }
}

{
  "tool": "click",
  "arguments": {
    "selector": "[data-testid='follow']"
  }
}
```

---

## Search

### Search Tweets

```
User: Search for tweets about "rust programming"
```

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://twitter.com/search?q=rust%20programming&src=typed_query&f=live",
    "wait_for": "[data-testid='tweet']"
  }
}

{
  "tool": "scrape_page",
  "arguments": {
    "selectors": {
      "tweets": "[data-testid='tweet']",
      "authors": "[data-testid='User-Name']",
      "contents": "[data-testid='tweetText']"
    }
  }
}
```

### Advanced Search

```
User: Find tweets from @developer about "async" from last week
```

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://twitter.com/search?q=async%20from%3Adeveloper%20since%3A2024-01-08%20until%3A2024-01-15&src=typed_query",
    "wait_for": "[data-testid='tweet']"
  }
}
```

---

## Automation Macros

### Record a Workflow

```
User: Record a macro for posting my daily status update
```

```json
{
  "tool": "record_macro",
  "arguments": {
    "name": "daily-status",
    "description": "Post daily status update tweet"
  }
}
```

Then perform the actions manually. SynMem will record:
1. Navigate to compose
2. Type the status
3. Click post

### Replay a Workflow

```
User: Post my daily status: "Working on documentation today 📚"
```

```json
{
  "tool": "play_macro",
  "arguments": {
    "name": "daily-status",
    "variables": {
      "status_text": "Working on documentation today 📚"
    }
  }
}
```

---

## Monitoring

### Track Mentions

```
User: Check my mentions from the last hour
```

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://twitter.com/notifications/mentions",
    "wait_for": "[data-testid='tweet']"
  }
}

{
  "tool": "scrape_page",
  "arguments": {
    "selectors": {
      "mentions": "[data-testid='tweet']",
      "authors": "[data-testid='User-Name']",
      "contents": "[data-testid='tweetText']",
      "times": "time"
    }
  }
}
```

### Save Search Results

```
User: Save these search results for later
```

```json
{
  "tool": "save_context",
  "arguments": {
    "name": "twitter-search-rust-2024-01-15",
    "description": "Twitter search results for rust programming"
  }
}
```

---

## Best Practices

### 1. Rate Limiting

Twitter has rate limits. Add delays between actions:

```json
{
  "tool": "wait_for",
  "arguments": {
    "time_ms": 2000
  }
}
```

### 2. Handle Dynamic Content

Twitter loads content dynamically. Always wait for elements:

```json
{
  "tool": "wait_for",
  "arguments": {
    "selector": "[data-testid='tweet']",
    "timeout_ms": 10000
  }
}
```

### 3. Scroll for More Content

```json
{
  "tool": "scroll",
  "arguments": {
    "direction": "down",
    "amount": 1000
  }
}

{
  "tool": "wait_for",
  "arguments": {
    "time_ms": 1500
  }
}
```

### 4. Verify Actions

After posting, verify the tweet was created:

```json
{
  "tool": "wait_for",
  "arguments": {
    "text": "Your post was sent",
    "timeout_ms": 5000
  }
}
```

---

## Error Handling

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| "Not logged in" | Session expired | Log into Twitter in Chrome |
| "Rate limited" | Too many requests | Wait and retry |
| "Tweet too long" | Over 280 characters | Shorten the text |
| "Element not found" | UI changed | Update selector |

### Handling Failures

```json
{
  "success": false,
  "error": {
    "code": "RATE_LIMITED",
    "message": "Twitter rate limit exceeded",
    "retry_after": 900
  }
}
```

---

## Security Notes

1. **Never share your session** - Your cookies grant full access to your account
2. **Review before posting** - Always confirm content before automated posting
3. **Monitor activity** - Check your account for unexpected activity
4. **Use responsibly** - Respect Twitter's ToS and other users

---

## Next Steps

- [Basic Scraping](basic_scraping.md) - General web scraping
- [Chat Capture](chat_capture.md) - Capture AI conversations
- [MCP Tools Reference](../MCP_TOOLS.md) - Complete tool documentation
