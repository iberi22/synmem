# 🧠 SynMem - Synthetic Memory Browser Agent

[![CI](https://github.com/iberi22/synmem/actions/workflows/ci.yml/badge.svg)](https://github.com/iberi22/synmem/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

> **The most advanced synthetic memory system for AI agents** - Navigate, scrape, and automate the web with your authenticated sessions, all exposed via MCP.

---

## ✨ Features

- 🌐 **Browser Automation** - Navigate, click, type, scroll with full browser control
- 📊 **Smart Scraping** - Extract structured content with site-specific scrapers
- 💾 **Semantic Memory** - Store and search browsing history with embeddings
- 🤖 **MCP Integration** - Works seamlessly with Claude Desktop and other MCP clients
- 🔌 **Chrome Extension** - Capture content from authenticated sessions
- ⚡ **High Performance** - Rust-powered with parallel processing via Rayon

---

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/iberi22/synmem.git
cd synmem

# Build with Rust
cargo build --release
```

### Configure Claude Desktop

Add to your Claude Desktop config (`~/.config/claude/config.json`):

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

### Start Using

Once connected, you can use natural language commands like:

```
Navigate to https://news.ycombinator.com and get me the top stories
```

```
Save my ChatGPT conversation about Rust async programming
```

```
Search my memory for articles about database migrations
```

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [Quick Start](docs/README.md) | Get up and running in minutes |
| [Installation](docs/INSTALLATION.md) | Detailed installation guide (Windows focus) |
| [MCP Tools](docs/MCP_TOOLS.md) | Complete MCP tools reference |
| [Extension API](docs/EXTENSION_API.md) | Chrome extension documentation |
| [Architecture](docs/ARCHITECTURE.md) | Technical deep-dive |
| [Security](docs/SECURITY.md) | Security practices and guidelines |
| [Contributing](docs/CONTRIBUTING.md) | How to contribute |

### Examples

- [Basic Scraping](docs/examples/basic_scraping.md) - Extract content from websites
- [Twitter Automation](docs/examples/twitter_automation.md) - Automate Twitter interactions
- [Chat Capture](docs/examples/chat_capture.md) - Capture AI chat conversations

---

## 🛠️ MCP Tools

SynMem exposes powerful tools via MCP:

### Navigation
- `navigate_to` - Go to any URL
- `click` - Click on elements
- `type_text` - Type into inputs
- `screenshot` - Capture screenshots

### Scraping
- `scrape_page` - Extract structured content
- `scrape_chat` - Capture AI conversations
- `extract_links` - Get all page links
- `extract_text` - Get clean text

### Memory
- `search_memory` - Semantic search across history
- `get_recent` - Recent pages and chats
- `save_context` - Bookmark current context

### Automation
- `record_macro` - Record action sequences
- `play_macro` - Replay recorded macros
- `fill_form` - Auto-fill forms

[See full MCP Tools Reference →](docs/MCP_TOOLS.md)

---

## 🏗️ Architecture

SynMem uses **Hexagonal Architecture** (Ports & Adapters) for flexibility and testability:

```
┌─────────────────────────────────────────────────────────────┐
│                        SYNMEM CORE                          │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              DOMAIN LAYER (Services)                   │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │           APPLICATION LAYER (Ports)                    │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
         │                    │                    │
    MCP Server          Extension            REST API
    (Primary)           (Primary)           (Primary)
         │                    │                    │
    Chromium             SQLite              Embeddings
    (Secondary)         (Secondary)         (Secondary)
```

[See Architecture Documentation →](docs/ARCHITECTURE.md)

---

## 🔒 Security

- **Encrypted at rest** - Cookies and tokens encrypted with AES-256-GCM
- **Local-first** - All data stays on your machine by default
- **Minimal permissions** - Extension uses only necessary permissions
- **Rate limiting** - Built-in protection against abuse

[See Security Documentation →](docs/SECURITY.md)

---

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](docs/CONTRIBUTING.md) for details.

```bash
# Setup development environment
git clone https://github.com/iberi22/synmem.git
cd synmem
cargo build
cargo test
```

---

## 📄 License

Apache 2.0 - See [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

Built with:
- [chromiumoxide](https://github.com/mattsse/chromiumoxide) - Browser automation
- [tokio](https://tokio.rs/) - Async runtime
- [rayon](https://github.com/rayon-rs/rayon) - Parallel processing
- [MCP Protocol](https://modelcontextprotocol.io/) - AI integration

---

**Created with 🧠 by [@iberi22](https://github.com/iberi22)**

---

# 📜 Git-Core Protocol

This project follows the **Git-Core Protocol** for AI-assisted development.

## 🤔 Why This Approach?

| Problem | Git-Core Solution |
|---------|-------------------|
| AI "forgets" task state | State in GitHub Issues (persistent) |
| Context grows = more tokens = more cost | Only load current issue + architecture |
| Messy TODO.md files | Organized GitHub board |
| Ecosystem dependency (NPM, etc.) | Language-agnostic bash/PowerShell scripts |

## 📦 Installation Options

### Option 1: Remote Installation (⚡ God Mode)

**Windows PowerShell:**
```powershell
# In your project folder
irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex

# Auto mode (for AI Agents)
$env:GIT_CORE_AUTO = "1"; $env:GIT_CORE_ORGANIZE = "1"; irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
```

**Linux/Mac:**
```bash
# In your project folder
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash

# Auto mode (for AI Agents)
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash -s -- --auto --organize
```

### Option 2: Use as Template

1. Click **"Use this template"** above
2. Clone your new repository
3. Run: `./scripts/init_project.sh` or `.\scripts\init_project.ps1`

## 📂 Structure

```
/
├── .ai/
│   ├── ARCHITECTURE.md       # 📖 System context
│   └── CONTEXT_LOG.md        # 📝 Ephemeral session notes
├── .github/
│   ├── copilot-instructions.md  # 🤖 GitHub Copilot rules
│   └── ISSUE_TEMPLATE/       # 📋 Issue templates
├── scripts/
│   ├── init_project.sh       # 🐧 Linux/Mac initializer
│   └── init_project.ps1      # 🪟 Windows initializer
├── AGENTS.md                 # 🤖 All AI agents config
├── .cursorrules              # 🎯 Cursor rules
└── .windsurfrules            # 🌊 Windsurf rules
```

## 🔄 The Workflow Loop

```
┌─────────────────────────────────────────────────────────┐
│                    THE LOOP                              │
├─────────────────────────────────────────────────────────┤
│   1. READ: cat .ai/ARCHITECTURE.md                      │
│           gh issue list --assignee "@me"                │
│   2. ACT:  gh issue edit <id> --add-assignee "@me"      │
│           git checkout -b feat/issue-<id>               │
│   3. UPDATE: git commit -m "feat: ... (closes #<id>)"   │
│             gh pr create --fill                         │
│   ↺ Repeat                                               │
└─────────────────────────────────────────────────────────┘
```

## 🤖 Compatible AI Agents

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT | ✅ Any LLM with terminal access

---

# 🇪🇸 Español

## 🚀 Inicio Rápido - ¡Solo Copia Este Prompt!

> **Copia este prompt a tu agente de código IA (Copilot, Cursor, Claude, etc.) para auto-configurar:**

```
Inicializa este proyecto con Git-Core Protocol. Ejecuta:
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
Luego ejecuta: .\scripts\init_project.ps1 -Auto
Después del setup, lee .ai/ARCHITECTURE.md y empieza con el primer issue de: gh issue list
```

Para Linux/Mac:
```
Inicializa este proyecto con Git-Core Protocol. Ejecuta:
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash -s -- --auto
Luego ejecuta: ./scripts/init_project.sh
Después del setup, lee .ai/ARCHITECTURE.md y empieza con el primer issue de: gh issue list
```

---

## 🤔 ¿Por Qué Este Enfoque?

| Problema | Solución Git-Core |
|----------|-------------------|
| La IA "olvida" el estado de tareas | Estado en GitHub Issues (persistente) |
| Contexto crece = más tokens = más costo | Solo cargar issue actual + arquitectura |
| Archivos TODO.md desordenados | Tablero GitHub organizado |
| Dependencia de ecosistema (NPM, etc.) | Scripts bash/PowerShell agnósticos |

## 📦 Opciones de Instalación

### Opción 1: Instalación Remota (⚡ Nivel Dios)

**Windows PowerShell:**
```powershell
# En tu carpeta de proyecto
irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex

# Modo automático (para AI Agents)
$env:GIT_CORE_AUTO = "1"; $env:GIT_CORE_ORGANIZE = "1"; irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
```

**Linux/Mac:**
```bash
# En tu carpeta de proyecto
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash

# Modo automático (para AI Agents)
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash -s -- --auto --organize
```

### Opción 2: Usar como Template

1. Click en **"Use this template"** arriba
2. Clona tu nuevo repositorio
3. Ejecuta: `./scripts/init_project.sh` o `.\scripts\init_project.ps1`

## 🗂️ Organización Automática

| Tipo de archivo | Destino |
|-----------------|---------|
| `*.md` (excepto README, AGENTS) | `docs/archive/` |
| `test_*.py`, `*.test.js` | `tests/` |
| `*.sh`, `*.bat` (scripts sueltos) | `scripts/` |

## 🏷️ Etiquetas Semánticas

| Label | Uso |
|-------|-----|
| `ai-plan` | Tareas de planificación |
| `ai-context` | Información crítica |
| `ai-blocked` | Requiere intervención humana |
| `in-progress` | Tarea en desarrollo |

---

# 🇧🇷 Português

## 🚀 Início Rápido - Apenas Copie Este Prompt!

> **Copie este prompt para seu agente de código IA (Copilot, Cursor, Claude, etc.) para auto-configurar:**

```
Inicialize este projeto com Git-Core Protocol. Execute:
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
Depois execute: .\scripts\init_project.ps1 -Auto
Após o setup, leia .ai/ARCHITECTURE.md e comece com a primeira issue de: gh issue list
```

Para Linux/Mac:
```
Inicialize este projeto com Git-Core Protocol. Execute:
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash -s -- --auto
Depois execute: ./scripts/init_project.sh
Após o setup, leia .ai/ARCHITECTURE.md e comece com a primeira issue de: gh issue list
```

---

## 🤔 Por Que Esta Abordagem?

| Problema | Solução Git-Core |
|----------|------------------|
| IA "esquece" o estado das tarefas | Estado no GitHub Issues (persistente) |
| Contexto cresce = mais tokens = mais custo | Carregar apenas issue atual + arquitetura |
| Arquivos TODO.md desorganizados | Quadro GitHub organizado |
| Dependência de ecossistema (NPM, etc.) | Scripts bash/PowerShell agnósticos |

## 📦 Opções de Instalação

### Opção 1: Instalação Remota (⚡ Modo Deus)

**Windows PowerShell:**
```powershell
irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
```

**Linux/Mac:**
```bash
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash
```

## 🤖 Agentes IA Compatíveis

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT

---

# 🇩🇪 Deutsch

## 🚀 Schnellstart - Kopiere Einfach Diesen Prompt!

> **Kopiere diesen Prompt zu deinem KI-Coding-Agenten (Copilot, Cursor, Claude, etc.) für Auto-Setup:**

```
Initialisiere dieses Projekt mit Git-Core Protocol. Führe aus:
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
Dann führe aus: .\scripts\init_project.ps1 -Auto
Nach dem Setup, lies .ai/ARCHITECTURE.md und beginne mit dem ersten Issue von: gh issue list
```

Für Linux/Mac:
```
Initialisiere dieses Projekt mit Git-Core Protocol. Führe aus:
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash -s -- --auto
Dann führe aus: ./scripts/init_project.sh
Nach dem Setup, lies .ai/ARCHITECTURE.md und beginne mit dem ersten Issue von: gh issue list
```

---

## 🤔 Warum Dieser Ansatz?

| Problem | Git-Core Lösung |
|---------|-----------------|
| KI "vergisst" Aufgabenstatus | Status in GitHub Issues (persistent) |
| Kontext wächst = mehr Tokens = mehr Kosten | Nur aktuelles Issue + Architektur laden |
| Unordentliche TODO.md Dateien | Organisiertes GitHub Board |
| Ökosystem-Abhängigkeit (NPM, etc.) | Sprachunabhängige bash/PowerShell Skripte |

## 📦 Installationsoptionen

**Windows PowerShell:**
```powershell
irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
```

**Linux/Mac:**
```bash
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash
```

## 🤖 Kompatible KI-Agenten

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT

---

# 🇫🇷 Français

## 🚀 Démarrage Rapide - Copiez Simplement Ce Prompt!

> **Copiez ce prompt vers votre agent de code IA (Copilot, Cursor, Claude, etc.) pour auto-configurer:**

```
Initialise ce projet avec Git-Core Protocol. Exécute:
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
Puis exécute: .\scripts\init_project.ps1 -Auto
Après le setup, lis .ai/ARCHITECTURE.md et commence avec la première issue de: gh issue list
```

Pour Linux/Mac:
```
Initialise ce projet avec Git-Core Protocol. Exécute:
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash -s -- --auto
Puis exécute: ./scripts/init_project.sh
Après le setup, lis .ai/ARCHITECTURE.md et commence avec la première issue de: gh issue list
```

---

## 🤔 Pourquoi Cette Approche?

| Problème | Solution Git-Core |
|----------|-------------------|
| L'IA "oublie" l'état des tâches | État dans GitHub Issues (persistant) |
| Contexte grandit = plus de tokens = plus de coût | Charger seulement l'issue actuelle + architecture |
| Fichiers TODO.md désordonnés | Tableau GitHub organisé |
| Dépendance d'écosystème (NPM, etc.) | Scripts bash/PowerShell agnostiques |

## 📦 Options d'Installation

**Windows PowerShell:**
```powershell
irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
```

**Linux/Mac:**
```bash
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash
```

## 🤖 Agents IA Compatibles

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT

---

# 🇯🇵 日本語

## 🚀 クイックスタート - このプロンプトをコピーするだけ！

> **AIコーディングエージェント（Copilot、Cursor、Claudeなど）にこのプロンプトをコピーして自動セットアップ：**

```
Git-Core Protocolでこのプロジェクトを初期化してください。実行：
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
次に実行: .\scripts\init_project.ps1 -Auto
セットアップ後、.ai/ARCHITECTURE.mdを読み、gh issue listから最初のissueを始めてください
```

Linux/Macの場合:
```
Git-Core Protocolでこのプロジェクトを初期化してください。実行：
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash -s -- --auto
次に実行: ./scripts/init_project.sh
セットアップ後、.ai/ARCHITECTURE.mdを読み、gh issue listから最初のissueを始めてください
```

---

## 🤔 なぜこのアプローチ？

| 問題 | Git-Core ソリューション |
|------|------------------------|
| AIがタスク状態を「忘れる」 | GitHub Issuesで状態管理（永続的） |
| コンテキスト増加 = トークン増 = コスト増 | 現在のissue + アーキテクチャのみロード |
| 乱雑なTODO.mdファイル | 整理されたGitHubボード |
| エコシステム依存（NPMなど） | 言語非依存のbash/PowerShellスクリプト |

## 📦 インストールオプション

**Windows PowerShell:**
```powershell
irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
```

**Linux/Mac:**
```bash
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash
```

## 🤖 対応AIエージェント

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT

---

# 🇨🇳 中文

## 🚀 快速开始 - 只需复制这个提示词！

> **将此提示词复制到您的AI编程助手（Copilot、Cursor、Claude等）以自动设置：**

```
使用Git-Core Protocol初始化此项目。执行：
$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
然后执行: .\scripts\init_project.ps1 -Auto
设置完成后，阅读.ai/ARCHITECTURE.md并从gh issue list开始第一个issue
```

Linux/Mac:
```
使用Git-Core Protocol初始化此项目。执行：
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash -s -- --auto
然后执行: ./scripts/init_project.sh
设置完成后，阅读.ai/ARCHITECTURE.md并从gh issue list开始第一个issue
```

---

## 🤔 为什么选择这种方法？

| 问题 | Git-Core 解决方案 |
|------|-------------------|
| AI"忘记"任务状态 | 状态存储在GitHub Issues（持久化） |
| 上下文增长 = 更多token = 更多成本 | 仅加载当前issue + 架构 |
| 混乱的TODO.md文件 | 有组织的GitHub看板 |
| 生态系统依赖（NPM等） | 语言无关的bash/PowerShell脚本 |

## 📦 安装选项

**Windows PowerShell:**
```powershell
irm https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.ps1 | iex
```

**Linux/Mac:**
```bash
curl -sL https://raw.githubusercontent.com/iberi22/ai-git-core-template/main/install.sh | bash
```

## 🤖 兼容的AI助手

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT

---

## 📋 Requirements | Requisitos | Requisitos | Anforderungen | Prérequis | 要件 | 要求

- [Git](https://git-scm.com/)
- [GitHub CLI](https://cli.github.com/) (`gh`) - authenticated | autenticado | authentifié | 認証済み | 已认证

---

## 📄 License | Licencia | Licença | Lizenz | Licence | ライセンス | 许可证

MIT - Use it however you want | Úsalo como quieras | Use como quiser | Verwende es wie du willst | Utilisez-le comme vous voulez | 好きなように使ってください | 随意使用

---

**Created with 🧠 by [@iberi22](https://github.com/iberi22)**
