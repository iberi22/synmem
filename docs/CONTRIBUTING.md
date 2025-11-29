# 🤝 Contributing to SynMem

Thank you for your interest in contributing to SynMem! This guide will help you get started.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please:

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Give constructive feedback
- Focus on what's best for the community

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) 1.70+
- [Node.js](https://nodejs.org/) 18+ (for extension)
- [Git](https://git-scm.com/)
- [GitHub CLI](https://cli.github.com/) (recommended)

### Setup

1. Fork the repository on GitHub

2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/synmem.git
   cd synmem
   ```

3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/iberi22/synmem.git
   ```

4. Build the project:
   ```bash
   cargo build
   ```

5. Run tests:
   ```bash
   cargo test
   ```

---

## Development Workflow

### Branch Naming

Use descriptive branch names:

```bash
# Features
git checkout -b feat/add-twitter-scraper

# Bug fixes
git checkout -b fix/cookie-encryption-issue

# Documentation
git checkout -b docs/update-installation-guide

# Refactoring
git checkout -b refactor/simplify-navigation-service
```

### Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting (no code change)
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Maintenance tasks

Examples:
```
feat(scraper): add support for LinkedIn profiles
fix(storage): handle corrupted cookie files gracefully
docs(readme): add installation instructions for macOS
```

### Pull Requests

1. **Create an issue first** (for significant changes)
   ```bash
   gh issue create --title "FEAT: Add LinkedIn scraper" --body "Description..."
   ```

2. **Reference the issue in your PR**
   ```
   feat(scraper): add LinkedIn profile scraper

   Closes #123
   ```

3. **Keep PRs focused** - One feature/fix per PR

4. **Write tests** - All new code should have tests

5. **Update documentation** - If your change affects the API

---

## Code Standards

### Rust

#### Formatting

Use `rustfmt` with default settings:

```bash
cargo fmt
```

#### Linting

Use `clippy` for linting:

```bash
cargo clippy -- -D warnings
```

#### Documentation

Document all public APIs:

```rust
/// Navigates the browser to the specified URL.
///
/// # Arguments
///
/// * `url` - The URL to navigate to
/// * `options` - Optional navigation settings
///
/// # Returns
///
/// A `NavigationResult` containing the final URL and page metadata.
///
/// # Errors
///
/// Returns an error if:
/// - The URL is invalid
/// - Navigation times out
/// - The page fails to load
///
/// # Examples
///
/// ```rust
/// let result = navigator.navigate_to("https://example.com", None).await?;
/// assert_eq!(result.status, 200);
/// ```
pub async fn navigate_to(&self, url: &str, options: Option<NavOptions>) -> Result<NavigationResult> {
    // implementation
}
```

#### Error Handling

Use `thiserror` for custom errors:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NavigationError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Navigation timeout after {0}ms")]
    Timeout(u64),

    #[error("Page load failed: {0}")]
    LoadFailed(String),

    #[error(transparent)]
    Browser(#[from] chromiumoxide::error::Error),
}
```

### TypeScript (Extension)

#### Formatting

Use Prettier:

```bash
cd extension
npm run format
```

#### Linting

Use ESLint:

```bash
cd extension
npm run lint
```

#### Type Safety

Always use strict TypeScript:

```typescript
// Good - explicit types
function processMessage(message: ExtensionMessage): ExtensionResponse {
    // ...
}

// Avoid - any types
function processMessage(message: any): any {
    // ...
}
```

---

## Testing

### Unit Tests

Write unit tests for all business logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_sanitization() {
        assert!(sanitize_url("https://example.com").is_ok());
        assert!(sanitize_url("javascript:alert(1)").is_err());
        assert!(sanitize_url("file:///etc/passwd").is_err());
    }

    #[tokio::test]
    async fn test_navigation() {
        let driver = MockBrowserDriver::new();
        let navigator = NavigationService::new(driver);

        let result = navigator.navigate_to("https://example.com", None).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

Place integration tests in `tests/`:

```rust
// tests/integration/scraping.rs

#[tokio::test]
async fn test_full_scraping_workflow() {
    let app = TestApp::new().await;

    // Navigate
    app.navigate("https://httpbin.org/html").await.unwrap();

    // Scrape
    let result = app.scrape_page(None).await.unwrap();

    // Verify
    assert!(!result.content.is_empty());
    assert!(!result.links.is_empty());
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_url_sanitization

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test integration
```

---

## Project Structure

```
synmem/
├── crates/
│   ├── synmem-core/        # Domain + Application Layer
│   │   ├── src/
│   │   │   ├── domain/     # Entities, Services
│   │   │   ├── ports/      # Port interfaces
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── synmem-browser/     # Browser Driver Adapter
│   ├── synmem-storage/     # Storage Adapter
│   ├── synmem-mcp/         # MCP Server Adapter
│   └── synmem-cli/         # CLI Binary
│
├── extension/              # Chrome Extension
│   ├── src/
│   │   ├── background/
│   │   ├── content/
│   │   └── popup/
│   └── package.json
│
├── docs/                   # Documentation
├── tests/                  # Integration tests
└── scripts/                # Build/setup scripts
```

---

## Adding New Features

### New MCP Tool

1. **Define the tool in domain**:
   ```rust
   // crates/synmem-core/src/domain/services/mod.rs
   pub async fn my_new_tool(&self, params: MyParams) -> Result<MyResult>;
   ```

2. **Add to inbound port**:
   ```rust
   // crates/synmem-core/src/ports/inbound/mod.rs
   async fn my_new_tool(&self, params: MyParams) -> Result<MyResult>;
   ```

3. **Implement MCP handler**:
   ```rust
   // crates/synmem-mcp/src/tools/mod.rs
   pub async fn handle_my_new_tool(args: Value) -> Result<Value>;
   ```

4. **Add tests**:
   ```rust
   #[tokio::test]
   async fn test_my_new_tool() {
       // ...
   }
   ```

5. **Update documentation**:
   - Add to `docs/MCP_TOOLS.md`
   - Update examples if applicable

### New Site Scraper

1. **Create scraper file**:
   ```typescript
   // extension/src/content/scrapers/mysite.ts
   export async function scrapeMySite(): Promise<MySiteData> {
       // ...
   }
   ```

2. **Register in content script**:
   ```typescript
   // extension/src/content/index.ts
   import { scrapeMySite } from './scrapers/mysite';
   
   if (window.location.hostname.includes('mysite.com')) {
       registerScraper('mysite', scrapeMySite);
   }
   ```

3. **Add host permission**:
   ```json
   // extension/manifest.json
   "host_permissions": [
       "https://*.mysite.com/*"
   ]
   ```

4. **Document**:
   - Add to `docs/EXTENSION_API.md`
   - Add example in `docs/examples/`

---

## Issue Labels

| Label | Description |
|-------|-------------|
| `bug` | Something isn't working |
| `enhancement` | New feature request |
| `documentation` | Documentation improvements |
| `good first issue` | Good for newcomers |
| `help wanted` | Extra attention needed |
| `security` | Security-related |
| `performance` | Performance improvements |

---

## Review Process

1. **Automated checks** must pass (CI, linting, tests)
2. **One approval** required from maintainer
3. **No merge conflicts**
4. **Documentation updated** (if applicable)
5. **Tests added** (for new features)

---

## Getting Help

- **Questions**: Open a [Discussion](https://github.com/iberi22/synmem/discussions)
- **Bugs**: Open an [Issue](https://github.com/iberi22/synmem/issues)
- **Security**: Email security@synmem.dev

---

## Recognition

All contributors are recognized in:
- `CONTRIBUTORS.md`
- Release notes
- GitHub contributors page

Thank you for contributing to SynMem! 🎉
