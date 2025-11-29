# 🏗️ SynMem Technical Architecture

This document provides a comprehensive technical overview of SynMem's architecture, design decisions, and implementation details.

## Overview

SynMem is a synthetic memory browser agent built with:
- **Rust** for the core engine (performance, safety)
- **TypeScript** for the Chrome extension
- **MCP Protocol** for AI assistant integration

---

## Architectural Pattern: Hexagonal (Ports & Adapters)

SynMem follows the Hexagonal Architecture pattern, also known as Ports and Adapters. This provides:

- **Testability**: Domain logic can be tested in isolation
- **Flexibility**: Easy to swap implementations (storage, browser engine)
- **Maintainability**: Clear separation of concerns

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              SYNMEM CORE                                     │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────────┐│
│  │                         DOMAIN LAYER                                     ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ ││
│  │  │  BrowserTask │  │   Session    │  │   Scraper    │  │   Memory     │ ││
│  │  │   Entity     │  │   Entity     │  │   Entity     │  │   Entity     │ ││
│  │  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘ ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                   ││
│  │  │  Navigation  │  │  Extraction  │  │  Automation  │                   ││
│  │  │   Service    │  │   Service    │  │   Service    │                   ││
│  │  └──────────────┘  └──────────────┘  └──────────────┘                   ││
│  └─────────────────────────────────────────────────────────────────────────┘│
│                                    │                                         │
│  ┌─────────────────────────────────┴───────────────────────────────────────┐│
│  │                      APPLICATION LAYER (Ports)                          ││
│  │                                                                          ││
│  │  ┌─────────────────────────────┐  ┌─────────────────────────────┐       ││
│  │  │      INBOUND PORTS          │  │      OUTBOUND PORTS         │       ││
│  │  │  - BrowserControlPort       │  │  - BrowserDriverPort        │       ││
│  │  │  - ScraperPort              │  │  - StoragePort              │       ││
│  │  │  - MemoryQueryPort          │  │  - SessionPersistencePort   │       ││
│  │  │  - AutomationPort           │  │  - EmbeddingPort            │       ││
│  │  └─────────────────────────────┘  └─────────────────────────────┘       ││
│  └─────────────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
        ┌────────────────────────────┼────────────────────────────┐
        │                            │                            │
        ▼                            ▼                            ▼
┌───────────────────┐    ┌───────────────────┐    ┌───────────────────┐
│  PRIMARY ADAPTERS │    │  PRIMARY ADAPTERS │    │  PRIMARY ADAPTERS │
│    (Driving)      │    │    (Driving)      │    │    (Driving)      │
├───────────────────┤    ├───────────────────┤    ├───────────────────┤
│   MCP Server      │    │  Chrome Extension │    │    REST API       │
│   (stdio/SSE)     │    │  (Native Host)    │    │   (Optional)      │
└───────────────────┘    └───────────────────┘    └───────────────────┘

        ┌────────────────────────────┼────────────────────────────┐
        │                            │                            │
        ▼                            ▼                            ▼
┌───────────────────┐    ┌───────────────────┐    ┌───────────────────┐
│ SECONDARY ADAPTERS│    │ SECONDARY ADAPTERS│    │ SECONDARY ADAPTERS│
│    (Driven)       │    │    (Driven)       │    │    (Driven)       │
├───────────────────┤    ├───────────────────┤    ├───────────────────┤
│  Chromium Driver  │    │   SQLite Storage  │    │  Qdrant/Faiss     │
│  (chromiumoxide)  │    │   (rusqlite)      │    │  (Embeddings)     │
└───────────────────┘    └───────────────────┘    └───────────────────┘
```

---

## Domain Layer

### Entities

#### BrowserTask

Represents a unit of browser work to be performed.

```rust
pub struct BrowserTask {
    pub id: Uuid,
    pub task_type: TaskType,
    pub url: Option<Url>,
    pub selector: Option<String>,
    pub data: Option<serde_json::Value>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

pub enum TaskType {
    Navigate,
    Click,
    Type,
    Scrape,
    Screenshot,
    WaitFor,
}

pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}
```

#### Session

Represents a browser session with authentication state.

```rust
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub cookies: Vec<Cookie>,
    pub local_storage: HashMap<String, String>,
    pub session_storage: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
}

pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<DateTime<Utc>>,
    pub http_only: bool,
    pub secure: bool,
    pub same_site: SameSite,
}
```

#### ScrapedPage

Represents extracted content from a webpage.

```rust
pub struct ScrapedPage {
    pub id: Uuid,
    pub url: Url,
    pub title: String,
    pub content: String,
    pub html: Option<String>,
    pub links: Vec<Link>,
    pub images: Vec<Image>,
    pub metadata: PageMetadata,
    pub scraped_at: DateTime<Utc>,
}

pub struct PageMetadata {
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub author: Option<String>,
    pub published_date: Option<DateTime<Utc>>,
}
```

#### Memory

Represents a stored memory entry with embeddings for semantic search.

```rust
pub struct Memory {
    pub id: Uuid,
    pub source_type: SourceType,
    pub source_id: Uuid,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

pub enum SourceType {
    Page,
    Chat,
    Screenshot,
    Custom,
}
```

### Services

#### Navigation Service

Orchestrates browser navigation operations.

```rust
pub struct NavigationService<D: BrowserDriverPort> {
    driver: D,
}

impl<D: BrowserDriverPort> NavigationService<D> {
    pub async fn navigate(&self, url: Url, options: NavigateOptions) -> Result<NavigationResult>;
    pub async fn click(&self, selector: &str) -> Result<()>;
    pub async fn type_text(&self, selector: &str, text: &str) -> Result<()>;
    pub async fn scroll(&self, direction: ScrollDirection, amount: Option<u32>) -> Result<()>;
}
```

#### Extraction Service

Handles content extraction from pages.

```rust
pub struct ExtractionService<D: BrowserDriverPort, S: StoragePort> {
    driver: D,
    storage: S,
}

impl<D: BrowserDriverPort, S: StoragePort> ExtractionService<D, S> {
    pub async fn scrape_page(&self, selectors: Option<Selectors>) -> Result<ScrapedPage>;
    pub async fn scrape_chat(&self, platform: ChatPlatform) -> Result<ChatConversation>;
    pub async fn extract_links(&self, scope: Option<String>) -> Result<Vec<Link>>;
    pub async fn extract_text(&self, selector: Option<String>) -> Result<String>;
}
```

#### Automation Service

Manages macros and automated workflows.

```rust
pub struct AutomationService<D: BrowserDriverPort, S: StoragePort> {
    driver: D,
    storage: S,
    macros: HashMap<String, Macro>,
}

impl<D: BrowserDriverPort, S: StoragePort> AutomationService<D, S> {
    pub async fn record_macro(&mut self, name: &str) -> Result<MacroRecorder>;
    pub async fn play_macro(&self, name: &str, variables: HashMap<String, String>) -> Result<()>;
    pub async fn fill_form(&self, fields: HashMap<String, String>) -> Result<()>;
}
```

---

## Application Layer (Ports)

### Inbound Ports

These interfaces define how external systems interact with the domain.

```rust
#[async_trait]
pub trait BrowserControlPort {
    async fn navigate(&self, url: Url, options: NavigateOptions) -> Result<NavigationResult>;
    async fn click(&self, selector: &str) -> Result<()>;
    async fn type_text(&self, selector: &str, text: &str) -> Result<()>;
    async fn screenshot(&self, options: ScreenshotOptions) -> Result<Vec<u8>>;
}

#[async_trait]
pub trait ScraperPort {
    async fn scrape_page(&self, options: ScrapeOptions) -> Result<ScrapedPage>;
    async fn scrape_chat(&self, platform: ChatPlatform) -> Result<ChatConversation>;
}

#[async_trait]
pub trait MemoryQueryPort {
    async fn search(&self, query: &str, options: SearchOptions) -> Result<Vec<SearchResult>>;
    async fn get_recent(&self, count: usize, type_filter: Option<SourceType>) -> Result<Vec<Memory>>;
}

#[async_trait]
pub trait AutomationPort {
    async fn record_macro(&self, name: &str) -> Result<()>;
    async fn stop_recording(&self) -> Result<Macro>;
    async fn play_macro(&self, name: &str, variables: HashMap<String, String>) -> Result<()>;
}
```

### Outbound Ports

These interfaces define dependencies that the domain requires.

```rust
#[async_trait]
pub trait BrowserDriverPort {
    async fn new_page(&self) -> Result<PageHandle>;
    async fn navigate(&self, page: &PageHandle, url: Url) -> Result<()>;
    async fn evaluate(&self, page: &PageHandle, script: &str) -> Result<serde_json::Value>;
    async fn wait_for_selector(&self, page: &PageHandle, selector: &str) -> Result<()>;
    async fn screenshot(&self, page: &PageHandle) -> Result<Vec<u8>>;
}

#[async_trait]
pub trait StoragePort {
    async fn save_page(&self, page: &ScrapedPage) -> Result<()>;
    async fn get_page(&self, id: Uuid) -> Result<Option<ScrapedPage>>;
    async fn save_session(&self, session: &Session) -> Result<()>;
    async fn get_session(&self, id: Uuid) -> Result<Option<Session>>;
}

#[async_trait]
pub trait EmbeddingPort {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>>;
    async fn search_similar(&self, embedding: &[f32], limit: usize) -> Result<Vec<Uuid>>;
}
```

---

## Adapters

### Primary Adapters (Driving)

#### MCP Server Adapter

Exposes functionality via the Model Context Protocol.

```rust
pub struct McpServer {
    browser_control: Arc<dyn BrowserControlPort>,
    scraper: Arc<dyn ScraperPort>,
    memory: Arc<dyn MemoryQueryPort>,
    automation: Arc<dyn AutomationPort>,
}

impl McpServer {
    pub async fn serve(self, transport: Transport) -> Result<()> {
        // Handle MCP protocol messages
        // Route to appropriate domain services
    }
}
```

#### Native Messaging Host Adapter

Bridges the Chrome extension with the Rust backend.

```rust
pub struct NativeHost {
    browser_control: Arc<dyn BrowserControlPort>,
    scraper: Arc<dyn ScraperPort>,
}

impl NativeHost {
    pub async fn run(self) -> Result<()> {
        // Read/write length-prefixed JSON from stdio
        // Route messages to domain services
    }
}
```

### Secondary Adapters (Driven)

#### Chromium Driver Adapter

Implements browser control using chromiumoxide.

```rust
pub struct ChromiumDriver {
    browser: Browser,
}

impl ChromiumDriver {
    pub async fn new(options: BrowserOptions) -> Result<Self> {
        let (browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .with_head(options.headless)
                .user_data_dir(options.user_data_dir)
                .build()?
        ).await?;

        tokio::spawn(async move {
            handler.next().await;
        });

        Ok(Self { browser })
    }
}

#[async_trait]
impl BrowserDriverPort for ChromiumDriver {
    // Implementation using chromiumoxide APIs
}
```

#### SQLite Storage Adapter

Implements persistence using SQLite.

```rust
pub struct SqliteStorage {
    pool: Pool<SqliteConnectionManager>,
}

impl SqliteStorage {
    pub fn new(path: &Path) -> Result<Self> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        
        // Run migrations
        let conn = pool.get()?;
        conn.execute_batch(include_str!("migrations.sql"))?;
        
        Ok(Self { pool })
    }
}

#[async_trait]
impl StoragePort for SqliteStorage {
    // Implementation using rusqlite
}
```

#### FastEmbed Adapter

Implements local embeddings using fastembed.

```rust
pub struct FastEmbedAdapter {
    model: TextEmbedding,
}

impl FastEmbedAdapter {
    pub fn new() -> Result<Self> {
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2)
        )?;
        
        Ok(Self { model })
    }
}

#[async_trait]
impl EmbeddingPort for FastEmbedAdapter {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.model.embed(vec![text], None)?;
        Ok(embeddings.into_iter().next().unwrap())
    }
}
```

---

## Parallelization Strategy

SynMem uses Rayon for CPU-bound parallelization:

### DOM Processing

```rust
use rayon::prelude::*;

pub fn process_pages_parallel(pages: Vec<RawPage>) -> Vec<ScrapedPage> {
    pages.par_iter()
        .map(|page| {
            let dom = parse_dom(&page.html);
            let content = extract_content(&dom);
            let links = extract_links(&dom);
            
            ScrapedPage {
                content,
                links,
                // ...
            }
        })
        .collect()
}
```

### Batch Scraping

```rust
pub async fn batch_scrape(urls: Vec<Url>) -> Vec<Result<ScrapedPage>> {
    // Split into chunks for parallel processing
    let chunk_size = num_cpus::get();
    
    let results: Vec<_> = urls.chunks(chunk_size)
        .map(|chunk| {
            chunk.par_iter()
                .map(|url| scrape_single(url))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    
    results
}
```

---

## Data Flow

### Scraping Flow

```
1. MCP Request (scrape_page)
        │
        ▼
2. McpServer.handle_tool()
        │
        ▼
3. ScraperPort.scrape_page()
        │
        ▼
4. ExtractionService.scrape_page()
        │
        ├─────────────────────────────┐
        ▼                             ▼
5. BrowserDriverPort.evaluate()   6. Parse DOM (Rayon)
        │                             │
        └──────────────┬──────────────┘
                       ▼
              7. StoragePort.save_page()
                       │
                       ▼
              8. EmbeddingPort.generate()
                       │
                       ▼
              9. Return ScrapedPage
```

### Memory Search Flow

```
1. MCP Request (search_memory)
        │
        ▼
2. MemoryQueryPort.search()
        │
        ▼
3. EmbeddingPort.generate_embedding(query)
        │
        ▼
4. EmbeddingPort.search_similar(embedding)
        │
        ▼
5. StoragePort.get_pages(ids)
        │
        ▼
6. Return SearchResults
```

---

## Technology Stack

### Core (Rust)

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.x | Async runtime |
| `rayon` | 1.x | CPU parallelization |
| `chromiumoxide` | 0.7.x | Browser automation (CDP) |
| `serde` | 1.x | Serialization |
| `rusqlite` | 0.31.x | SQLite storage |
| `tower` | 0.4.x | Service abstractions |
| `tracing` | 0.1.x | Observability |
| `mcp-rust-sdk` | latest | MCP protocol |
| `fastembed` | latest | Local embeddings |

### Extension (TypeScript)

| Package | Purpose |
|---------|---------|
| `chrome-types` | Chrome API types |
| `vite` | Build tool |
| `react` | Popup UI |

---

## Performance Targets

| Metric | Target | Implementation |
|--------|--------|----------------|
| Page scrape | < 500ms | chromiumoxide + Rayon parsing |
| Batch scrape (10 pages) | < 2s | Parallel processing |
| Memory search | < 100ms | fastembed + SQLite FTS |
| MCP tool response | < 200ms | Efficient message routing |
| Extension → Native | < 50ms | Native messaging |

---

## Security Model

### Credentials Storage

- Cookies encrypted with AES-256-GCM (via `ring` crate)
- Master password derived with Argon2id
- Never stored in plaintext, never logged

### Extension Permissions

- Minimal permissions by default
- User consent required for each site
- No remote code execution

### MCP Security

- Local-only by default (stdio transport)
- Optional TLS for remote connections
- Rate limiting on all endpoints
