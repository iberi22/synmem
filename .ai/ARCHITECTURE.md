# 🧠 SynMem - Synthetic Memory Browser Agent

## Vision
**El sistema de memoria sintética más avanzado para agentes AI** - Captura, navega y automatiza la web con tu sesión autenticada, exponiendo todo via MCP.

---

## 🎯 Product Strategy

### Open Source Strategy: **Open Core Model**
- **Core Engine**: 100% Open Source (Apache 2.0)
- **Premium Features**: Licencia comercial
  - Cloud sync de sesiones
  - Dashboard analytics
  - Team collaboration
  - Priority support

### Revenue Streams
1. **SynMem Cloud** - $19/mes: Browser sessions en la nube, sync automático
2. **SynMem Pro** - $49/mes: Multi-browser, API ilimitada, webhooks
3. **SynMem Enterprise** - Custom: On-premise, SSO, audit logs
4. **Marketplace de Scrapers** - 30% comisión: Comunidad vende scrapers específicos

### Target Market
- **Fase 1**: Power users, developers, AI enthusiasts (TÚ)
- **Fase 2**: Indie hackers, solopreneurs
- **Fase 3**: Startups, small teams
- **Fase 4**: Enterprise

---

## 🏗️ Architecture: Hexagonal (Ports & Adapters)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              SYNMEM CORE                                     │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────────┐│
│  │                         DOMAIN LAYER                                     ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ ││
│  │  │  BrowserTask │  │   Session    │  │  Scraper     │  │  Memory      │ ││
│  │  │  Entity      │  │   Entity     │  │  Entity      │  │  Entity      │ ││
│  │  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘ ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                   ││
│  │  │  Navigation  │  │  Extraction  │  │  Automation  │                   ││
│  │  │  Service     │  │  Service     │  │  Service     │                   ││
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

## 🦀 Tech Stack

### Core (Rust)
| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.x | Async runtime |
| `rayon` | 1.x | **Parallelización CPU-bound** |
| `chromiumoxide` | 0.7.x | Browser automation (CDP) |
| `serde` | 1.x | Serialization |
| `rusqlite` | 0.31.x | SQLite storage |
| `tower` | 0.4.x | Service abstractions |
| `tracing` | 0.1.x | Observability |

### MCP Server
| Crate | Purpose |
|-------|---------|
| `mcp-rust-sdk` | MCP protocol implementation |
| `async-trait` | Async trait definitions |

### Parallelization (Rayon)
| Crate | Purpose |
|-------|---------|
| `rayon` | Data parallelism for CPU tasks |
| `crossbeam` | Concurrent data structures |
| `dashmap` | Concurrent HashMap |

### Browser Extension (TypeScript)
| Package | Purpose |
|---------|---------|
| `chrome-types` | Chrome API types |
| `vite` | Build tool |

### Embeddings (Optional)
| Crate | Purpose |
|-------|---------|
| `fastembed` | Local embeddings (Rust native) |
| `qdrant-client` | Vector DB client |

---

## 📁 Project Structure

```
synmem/
├── Cargo.toml                    # Workspace root
├── .ai/
│   ├── ARCHITECTURE.md           # This file
│   └── CONTEXT_LOG.md            # Session notes
│
├── crates/
│   ├── synmem-core/              # Domain + Application Layer
│   │   ├── src/
│   │   │   ├── domain/
│   │   │   │   ├── entities/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── browser_task.rs
│   │   │   │   │   ├── session.rs
│   │   │   │   │   ├── scraped_page.rs
│   │   │   │   │   └── memory.rs
│   │   │   │   ├── services/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── navigation.rs
│   │   │   │   │   ├── extraction.rs
│   │   │   │   │   └── automation.rs
│   │   │   │   └── mod.rs
│   │   │   ├── ports/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inbound/
│   │   │   │   │   ├── browser_control.rs
│   │   │   │   │   ├── scraper.rs
│   │   │   │   │   └── memory_query.rs
│   │   │   │   └── outbound/
│   │   │   │       ├── browser_driver.rs
│   │   │   │       ├── storage.rs
│   │   │   │       └── embedding.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── synmem-browser/           # Browser Driver Adapter
│   │   ├── src/
│   │   │   ├── chromium/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── driver.rs
│   │   │   │   ├── session_manager.rs
│   │   │   │   └── dom_extractor.rs
│   │   │   ├── parallel/         # Rayon parallelization
│   │   │   │   ├── mod.rs
│   │   │   │   ├── page_processor.rs
│   │   │   │   └── batch_scraper.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── synmem-storage/           # Storage Adapter
│   │   ├── src/
│   │   │   ├── sqlite/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── repository.rs
│   │   │   │   └── migrations.rs
│   │   │   ├── embeddings/
│   │   │   │   ├── mod.rs
│   │   │   │   └── fastembed_adapter.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── synmem-mcp/               # MCP Server Adapter
│   │   ├── src/
│   │   │   ├── server.rs
│   │   │   ├── tools/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── navigate.rs
│   │   │   │   ├── scrape.rs
│   │   │   │   ├── search.rs
│   │   │   │   └── automate.rs
│   │   │   ├── resources/
│   │   │   │   └── mod.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   └── synmem-cli/               # CLI Binary
│       ├── src/
│       │   └── main.rs
│       └── Cargo.toml
│
├── extension/                    # Chrome Extension
│   ├── src/
│   │   ├── background/
│   │   │   └── service-worker.ts
│   │   ├── content/
│   │   │   ├── scrapers/
│   │   │   │   ├── universal.ts
│   │   │   │   ├── gemini.ts
│   │   │   │   ├── chatgpt.ts
│   │   │   │   ├── claude.ts
│   │   │   │   └── twitter.ts
│   │   │   └── index.ts
│   │   ├── popup/
│   │   │   └── index.tsx
│   │   └── native-host/
│   │       └── bridge.ts
│   ├── manifest.json
│   ├── package.json
│   └── vite.config.ts
│
├── native-host/                  # Native Messaging Host (Rust)
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
│
├── tests/
│   ├── integration/
│   └── e2e/
│
├── docs/
│   ├── MCP_TOOLS.md
│   ├── EXTENSION_API.md
│   └── DEPLOYMENT.md
│
└── scripts/
    ├── init_project.ps1
    └── install_extension.ps1
```

---

## 🔧 MCP Tools Specification

### Navigation Tools
```rust
// navigate_to: Navega a una URL
// click: Click en elemento por selector/texto
// type_text: Escribe texto en input
// scroll: Scroll en la página
// screenshot: Captura pantalla
// wait_for: Espera elemento/condición
```

### Scraping Tools
```rust
// scrape_page: Extrae contenido estructurado
// scrape_chat: Extrae conversación de AI chat
// extract_links: Lista todos los enlaces
// extract_text: Extrae texto limpio
// get_dom: Obtiene DOM simplificado
```

### Memory Tools
```rust
// search_memory: Búsqueda semántica en historial
// get_recent: Últimas N páginas/chats
// save_context: Guarda contexto actual
// list_sessions: Lista sesiones guardadas
```

### Automation Tools
```rust
// record_macro: Graba secuencia de acciones
// play_macro: Reproduce macro grabado
// twitter_post: Publica tweet
// twitter_read_thread: Lee hilo completo
// fill_form: Llena formulario con datos
```

---

## 🚀 Roadmap Completo

### Phase 0: Foundation (Semanas 1-2)
- [ ] Setup workspace Cargo
- [ ] Definir traits/ports
- [ ] Estructura hexagonal base
- [ ] CI/CD básico

### Phase 1: Core Engine (Semanas 3-6)
- [ ] Browser driver con chromiumoxide
- [ ] Session management (cookies, storage)
- [ ] DOM extraction paralelo (Rayon)
- [ ] SQLite storage adapter

### Phase 2: MCP Server (Semanas 7-8)
- [ ] MCP protocol implementation
- [ ] Navigation tools
- [ ] Scraping tools
- [ ] Memory tools

### Phase 3: Extension (Semanas 9-12)
- [ ] Chrome extension base
- [ ] Native messaging host
- [ ] Site-specific scrapers
- [ ] Real-time sync

### Phase 4: AI Integration (Semanas 13-16)
- [ ] Local embeddings (fastembed)
- [ ] Semantic search
- [ ] Smart replay (record → optimize → replay)
- [ ] LLM-guided navigation

### Phase 5: Polish & Launch (Semanas 17-20)
- [ ] Documentation completa
- [ ] Website + landing page
- [ ] Chrome Web Store submission
- [ ] Product Hunt launch

### Phase 6: Monetization (Post-launch)
- [ ] SynMem Cloud infrastructure
- [ ] Subscription system
- [ ] Scraper marketplace
- [ ] Enterprise features

---

## 🔒 Security Considerations

### Credentials Storage
- Cookies encriptados con `ring` (AES-256-GCM)
- Master password derivado con Argon2
- Nunca en plaintext, nunca en logs

### Extension Permissions
- Minimal permissions model
- User consent for each site
- No remote code execution

### MCP Security
- Local-only by default
- Optional TLS for remote
- Rate limiting

---

## 📊 Performance Targets

| Metric | Target |
|--------|--------|
| Page scrape | < 500ms |
| Batch scrape (10 pages) | < 2s (parallel) |
| Memory search | < 100ms |
| MCP tool response | < 200ms |
| Extension → Native | < 50ms |

### Rayon Parallelization Strategy
```rust
// CPU-bound tasks parallelized:
// - DOM parsing
// - Text extraction
// - Embedding generation
// - Batch page processing

use rayon::prelude::*;

pages.par_iter()
    .map(|page| extract_content(page))
    .collect()
```

---

## 🧪 Testing Strategy

### Unit Tests
- Domain logic (services, entities)
- Port implementations
- Utility functions

### Integration Tests
- Browser driver + storage
- MCP server + tools
- Extension + native host

### E2E Tests
- Full workflow: navigate → scrape → store → search
- Claude Desktop integration
- Real site scraping (Twitter, Gemini, etc.)

---

## Key Decisions

### Decision 1: Rust over Python
- **Date:** 2025-11-29
- **Context:** Need high performance, memory safety, and parallelism
- **Decision:** Rust with Rayon for CPU parallelism, Tokio for async I/O
- **Consequences:** Steeper learning curve, but 10-100x faster scraping

### Decision 2: Hexagonal Architecture
- **Date:** 2025-11-29
- **Context:** Need flexibility to swap browser engines, storage backends
- **Decision:** Ports & Adapters pattern with clear boundaries
- **Consequences:** More boilerplate, but highly testable and extensible

### Decision 3: Open Core Model
- **Date:** 2025-11-29
- **Context:** Need sustainable revenue while building community
- **Decision:** Core open source, premium cloud features
- **Consequences:** Community contributions, clear monetization path

### Decision 4: chromiumoxide over Playwright
- **Date:** 2025-11-29
- **Context:** Need native Rust browser automation
- **Decision:** chromiumoxide (pure Rust CDP implementation)
- **Consequences:** No Node.js dependency, better integration with Rayon

---

*Last updated by AI Agent: 2025-11-29*
