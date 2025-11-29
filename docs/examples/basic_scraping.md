# 🌐 Basic Web Scraping with SynMem

This guide shows you how to use SynMem's MCP tools for basic web scraping tasks.

## Overview

SynMem provides powerful tools for extracting content from web pages. You can:
- Navigate to any URL
- Extract text, links, and images
- Use custom selectors for structured data
- Store scraped content for later search

---

## Simple Page Scrape

### Navigate and Scrape

The most basic workflow is to navigate to a page and scrape its content:

```
User: Go to https://news.ycombinator.com and get me the top stories
```

SynMem will:
1. Use `navigate_to` to load the page
2. Use `scrape_page` to extract content
3. Return structured data

**Behind the scenes (MCP tools):**

```json
// Step 1: Navigate
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://news.ycombinator.com",
    "wait_for": ".athing"
  }
}

// Step 2: Scrape with custom selectors
{
  "tool": "scrape_page",
  "arguments": {
    "selectors": {
      "stories": ".athing .titleline > a",
      "scores": ".score"
    }
  }
}
```

**Result:**

```json
{
  "success": true,
  "data": {
    "stories": [
      "Show HN: I built a thing",
      "Ask HN: What are you working on?",
      "PostgreSQL 17 Released"
    ],
    "scores": [
      "324 points",
      "156 points",
      "892 points"
    ]
  }
}
```

---

## Extracting Specific Elements

### Using CSS Selectors

Extract specific elements using CSS selectors:

```
User: Get all links from the navigation menu on example.com
```

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://example.com"
  }
}

{
  "tool": "extract_links",
  "arguments": {
    "selector": "nav",
    "include_external": false
  }
}
```

### Extracting Text Content

Get clean text from an article:

```
User: Extract the main article text from this blog post
```

```json
{
  "tool": "extract_text",
  "arguments": {
    "selector": "article",
    "preserve_structure": true,
    "max_length": 10000
  }
}
```

**Result:**

```json
{
  "success": true,
  "text": "Introduction\n\nThis article covers...\n\nSection 1: Background\n\nThe history of...",
  "word_count": 1547
}
```

---

## Working with Dynamic Content

### Waiting for Elements

Many modern websites load content dynamically. Use `wait_for` to ensure content is loaded:

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://app.example.com/dashboard",
    "wait_for": ".dashboard-loaded",
    "timeout_ms": 10000
  }
}
```

### Scrolling to Load More

For infinite scroll pages:

```
User: Get all products from this page, scrolling to load more
```

```json
// Navigate first
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://shop.example.com/products"
  }
}

// Scroll down multiple times
{
  "tool": "scroll",
  "arguments": {
    "direction": "down",
    "amount": 1000
  }
}

// Wait for new content
{
  "tool": "wait_for",
  "arguments": {
    "selector": ".product-card:nth-child(20)",
    "timeout_ms": 5000
  }
}

// Repeat scroll if needed...

// Finally scrape
{
  "tool": "scrape_page",
  "arguments": {
    "selectors": {
      "products": ".product-card",
      "prices": ".product-price"
    }
  }
}
```

---

## Capturing Screenshots

### Full Page Screenshot

```
User: Take a screenshot of the entire page
```

```json
{
  "tool": "screenshot",
  "arguments": {
    "full_page": true,
    "format": "png"
  }
}
```

### Element Screenshot

```
User: Screenshot just the chart on this page
```

```json
{
  "tool": "screenshot",
  "arguments": {
    "selector": "#main-chart",
    "format": "png"
  }
}
```

---

## Saving to Memory

### Automatic Storage

All scraped pages are automatically stored in SynMem's memory for later search:

```
User: Save this article for later reference
```

```json
{
  "tool": "save_context",
  "arguments": {
    "name": "rust-async-article",
    "description": "Comprehensive guide to async programming in Rust"
  }
}
```

### Searching Saved Content

Later, you can search across all saved content:

```
User: Find the article I saved about Rust async
```

```json
{
  "tool": "search_memory",
  "arguments": {
    "query": "rust async programming await",
    "limit": 5
  }
}
```

---

## Multi-Page Scraping

### Following Links

Scrape multiple pages by following links:

```
User: Get the content from the first 5 articles on this news site
```

```json
// First, navigate and get links
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://news.example.com"
  }
}

{
  "tool": "extract_links",
  "arguments": {
    "selector": ".article-list",
    "filter_pattern": "^/article/"
  }
}

// For each link, navigate and scrape
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://news.example.com/article/123"
  }
}

{
  "tool": "scrape_page",
  "arguments": {
    "selectors": {
      "title": "h1",
      "content": ".article-body",
      "author": ".author-name",
      "date": ".publish-date"
    }
  }
}

// Repeat for remaining articles...
```

---

## Error Handling

### Handling Timeouts

```json
{
  "tool": "navigate_to",
  "arguments": {
    "url": "https://slow-site.example.com",
    "timeout_ms": 60000
  }
}
```

If the operation times out:

```json
{
  "success": false,
  "error": {
    "code": "TIMEOUT",
    "message": "Navigation timeout after 60000ms"
  }
}
```

### Handling Missing Elements

If a selector doesn't match:

```json
{
  "success": true,
  "data": {
    "title": "Found Title",
    "author": null  // Element not found
  },
  "warnings": [
    "Selector '.author-name' returned no matches"
  ]
}
```

---

## Best Practices

### 1. Use Specific Selectors

```json
// Good - specific selector
"selector": "#main-content article h1"

// Avoid - too broad
"selector": "h1"
```

### 2. Handle Rate Limiting

Add delays between requests:

```json
{
  "tool": "wait_for",
  "arguments": {
    "time_ms": 1000
  }
}
```

### 3. Respect robots.txt

Check the site's robots.txt before scraping automated content.

### 4. Use Appropriate Timeouts

- Fast sites: 5-10 seconds
- Dynamic apps: 15-30 seconds
- Slow/heavy sites: 30-60 seconds

---

## Complete Example: News Aggregator

Here's a complete example of building a simple news aggregator:

```
User: Get the top 3 stories from Hacker News with their comments count
```

**SynMem workflow:**

1. Navigate to Hacker News:
   ```json
   {
     "tool": "navigate_to",
     "arguments": {
       "url": "https://news.ycombinator.com",
       "wait_for": ".athing"
     }
   }
   ```

2. Scrape stories:
   ```json
   {
     "tool": "scrape_page",
     "arguments": {
       "selectors": {
         "titles": ".titleline > a",
         "links": ".titleline > a[href]",
         "comments": ".subtext a:last-child"
       }
     }
   }
   ```

3. Save to memory:
   ```json
   {
     "tool": "save_context",
     "arguments": {
       "name": "hn-top-stories-2024-01-15",
       "description": "Top Hacker News stories from today"
     }
   }
   ```

**Result delivered to user:**

| Title | Comments |
|-------|----------|
| Show HN: I built a thing | 142 comments |
| PostgreSQL 17 Released | 456 comments |
| Ask HN: What are you working on? | 289 comments |

---

## Next Steps

- [Twitter Automation](twitter_automation.md) - Automate Twitter interactions
- [Chat Capture](chat_capture.md) - Capture AI chat conversations
- [MCP Tools Reference](../MCP_TOOLS.md) - Complete tool documentation
