# 🔧 SynMem MCP Tools Reference

This document provides a complete reference for all MCP (Model Context Protocol) tools exposed by SynMem.

## Overview

SynMem exposes browser automation capabilities through MCP tools that can be used by AI assistants like Claude. Tools are organized into four categories:

1. **Navigation** - Browser control and page navigation
2. **Scraping** - Content extraction and DOM manipulation
3. **Memory** - Semantic search and history management
4. **Automation** - Macros, form filling, and site-specific actions

---

## Navigation Tools

### `navigate_to`

Navigate the browser to a specified URL.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `url` | string | Yes | The URL to navigate to |
| `wait_for` | string | No | CSS selector to wait for before returning |
| `timeout_ms` | number | No | Maximum wait time in milliseconds (default: 30000) |

**Example:**

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://example.com",
    "wait_for": "#main-content",
    "timeout_ms": 10000
  }
}
```

**Returns:**

```json
{
  "success": true,
  "url": "https://example.com",
  "title": "Example Domain",
  "load_time_ms": 523
}
```

---

### `click`

Click on an element identified by selector or text content.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `selector` | string | No* | CSS selector of the element |
| `text` | string | No* | Text content to find and click |
| `wait_after_ms` | number | No | Time to wait after clicking (default: 500) |

*At least one of `selector` or `text` is required.

**Example:**

```json
{
  "tool": "click",
  "arguments": {
    "text": "Sign In",
    "wait_after_ms": 1000
  }
}
```

**Returns:**

```json
{
  "success": true,
  "element": {
    "tag": "button",
    "text": "Sign In"
  }
}
```

---

### `type_text`

Type text into an input field.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `selector` | string | Yes | CSS selector of the input element |
| `text` | string | Yes | Text to type |
| `clear_first` | boolean | No | Clear the field before typing (default: true) |
| `delay_ms` | number | No | Delay between keystrokes in ms (default: 50) |

**Example:**

```json
{
  "tool": "type_text",
  "arguments": {
    "selector": "#search-input",
    "text": "hello world",
    "delay_ms": 30
  }
}
```

---

### `scroll`

Scroll the page or a specific element.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `direction` | string | Yes | One of: "up", "down", "top", "bottom" |
| `amount` | number | No | Pixels to scroll (for up/down) |
| `selector` | string | No | Element to scroll within |

**Example:**

```json
{
  "tool": "scroll",
  "arguments": {
    "direction": "down",
    "amount": 500
  }
}
```

---

### `screenshot`

Capture a screenshot of the current page or element.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `selector` | string | No | Element to screenshot (full page if omitted) |
| `format` | string | No | "png" or "jpeg" (default: "png") |
| `full_page` | boolean | No | Capture full scrollable page (default: false) |

**Example:**

```json
{
  "tool": "screenshot",
  "arguments": {
    "full_page": true,
    "format": "png"
  }
}
```

**Returns:**

```json
{
  "success": true,
  "data": "base64-encoded-image-data",
  "format": "png",
  "width": 1920,
  "height": 3000
}
```

---

### `wait_for`

Wait for a condition to be met.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `selector` | string | No* | Wait for element to appear |
| `text` | string | No* | Wait for text to appear on page |
| `timeout_ms` | number | No | Maximum wait time (default: 30000) |
| `visible` | boolean | No | Wait for element to be visible (default: true) |

**Example:**

```json
{
  "tool": "wait_for",
  "arguments": {
    "selector": ".loading-spinner",
    "visible": false,
    "timeout_ms": 5000
  }
}
```

---

## Scraping Tools

### `scrape_page`

Extract structured content from the current page.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `selectors` | object | No | Custom selectors for extraction |
| `include_html` | boolean | No | Include raw HTML (default: false) |
| `include_links` | boolean | No | Extract all links (default: true) |
| `include_images` | boolean | No | Extract image URLs (default: false) |

**Example:**

```json
{
  "tool": "scrape_page",
  "arguments": {
    "selectors": {
      "title": "h1",
      "content": "article",
      "author": ".author-name"
    },
    "include_links": true
  }
}
```

**Returns:**

```json
{
  "success": true,
  "url": "https://example.com/article",
  "data": {
    "title": "Article Title",
    "content": "Full article text...",
    "author": "John Doe"
  },
  "links": [
    {"text": "Home", "href": "/"},
    {"text": "About", "href": "/about"}
  ],
  "metadata": {
    "scraped_at": "2024-01-15T10:30:00Z",
    "word_count": 1250
  }
}
```

---

### `scrape_chat`

Extract conversation from AI chat interfaces (ChatGPT, Claude, Gemini).

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `platform` | string | No | Auto-detected if not specified: "chatgpt", "claude", "gemini" |
| `include_timestamps` | boolean | No | Include message timestamps (default: true) |
| `max_messages` | number | No | Limit number of messages (default: all) |

**Example:**

```json
{
  "tool": "scrape_chat",
  "arguments": {
    "platform": "claude",
    "max_messages": 50
  }
}
```

**Returns:**

```json
{
  "success": true,
  "platform": "claude",
  "conversation": {
    "id": "conv-123",
    "title": "Help with Rust",
    "messages": [
      {
        "role": "user",
        "content": "How do I use async in Rust?",
        "timestamp": "2024-01-15T10:00:00Z"
      },
      {
        "role": "assistant",
        "content": "In Rust, async programming...",
        "timestamp": "2024-01-15T10:00:05Z"
      }
    ]
  }
}
```

---

### `extract_links`

Extract all links from the current page.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `selector` | string | No | Scope extraction to a container |
| `include_external` | boolean | No | Include external links (default: true) |
| `include_internal` | boolean | No | Include internal links (default: true) |
| `filter_pattern` | string | No | Regex pattern to filter URLs |

**Example:**

```json
{
  "tool": "extract_links",
  "arguments": {
    "selector": "nav",
    "include_external": false
  }
}
```

---

### `extract_text`

Extract clean text content from the page.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `selector` | string | No | Extract from specific element |
| `preserve_structure` | boolean | No | Keep paragraph breaks (default: true) |
| `max_length` | number | No | Truncate to character limit |

**Example:**

```json
{
  "tool": "extract_text",
  "arguments": {
    "selector": "article",
    "max_length": 5000
  }
}
```

---

### `get_dom`

Get a simplified DOM representation.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `selector` | string | No | Root element for extraction |
| `depth` | number | No | Maximum nesting depth (default: 5) |
| `include_attributes` | boolean | No | Include element attributes (default: true) |

**Example:**

```json
{
  "tool": "get_dom",
  "arguments": {
    "selector": "#main",
    "depth": 3
  }
}
```

---

## Memory Tools

### `search_memory`

Perform semantic search across browsing history.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `query` | string | Yes | Search query |
| `limit` | number | No | Maximum results (default: 10) |
| `date_from` | string | No | ISO date string for start date |
| `date_to` | string | No | ISO date string for end date |
| `domains` | array | No | Filter by domain list |

**Example:**

```json
{
  "tool": "search_memory",
  "arguments": {
    "query": "rust async programming tutorial",
    "limit": 5,
    "date_from": "2024-01-01"
  }
}
```

**Returns:**

```json
{
  "success": true,
  "results": [
    {
      "url": "https://rust-lang.org/async",
      "title": "Async Programming in Rust",
      "snippet": "...comprehensive guide to async/await...",
      "relevance_score": 0.92,
      "visited_at": "2024-01-10T14:30:00Z"
    }
  ]
}
```

---

### `get_recent`

Get recently visited pages or captured chats.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `type` | string | No | "pages", "chats", or "all" (default: "all") |
| `limit` | number | No | Number of results (default: 10) |
| `include_content` | boolean | No | Include full content (default: false) |

**Example:**

```json
{
  "tool": "get_recent",
  "arguments": {
    "type": "chats",
    "limit": 5,
    "include_content": true
  }
}
```

---

### `save_context`

Save the current browsing context for later reference.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | string | Yes | Context identifier |
| `description` | string | No | Human-readable description |
| `include_session` | boolean | No | Save session data (default: true) |

**Example:**

```json
{
  "tool": "save_context",
  "arguments": {
    "name": "research-rust-async",
    "description": "Resources for learning async Rust"
  }
}
```

---

### `list_sessions`

List all saved browsing sessions.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `sort_by` | string | No | "created", "modified", "name" (default: "modified") |
| `filter` | string | No | Filter by name pattern |

**Example:**

```json
{
  "tool": "list_sessions",
  "arguments": {
    "sort_by": "modified"
  }
}
```

---

## Automation Tools

### `record_macro`

Start recording a sequence of browser actions.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | string | Yes | Name for the macro |
| `description` | string | No | Description of what the macro does |

**Example:**

```json
{
  "tool": "record_macro",
  "arguments": {
    "name": "login-twitter",
    "description": "Log into Twitter account"
  }
}
```

---

### `play_macro`

Execute a previously recorded macro.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | string | Yes | Name of the macro to play |
| `variables` | object | No | Variables to substitute in the macro |
| `speed` | number | No | Playback speed multiplier (default: 1.0) |

**Example:**

```json
{
  "tool": "play_macro",
  "arguments": {
    "name": "login-twitter",
    "speed": 2.0
  }
}
```

---

### `twitter_post`

Post a tweet (requires authenticated Twitter session).

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | string | Yes | Tweet content (max 280 chars) |
| `reply_to` | string | No | Tweet ID to reply to |
| `media` | array | No | Media URLs to attach |

**Example:**

```json
{
  "tool": "twitter_post",
  "arguments": {
    "text": "Hello, Twitter! 👋"
  }
}
```

---

### `twitter_read_thread`

Read an entire Twitter thread.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `url` | string | Yes | URL of the thread |
| `include_replies` | boolean | No | Include reply tweets (default: false) |
| `max_tweets` | number | No | Maximum tweets to fetch (default: 100) |

**Example:**

```json
{
  "tool": "twitter_read_thread",
  "arguments": {
    "url": "https://twitter.com/user/status/123456789",
    "include_replies": true
  }
}
```

---

### `fill_form`

Fill out a form with provided data.

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `fields` | object | Yes | Map of field selectors to values |
| `submit` | boolean | No | Submit the form after filling (default: false) |
| `submit_selector` | string | No | Custom submit button selector |

**Example:**

```json
{
  "tool": "fill_form",
  "arguments": {
    "fields": {
      "#name": "John Doe",
      "#email": "john@example.com",
      "#message": "Hello, this is a test message."
    },
    "submit": true
  }
}
```

---

## Error Handling

All tools return errors in a consistent format:

```json
{
  "success": false,
  "error": {
    "code": "ELEMENT_NOT_FOUND",
    "message": "Could not find element matching selector: #nonexistent",
    "details": {
      "selector": "#nonexistent",
      "timeout_ms": 30000
    }
  }
}
```

### Common Error Codes

| Code | Description |
|------|-------------|
| `ELEMENT_NOT_FOUND` | Selector did not match any elements |
| `TIMEOUT` | Operation exceeded timeout |
| `NAVIGATION_FAILED` | Failed to navigate to URL |
| `SESSION_EXPIRED` | Browser session is no longer valid |
| `PERMISSION_DENIED` | Action not allowed on this page |
| `INVALID_PARAMETER` | Invalid or missing parameter |

---

## Best Practices

1. **Use `wait_for`** after navigation to ensure page is loaded
2. **Set appropriate timeouts** for slow-loading pages
3. **Use selectors** over text matching when possible for reliability
4. **Handle errors** gracefully in automation workflows
5. **Use semantic search** in memory tools for better results
