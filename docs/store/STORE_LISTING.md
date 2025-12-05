# Chrome Web Store Listing - SynMem

## Store Information

### Title
**SynMem - AI Browser Memory**

### Short Description (132 characters max)
```
Give your AI agents memory. Capture conversations, automate browsing, search everything via MCP.
```
*Character count: 96 characters* ✅

### Detailed Description

```
🧠 SynMem - The Memory Layer for AI Agents

Transform your browser into an intelligent memory system for AI agents. SynMem captures, stores, and retrieves web content using the Model Context Protocol (MCP).

⭐ KEY FEATURES

📝 Smart Conversation Capture
- Automatically capture chats from ChatGPT, Claude, Gemini
- Preserve conversation context across sessions
- Search through past conversations instantly

🌐 Universal Web Scraping
- Extract structured content from any webpage
- Save articles, documentation, and research
- Build your personal knowledge base

🤖 MCP Integration
- Expose your browser memory to AI agents
- Let Claude Desktop access your captured content
- Semantic search across all stored data

🔒 Privacy First
- ALL data stored locally on your device
- No cloud servers, no tracking, no ads
- Optional encryption for sensitive data

🚀 Automation Ready
- Record and replay browser actions
- Automate repetitive tasks
- Create custom workflows

💡 PERFECT FOR
- AI enthusiasts who want agents with persistent memory
- Researchers building knowledge bases
- Developers integrating browser automation
- Power users who want local-first tools

🔧 TECHNICAL HIGHLIGHTS
- Built with Rust for performance
- MCP server for AI agent integration
- SQLite for reliable local storage
- Open source (Apache 2.0)

📖 GETTING STARTED
1. Install the extension
2. Click the SynMem icon to start capturing
3. Connect your MCP-compatible AI agent
4. Your AI now has browser memory!

🌟 Open Source
SynMem is 100% open source. Contribute, customize, or self-host.
GitHub: github.com/iberi22/synmem

⚡ Start giving your AI agents the memory they deserve!
```

### Category
**Productivity**

### Language
**English (United States)**

## Visual Assets

### Icon Requirements

| Size | Use | File |
|------|-----|------|
| 128×128 px | Store listing, extension management | `icon-128.png` |
| 48×48 px | Extensions page | `icon-48.png` |
| 16×16 px | Toolbar (favicon size) | `icon-16.png` |

**Icon Design Guidelines:**
- Simple, recognizable brain/memory motif
- Works well at small sizes
- Consistent with brand colors
- No text (illegible at small sizes)

### Screenshots (1280×800 px)

**Required: Minimum 1, Maximum 5**

| Screenshot | Description |
|------------|-------------|
| 1. Main Popup | Show the main extension popup with capture controls |
| 2. Conversation Capture | Demonstrate capturing an AI chat conversation |
| 3. Search Interface | Show semantic search across stored content |
| 4. Settings Panel | Display privacy and configuration options |
| 5. MCP Integration | Show connection to Claude Desktop or similar |

### Promotional Images

| Type | Size | Purpose |
|------|------|---------|
| Small Promo Tile | 440×280 px | Store listing grid |
| Large Promo Tile | 920×680 px | Featured placement |
| Marquee | 1400×560 px | Featured collections |

**Promotional Image Content:**
- Clear product branding
- Feature highlights
- "AI Browser Memory" tagline
- Visual representation of brain/memory concept

## Submission Checklist

### Before Submission
- [ ] All icons created and optimized
- [ ] Screenshots captured at 1280×800
- [ ] Privacy policy URL is live
- [ ] Extension passes `npm run lint`
- [ ] Extension passes Chrome extension review guidelines
- [ ] manifest.json has correct version
- [ ] All permissions have justifications
- [ ] No remote code execution
- [ ] No minified/obfuscated code (or provide source maps)

### Permissions Justification

| Permission | Justification |
|------------|---------------|
| `activeTab` | Required to capture content from the current page when user triggers capture |
| `storage` | Required to store captured data locally for retrieval |
| `tabs` | Required to navigate and manage tabs for browser automation features |
| `nativeMessaging` | Required to communicate with local MCP server for AI agent integration |
| `clipboardRead` | Required for paste automation in forms and input fields |
| `clipboardWrite` | Required to copy extracted content to clipboard |
| `<all_urls>` | Required to capture content from any website the user chooses |

### Compliance Requirements
- [ ] No malware or unwanted software
- [ ] No deceptive functionality
- [ ] Clear privacy policy
- [ ] No unauthorized data collection
- [ ] No cryptocurrency mining
- [ ] No affiliate/redirect schemes

## Developer Account

### Requirements
- Google account
- $5 one-time registration fee
- Valid payment method
- Verification may be required

### Account Setup Steps
1. Go to [Chrome Web Store Developer Dashboard](https://chrome.google.com/webstore/devconsole)
2. Pay $5 registration fee
3. Complete developer verification
4. Add store listing information
5. Upload extension package (.zip)
6. Submit for review

## Review Timeline

| Review Type | Typical Duration |
|-------------|------------------|
| Initial submission | 1-3 business days |
| Update submission | 1-2 business days |
| Policy violation appeal | 3-7 business days |

## Post-Submission

### Monitoring
- Check dashboard for review status
- Monitor for policy violation emails
- Track install/uninstall metrics

### Updates
- Version bump in manifest.json
- Updated screenshots if UI changes
- Updated description for new features

---

*Document version: 1.0.0*
*Last updated: November 2024*
