# 🧠 SynMem - Quick Start Guide

Welcome to SynMem, the synthetic memory browser agent for AI systems. This guide will get you up and running in minutes.

## What is SynMem?

SynMem is a high-performance browser automation system that:
- **Navigates** the web using your authenticated sessions
- **Scrapes** content from any website with site-specific scrapers
- **Stores** browsing history with semantic search capabilities
- **Automates** repetitive browser tasks via MCP tools

## Quick Installation

### Windows (Recommended)

```powershell
# 1. Install prerequisites
winget install Rustup.Rustup
winget install GitHub.cli

# 2. Clone and build
git clone https://github.com/iberi22/synmem.git
cd synmem
cargo build --release

# 3. Install the Chrome extension (see INSTALLATION.md for details)
```

### Linux/macOS

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone and build
git clone https://github.com/iberi22/synmem.git
cd synmem
cargo build --release
```

## Basic Usage

### Start the MCP Server

```bash
# Start SynMem MCP server
synmem-mcp serve
```

### Configure Claude Desktop

Add to your Claude Desktop configuration (`~/.config/claude/config.json`):

```json
{
  "mcpServers": {
    "synmem": {
      "command": "synmem-mcp",
      "args": ["serve"]
    }
  }
}
```

### Your First Scrape

Once connected to Claude Desktop, you can use commands like:

```
Navigate to https://example.com and scrape the page content
```

Claude will use SynMem's MCP tools to:
1. Open the browser
2. Navigate to the URL
3. Extract the content
4. Return structured data

## Next Steps

- 📦 [Detailed Installation](INSTALLATION.md) - Full installation guide with troubleshooting
- 🔧 [MCP Tools Reference](MCP_TOOLS.md) - Complete list of available tools
- 🔌 [Extension API](EXTENSION_API.md) - Chrome extension documentation
- 🏗️ [Architecture](ARCHITECTURE.md) - Technical deep-dive
- 🔒 [Security](SECURITY.md) - Security best practices
- 🤝 [Contributing](CONTRIBUTING.md) - How to contribute

## Examples

- [Basic Scraping](examples/basic_scraping.md) - Extract content from websites
- [Twitter Automation](examples/twitter_automation.md) - Automate Twitter interactions
- [Chat Capture](examples/chat_capture.md) - Capture AI chat conversations

## Getting Help

- 📖 [Documentation](https://github.com/iberi22/synmem/docs)
- 🐛 [Report Issues](https://github.com/iberi22/synmem/issues)
- 💬 [Discussions](https://github.com/iberi22/synmem/discussions)

---

**License:** Apache 2.0 | **Author:** [@iberi22](https://github.com/iberi22)
