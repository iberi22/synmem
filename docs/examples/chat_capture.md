# 💬 Chat Capture with SynMem

This guide shows you how to capture and save conversations from AI chat interfaces like ChatGPT, Claude, and Gemini.

## Supported Platforms

| Platform | URL | Status |
|----------|-----|--------|
| ChatGPT | chat.openai.com | ✅ Supported |
| Claude | claude.ai | ✅ Supported |
| Gemini | gemini.google.com | ✅ Supported |

---

## Capturing Conversations

### From ChatGPT

```
User: Save my current ChatGPT conversation
```

SynMem will automatically:
1. Detect the ChatGPT interface
2. Extract all messages
3. Preserve code blocks and formatting
4. Store with metadata

**MCP Tool:**

```json
{
  "tool": "scrape_chat",
  "arguments": {
    "platform": "chatgpt"
  }
}
```

**Result:**

```json
{
  "success": true,
  "platform": "chatgpt",
  "conversation": {
    "id": "conv-abc123",
    "title": "Help with Rust async",
    "model": "GPT-4",
    "messages": [
      {
        "role": "user",
        "content": "How do I use async/await in Rust?",
        "timestamp": "2024-01-15T10:00:00Z"
      },
      {
        "role": "assistant",
        "content": "In Rust, async programming works through the `async` and `await` keywords...\n\n```rust\nasync fn fetch_data() -> Result<String, Error> {\n    let response = reqwest::get(\"https://api.example.com\").await?;\n    Ok(response.text().await?)\n}\n```",
        "timestamp": "2024-01-15T10:00:05Z"
      }
    ],
    "total_messages": 12
  }
}
```

---

### From Claude

```
User: Capture this Claude conversation
```

**MCP Tool:**

```json
{
  "tool": "scrape_chat",
  "arguments": {
    "platform": "claude"
  }
}
```

The Claude scraper handles:
- Multiple artifacts (code, documents)
- Thinking indicators
- Tool use outputs
- Conversation branches

---

### From Gemini

```
User: Save my Gemini chat
```

**MCP Tool:**

```json
{
  "tool": "scrape_chat",
  "arguments": {
    "platform": "gemini"
  }
}
```

The Gemini scraper captures:
- Multi-modal responses (text + images)
- Source citations
- Web search results
- Generated code

---

## Extracting Specific Messages

### Limit Message Count

```
User: Get just the last 5 messages
```

```json
{
  "tool": "scrape_chat",
  "arguments": {
    "max_messages": 5
  }
}
```

### Filter by Role

After capturing, you can filter:

```
User: Show me only the assistant responses about error handling
```

SynMem will:
1. Filter messages where `role == "assistant"`
2. Search content for "error handling"
3. Return matching messages

---

## Auto-Detection

SynMem automatically detects the chat platform based on the URL:

| URL Pattern | Detected Platform |
|-------------|-------------------|
| `chat.openai.com/*` | ChatGPT |
| `claude.ai/*` | Claude |
| `gemini.google.com/*` | Gemini |

You can also let SynMem auto-detect:

```json
{
  "tool": "scrape_chat",
  "arguments": {}  // Platform auto-detected
}
```

---

## Saving Conversations

### Save to Memory

All captured conversations are automatically saved to SynMem's memory:

```json
{
  "tool": "save_context",
  "arguments": {
    "name": "rust-async-help",
    "description": "ChatGPT conversation about async programming in Rust"
  }
}
```

### Export as Markdown

```
User: Export this conversation as markdown
```

**Generated Markdown:**

```markdown
# Help with Rust async

**Platform:** ChatGPT (GPT-4)
**Date:** 2024-01-15

---

## User

How do I use async/await in Rust?

---

## Assistant

In Rust, async programming works through the `async` and `await` keywords...

\`\`\`rust
async fn fetch_data() -> Result<String, Error> {
    let response = reqwest::get("https://api.example.com").await?;
    Ok(response.text().await?)
}
\`\`\`

---
```

### Export as JSON

```
User: Export as JSON for processing
```

Returns the full structured JSON response.

---

## Searching Chat History

### Semantic Search

```
User: Find the conversation where we discussed database migrations
```

```json
{
  "tool": "search_memory",
  "arguments": {
    "query": "database migrations schema changes",
    "type": "chats",
    "limit": 5
  }
}
```

**Result:**

```json
{
  "success": true,
  "results": [
    {
      "id": "conv-def456",
      "platform": "claude",
      "title": "PostgreSQL migration help",
      "relevance_score": 0.94,
      "snippet": "...to create a migration, use sqlx migrate add...",
      "captured_at": "2024-01-10T15:30:00Z"
    }
  ]
}
```

### Get Recent Conversations

```
User: Show my recent AI conversations
```

```json
{
  "tool": "get_recent",
  "arguments": {
    "type": "chats",
    "limit": 10,
    "include_content": false
  }
}
```

**Result:**

```json
{
  "success": true,
  "items": [
    {
      "id": "conv-abc123",
      "platform": "chatgpt",
      "title": "Help with Rust async",
      "captured_at": "2024-01-15T10:30:00Z",
      "message_count": 12
    },
    {
      "id": "conv-def456",
      "platform": "claude",
      "title": "PostgreSQL migration help",
      "captured_at": "2024-01-10T15:30:00Z",
      "message_count": 8
    }
  ]
}
```

---

## Conversation Comparison

### Compare Responses

```
User: Compare how ChatGPT and Claude explained async programming
```

SynMem can:
1. Search for relevant conversations on each platform
2. Extract the relevant sections
3. Present a side-by-side comparison

---

## Handling Code Blocks

### Preserving Formatting

Code blocks are preserved with language detection:

```json
{
  "messages": [
    {
      "role": "assistant",
      "content": "Here's an example:",
      "code_blocks": [
        {
          "language": "rust",
          "code": "async fn main() {\n    let result = fetch().await;\n}"
        }
      ]
    }
  ]
}
```

### Extracting Just Code

```
User: Get just the code snippets from this conversation
```

SynMem will filter to return only code blocks:

```json
{
  "success": true,
  "code_blocks": [
    {
      "language": "rust",
      "code": "async fn fetch_data()...",
      "context": "Response to: How do I use async?"
    },
    {
      "language": "rust",
      "code": "#[tokio::main]\nasync fn main()...",
      "context": "Response to: Show me a complete example"
    }
  ]
}
```

---

## Batch Operations

### Capture All Open Chats

```
User: Save all my open AI chat tabs
```

SynMem will:
1. List all tabs with supported chat URLs
2. Capture each conversation
3. Store with metadata

### Scheduled Capture

Using macros, you can set up regular captures:

```json
{
  "tool": "record_macro",
  "arguments": {
    "name": "daily-chat-backup",
    "description": "Capture all AI chats daily"
  }
}
```

---

## Privacy Considerations

### What's Captured

- Message content (text, code)
- Timestamps
- Platform metadata
- Conversation structure

### What's NOT Captured

- Authentication tokens
- Session cookies (for capture purposes)
- Personal account details
- Payment information

### Data Location

All captured data is stored locally:

```
~/.synmem/
├── memory/
│   ├── chats/
│   │   ├── chatgpt/
│   │   │   ├── conv-abc123.json
│   │   │   └── conv-xyz789.json
│   │   ├── claude/
│   │   └── gemini/
│   └── embeddings/
└── index.db
```

---

## Troubleshooting

### "No messages found"

**Cause:** Page not fully loaded

**Solution:**
```json
{
  "tool": "wait_for",
  "arguments": {
    "selector": "[data-message-author-role]",
    "timeout_ms": 10000
  }
}
```

### "Platform not detected"

**Cause:** URL doesn't match known patterns

**Solution:** Specify platform manually:
```json
{
  "tool": "scrape_chat",
  "arguments": {
    "platform": "chatgpt"
  }
}
```

### "Partial capture"

**Cause:** Long conversation not fully scrolled

**Solution:** Scroll to load all messages first:
```json
{
  "tool": "scroll",
  "arguments": {
    "direction": "top"
  }
}

{
  "tool": "wait_for",
  "arguments": {
    "time_ms": 2000
  }
}

{
  "tool": "scrape_chat",
  "arguments": {}
}
```

---

## Use Cases

### Research Notes

1. Have a conversation about a topic
2. Capture the conversation
3. Search later when you need that information

### Code Reference

1. Get help writing code
2. Capture the working solution
3. Search your chat history instead of web search

### Learning Archive

1. Capture explanations and tutorials
2. Build a personal knowledge base
3. Review past learning sessions

### Team Knowledge

1. Capture problem-solving conversations
2. Share context with team members
3. Build organizational knowledge

---

## Next Steps

- [Basic Scraping](basic_scraping.md) - General web scraping
- [Twitter Automation](twitter_automation.md) - Social media automation
- [MCP Tools Reference](../MCP_TOOLS.md) - Complete tool documentation
