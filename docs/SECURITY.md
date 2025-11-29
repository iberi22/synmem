# 🔒 SynMem Security Guide

This document outlines security practices, threat models, and best practices for using and developing SynMem.

## Overview

SynMem handles sensitive data including:
- Browser session cookies and authentication tokens
- Browsing history and scraped content
- Personal conversations from AI chats
- Automated actions on authenticated sites

Security is a first-class concern throughout the architecture.

---

## Threat Model

### Assets to Protect

| Asset | Sensitivity | Protection Level |
|-------|-------------|------------------|
| Session cookies | High | Encrypted at rest |
| Authentication tokens | High | Encrypted at rest |
| Browsing history | Medium | Local storage only |
| Scraped content | Medium | Access controlled |
| Macro recordings | Low | Parameterized |

### Threat Actors

1. **Local attacker** - Access to the user's machine
2. **Network attacker** - Man-in-the-middle on network
3. **Malicious extension** - Third-party extensions
4. **Malicious website** - XSS, CSRF attacks

### Attack Vectors

| Vector | Mitigation |
|--------|------------|
| Credential theft | Encryption at rest, memory protection |
| Session hijacking | Secure cookie handling, HTTPS only |
| Data exfiltration | Local-only by default, no cloud sync |
| Privilege escalation | Minimal permissions, sandboxing |
| Replay attacks | Nonce-based authentication, timestamps |

---

## Credentials Security

### Encryption at Rest

All sensitive data is encrypted using AES-256-GCM:

```rust
use ring::{aead, rand};

pub struct CredentialStore {
    key: aead::LessSafeKey,
}

impl CredentialStore {
    pub fn new(master_password: &str) -> Result<Self> {
        // Derive key from password using Argon2id
        let key = derive_key(master_password)?;
        Ok(Self { key })
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let nonce = generate_nonce()?;
        let mut in_out = data.to_vec();
        
        self.key.seal_in_place_append_tag(
            aead::Nonce::assume_unique_for_key(nonce),
            aead::Aad::empty(),
            &mut in_out
        )?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.extend(in_out);
        Ok(result)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let (nonce, ciphertext) = data.split_at(12);
        let mut in_out = ciphertext.to_vec();
        
        self.key.open_in_place(
            aead::Nonce::assume_unique_for_key(nonce.try_into()?),
            aead::Aad::empty(),
            &mut in_out
        )?;
        
        // Remove authentication tag
        in_out.truncate(in_out.len() - 16);
        Ok(in_out)
    }
}
```

### Key Derivation

Master password is never stored; a key is derived using Argon2id:

```rust
use argon2::{Argon2, password_hash::SaltString};

fn derive_key(password: &str) -> Result<[u8; 32]> {
    let salt = get_or_create_salt()?;
    
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(
            65536,  // 64 MiB memory
            3,      // 3 iterations
            4,      // 4 parallel lanes
            Some(32) // 256-bit key
        )?
    );
    
    let mut key = [0u8; 32];
    argon2.hash_password_into(
        password.as_bytes(),
        salt.as_ref(),
        &mut key
    )?;
    
    Ok(key)
}
```

### Memory Protection

Sensitive data is zeroized when no longer needed:

```rust
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
struct SensitiveData {
    #[zeroize(drop)]
    password: String,
    #[zeroize(drop)]
    token: Vec<u8>,
}
```

---

## Cookie Security

### Secure Cookie Handling

```rust
pub struct CookieManager {
    store: CredentialStore,
}

impl CookieManager {
    pub fn store_cookies(&self, cookies: Vec<Cookie>) -> Result<()> {
        // Filter sensitive cookies
        let filtered: Vec<_> = cookies.into_iter()
            .filter(|c| self.should_store(c))
            .collect();
        
        // Encrypt and store
        let serialized = serde_json::to_vec(&filtered)?;
        let encrypted = self.store.encrypt(&serialized)?;
        
        self.save_to_disk(&encrypted)
    }

    fn should_store(&self, cookie: &Cookie) -> bool {
        // Don't store session-only cookies
        if cookie.session_only {
            return false;
        }
        
        // Don't store expired cookies
        if let Some(expires) = cookie.expires {
            if expires < Utc::now() {
                return false;
            }
        }
        
        true
    }
}
```

### Cookie Isolation

Cookies are isolated per profile:

```
~/.synmem/
├── profiles/
│   ├── default/
│   │   ├── cookies.enc      # Encrypted cookies
│   │   ├── storage.enc      # Encrypted local storage
│   │   └── metadata.json
│   └── work/
│       ├── cookies.enc
│       ├── storage.enc
│       └── metadata.json
└── master.key               # Protected by OS keychain
```

---

## Extension Security

### Permission Model

SynMem uses a minimal permissions model:

```json
{
  "manifest_version": 3,
  "permissions": [
    "activeTab",      // Only active tab, not all tabs
    "storage",        // Local storage only
    "nativeMessaging" // Communicate with native host
  ],
  "host_permissions": [
    // Explicitly listed domains only
    "https://*.twitter.com/*",
    "https://chat.openai.com/*"
  ],
  "optional_host_permissions": [
    "<all_urls>"      // Requires explicit user consent
  ]
}
```

### Content Script Isolation

Content scripts run in an isolated world:

```typescript
// Content script - isolated from page scripts
const pageData = document.querySelector('#content').textContent;

// Cannot access page's JavaScript variables
// window.pageVariable is undefined

// Communication only via postMessage
window.postMessage({ type: 'SYNMEM_DATA', data: pageData }, '*');
```

### Input Validation

All data from content scripts is validated:

```typescript
// Background script
function validateScrapeResult(data: unknown): data is ScrapeResult {
    if (!data || typeof data !== 'object') {
        return false;
    }
    
    const obj = data as Record<string, unknown>;
    
    return (
        typeof obj.url === 'string' &&
        typeof obj.title === 'string' &&
        typeof obj.content === 'string' &&
        isValidUrl(obj.url)
    );
}

function handleContentMessage(message: unknown) {
    if (!validateScrapeResult(message)) {
        console.error('Invalid scrape result');
        return;
    }
    
    // Safe to use message
    processValidatedResult(message);
}
```

---

## MCP Security

### Transport Security

Default: Local stdio transport (no network exposure)

```rust
// Default: stdio transport
let transport = StdioTransport::new();
server.serve(transport).await?;
```

Optional: TLS for remote connections

```rust
// Optional: TLS transport
let cert = load_certificate("server.crt")?;
let key = load_private_key("server.key")?;

let tls_config = ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth()
    .with_single_cert(cert, key)?;

let transport = TlsTransport::new(tls_config);
server.serve(transport).await?;
```

### Rate Limiting

All MCP tools are rate-limited:

```rust
use governor::{Quota, RateLimiter};

pub struct RateLimitedService<S> {
    inner: S,
    limiter: RateLimiter<String, _, _>,
}

impl<S: McpService> McpService for RateLimitedService<S> {
    async fn call_tool(&self, name: &str, args: Value) -> Result<Value> {
        // Check rate limit
        self.limiter.check_key(&name.to_string())
            .map_err(|_| Error::RateLimited)?;
        
        self.inner.call_tool(name, args).await
    }
}

// Configuration: 10 requests per second per tool
let quota = Quota::per_second(std::num::NonZeroU32::new(10).unwrap());
let limiter = RateLimiter::keyed(quota);
```

### Input Sanitization

All tool inputs are sanitized:

```rust
pub fn sanitize_url(url: &str) -> Result<Url> {
    let parsed = Url::parse(url)?;
    
    // Only allow http/https
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err(Error::InvalidScheme);
    }
    
    // Block internal/private IPs
    if is_private_ip(&parsed) {
        return Err(Error::PrivateIpBlocked);
    }
    
    Ok(parsed)
}

fn is_private_ip(url: &Url) -> bool {
    if let Some(host) = url.host_str() {
        if let Ok(ip) = host.parse::<IpAddr>() {
            return ip.is_loopback() || ip.is_private();
        }
    }
    false
}
```

---

## Data Privacy

### Local-Only by Default

SynMem stores all data locally:

- No telemetry or analytics
- No cloud synchronization (unless explicitly enabled)
- No external API calls (except for opted-in features)

### Data Retention

Users control data retention:

```rust
pub struct RetentionPolicy {
    pub max_age_days: Option<u32>,
    pub max_entries: Option<usize>,
    pub excluded_domains: Vec<String>,
}

impl Storage {
    pub async fn apply_retention(&self, policy: &RetentionPolicy) -> Result<()> {
        // Delete old entries
        if let Some(max_age) = policy.max_age_days {
            let cutoff = Utc::now() - Duration::days(max_age as i64);
            self.delete_before(cutoff).await?;
        }
        
        // Limit total entries
        if let Some(max_entries) = policy.max_entries {
            self.truncate_to(max_entries).await?;
        }
        
        Ok(())
    }
}
```

### Data Export and Deletion

Users can export or delete their data:

```rust
impl Storage {
    pub async fn export_all(&self) -> Result<ExportData> {
        Ok(ExportData {
            pages: self.get_all_pages().await?,
            sessions: self.get_all_sessions().await?,
            macros: self.get_all_macros().await?,
            settings: self.get_settings().await?,
        })
    }
    
    pub async fn delete_all(&self) -> Result<()> {
        self.clear_pages().await?;
        self.clear_sessions().await?;
        self.clear_macros().await?;
        
        // Securely overwrite files
        secure_delete(&self.db_path)?;
        
        Ok(())
    }
}
```

---

## Logging Security

### Sensitive Data Filtering

Logs never contain sensitive data:

```rust
use tracing::instrument;

#[instrument(skip(password, cookies))]
pub async fn login(
    username: &str,
    password: &str,  // Skipped from logs
    cookies: &[Cookie],  // Skipped from logs
) -> Result<Session> {
    tracing::info!(username, "Attempting login");
    // ...
}
```

### Log Redaction

Automatic redaction of sensitive patterns:

```rust
pub struct RedactingFormatter;

impl FormatEvent for RedactingFormatter {
    fn format_event(&self, event: &Event<'_>) -> String {
        let formatted = default_format(event);
        
        // Redact patterns
        let redacted = COOKIE_REGEX.replace_all(&formatted, "[REDACTED]");
        let redacted = TOKEN_REGEX.replace_all(&redacted, "[REDACTED]");
        let redacted = PASSWORD_REGEX.replace_all(&redacted, "[REDACTED]");
        
        redacted.to_string()
    }
}
```

---

## Security Best Practices

### For Users

1. **Use a strong master password** - At least 12 characters with mixed types
2. **Enable disk encryption** - Protect against physical access
3. **Review extension permissions** - Only grant necessary permissions
4. **Regular backups** - Export data regularly
5. **Update regularly** - Security patches are important

### For Developers

1. **Never log sensitive data** - Use `#[instrument(skip)]`
2. **Validate all inputs** - Never trust external data
3. **Use constant-time comparisons** - Prevent timing attacks
4. **Minimize permissions** - Request only what's needed
5. **Security review** - All PRs reviewed for security

---

## Vulnerability Reporting

### Responsible Disclosure

If you discover a security vulnerability:

1. **Do not** open a public issue
2. Email: security@synmem.dev
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### Response Timeline

| Severity | Initial Response | Fix Target |
|----------|------------------|------------|
| Critical | 24 hours | 7 days |
| High | 48 hours | 14 days |
| Medium | 7 days | 30 days |
| Low | 14 days | 60 days |

---

## Security Checklist

### Before Release

- [ ] All dependencies audited (`cargo audit`)
- [ ] No secrets in code or config
- [ ] Input validation on all endpoints
- [ ] Rate limiting enabled
- [ ] Encryption at rest verified
- [ ] Memory zeroization confirmed
- [ ] Logging reviewed for sensitive data
- [ ] Extension permissions minimized
- [ ] Security documentation updated

### Ongoing

- [ ] Weekly dependency audit
- [ ] Monthly security review
- [ ] Quarterly penetration testing
- [ ] Annual security audit
