# 📦 SynMem - Press Kit

## Quick Facts

| Item | Value |
|------|-------|
| **Product Name** | SynMem (Synthetic Memory) |
| **Tagline** | AI browser automation with your authenticated sessions |
| **Category** | Developer Tools / AI Infrastructure |
| **License** | Apache 2.0 (Open Source) |
| **Language** | Rust |
| **Website** | [Coming Soon] |
| **GitHub** | https://github.com/[username]/synmem |

---

## Product Overview

### What is SynMem?

SynMem is an open-source browser automation platform that gives AI agents (Claude, GPT, etc.) secure access to authenticated web sessions. Unlike traditional scraping tools, SynMem enables AI assistants to browse the web AS the user - with full access to logged-in accounts.

### The Problem

AI agents are increasingly powerful but fundamentally limited: they cannot access the authenticated web. They can't read your emails, check your social feeds, export your AI conversations, or interact with any service requiring login. This creates a massive gap between what AI can theoretically do and what it can actually accomplish in the real world.

### The Solution

SynMem bridges this gap by:
1. **Managing browser sessions** with user's cookies and authentication
2. **Exposing browser control** via MCP (Model Context Protocol)
3. **Enabling semantic memory** for searchable browsing history
4. **Providing site-specific extractors** for popular platforms

---

## Key Features

### 🔐 Secure Session Management
- Encrypted cookie storage (AES-256-GCM)
- Master password with Argon2 key derivation
- Local-only by default
- No telemetry, no cloud requirement

### 🚀 High Performance
- Built entirely in Rust
- Rayon parallelization for CPU tasks
- Page scrape: < 500ms
- Batch 10 pages: < 2s (parallel)

### 🧠 Semantic Memory
- Vector-based embeddings for browsing history
- Natural language search
- Context-aware retrieval

### 🔌 MCP Protocol Native
- Works with Claude Desktop out of the box
- Compatible with any MCP-supporting AI
- Navigation, scraping, memory, automation tools

### 🎯 Site-Specific Extractors
- ChatGPT conversation export
- Claude chat extraction
- Gemini responses
- Twitter threads
- Universal scraper for any site

---

## Technical Architecture

### Hexagonal (Ports & Adapters) Design

```
┌──────────────────────────────────────────┐
│              SYNMEM CORE                  │
│                                           │
│  ┌─────────────────────────────────────┐ │
│  │          DOMAIN LAYER               │ │
│  │  Entities: Task, Session, Scraper   │ │
│  │  Services: Navigation, Extraction   │ │
│  └─────────────────────────────────────┘ │
│                                           │
│  ┌─────────────────────────────────────┐ │
│  │       APPLICATION LAYER (Ports)     │ │
│  │  Inbound: BrowserControl, Scraper   │ │
│  │  Outbound: BrowserDriver, Storage   │ │
│  └─────────────────────────────────────┘ │
└──────────────────────────────────────────┘
              │         │         │
              ▼         ▼         ▼
        ┌─────────┐ ┌─────────┐ ┌─────────┐
        │   MCP   │ │ Chrome  │ │  REST   │
        │ Server  │ │Extension│ │  API    │
        └─────────┘ └─────────┘ └─────────┘
```

### Tech Stack

| Component | Technology |
|-----------|------------|
| Core Runtime | Rust + Tokio |
| Parallelization | Rayon |
| Browser Automation | chromiumoxide (CDP) |
| Storage | SQLite (rusqlite) |
| Embeddings | fastembed (optional) |
| Protocol | MCP (stdio/SSE) |

---

## Business Model

### Open Core Model

| Tier | Price | Features |
|------|-------|----------|
| **Core (Free)** | $0 | Full browser agent, MCP tools, local storage |
| **SynMem Cloud** | $19/mo | Cloud sessions, sync, remote access |
| **SynMem Pro** | $49/mo | Multi-browser, unlimited API, webhooks |
| **Enterprise** | Custom | On-premise, SSO, audit logs |

### Future: Scraper Marketplace
- Community-built site-specific scrapers
- 30% platform commission
- Quality verification system

---

## Use Cases

### AI Research
Let AI assistants gather information from authenticated sources - research papers behind paywalls, internal wikis, premium content.

### Workflow Automation
Automate repetitive browser tasks with AI orchestration. Form filling, data extraction, multi-step processes.

### Chat Export & Search
Extract and semantically search conversations from AI platforms (ChatGPT, Claude, Gemini).

### Social Media Management
Read mentions, compose posts, manage threads across platforms - with AI assistance.

---

## Target Market

### Phase 1 (Launch)
- Power users
- Developers
- AI enthusiasts

### Phase 2
- Indie hackers
- Solopreneurs
- Content creators

### Phase 3
- Startups
- Small teams

### Phase 4
- Enterprise

---

## Competitive Landscape

| Tool | Approach | Limitation |
|------|----------|------------|
| **Playwright/Puppeteer** | Browser automation | No AI integration, no session management |
| **Browserless** | Cloud browsers | No MCP, focused on screenshots |
| **AgentQL** | AI queries | Requires cloud, no session persistence |
| **SynMem** | MCP-native, session-aware | ✅ Full AI integration, local-first |

---

## Media Assets

### Logo
[Logo files to be created]
- SVG (vector)
- PNG (512x512, 256x256, 128x128)
- Favicon (32x32, 16x16)

### Screenshots
[Screenshots to be captured]
- Terminal with MCP tools
- Architecture diagram
- Chrome extension UI
- Code examples

### Product Video
[Video to be produced]
- Length: < 3 minutes
- Content: Problem → Solution → Demo
- Style: Technical but accessible

---

## Brand Guidelines

### Colors

| Name | Hex | Usage |
|------|-----|-------|
| Primary | `#6366F1` | Indigo - main brand |
| Secondary | `#F59E0B` | Amber - accents |
| Dark | `#1F2937` | Gray 800 - text |
| Light | `#F9FAFB` | Gray 50 - backgrounds |

### Typography

| Use | Font |
|-----|------|
| Headings | Inter (Bold) |
| Body | Inter (Regular) |
| Code | JetBrains Mono |

### Voice & Tone
- **Technical but accessible** - We're developers talking to developers
- **Confident not arrogant** - We solve real problems
- **Open and transparent** - Open source values

---

## Contact Information

| Type | Contact |
|------|---------|
| Media Inquiries | [email] |
| Technical Questions | GitHub Discussions |
| Partnership | [email] |
| Twitter | [@handle] |

---

## Boilerplate

### Short (50 words)
SynMem is an open-source browser agent that gives AI assistants secure access to authenticated web sessions. Built in Rust for performance, it exposes browser control via MCP protocol - enabling AI to browse, scrape, and automate the web with your logins.

### Medium (100 words)
SynMem (Synthetic Memory) is an open-source browser automation platform designed for AI integration. It gives AI assistants like Claude and GPT secure access to authenticated web sessions - enabling them to browse the logged-in web as the user. Built entirely in Rust with a hexagonal architecture, SynMem delivers blazing performance (parallel scraping in under 2 seconds) while maintaining security through encrypted session management. The MCP protocol integration makes it compatible with modern AI tools out of the box. Site-specific extractors for ChatGPT, Claude, Gemini, and Twitter are included.

---

## FAQ for Press

**Q: Is this safe?**
A: Yes. SynMem encrypts all session data with AES-256-GCM, uses Argon2 for key derivation, and runs locally by default. Users maintain full control.

**Q: What's MCP?**
A: Model Context Protocol - an open standard for AI tool integration. It allows AI assistants to call external tools in a structured way.

**Q: Why Rust?**
A: Performance and safety. Rust's ownership model prevents memory bugs, and Rayon enables efficient parallelization without the overhead of Node.js.

**Q: Is it really free?**
A: The core engine is 100% free and open source (Apache 2.0). Premium cloud features for teams are paid.

**Q: Who made this?**
A: [Creator information]

---

*Press Kit Version 1.0 - Last updated: 2024*
