# 🏢 SynMem Enterprise Features

> Enterprise-grade features for organizations requiring advanced security, compliance, and management capabilities.

---

## 📋 Overview

SynMem Enterprise provides additional features designed for organizations that need:
- Single Sign-On (SSO) integration
- Comprehensive audit logging
- Team management and collaboration
- Compliance with industry standards
- On-premise deployment options

---

## 🔐 SSO Integration

### Supported Providers

| Provider | Protocol | Status |
|----------|----------|--------|
| SAML 2.0 | SAML | ✅ Supported |
| OIDC | OpenID Connect | ✅ Supported |
| Azure AD | OIDC/SAML | ✅ Supported |
| Okta | OIDC/SAML | ✅ Supported |
| Google Workspace | OIDC | ✅ Supported |

### Configuration

```rust
use serde::{Deserialize, Serialize};

/// SSO Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SsoProvider {
    Saml2 {
        entity_id: String,
        sso_url: String,
        certificate: String,
        attribute_mapping: AttributeMapping,
    },
    Oidc {
        client_id: String,
        client_secret: String,
        issuer_url: String,
        scopes: Vec<String>,
    },
    AzureAd {
        tenant_id: String,
        client_id: String,
        client_secret: String,
    },
    Okta {
        domain: String,
        client_id: String,
        client_secret: String,
    },
    GoogleWorkspace {
        client_id: String,
        client_secret: String,
        hosted_domain: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeMapping {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub groups: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoConfig {
    pub provider: SsoProvider,
    pub enabled: bool,
    pub enforce_sso: bool,
    pub allowed_domains: Vec<String>,
    pub default_role: UserRole,
    pub auto_provision_users: bool,
}
```

### SAML 2.0 Setup

1. Configure your Identity Provider (IdP) with SynMem's Service Provider (SP) metadata
2. Import the IdP metadata or manually configure:
   - Entity ID
   - SSO URL
   - X.509 Certificate
3. Map user attributes (email, name, groups)

### OIDC Setup

1. Register SynMem as an OAuth 2.0 client in your IdP
2. Configure the callback URL: `https://your-domain.com/auth/callback`
3. Set required scopes: `openid`, `email`, `profile`

---

## 📝 Audit Logging

### Audit Log Structure

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Comprehensive audit log entry for enterprise compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// Unique identifier for the log entry
    pub id: Uuid,
    /// Timestamp when the action occurred
    pub timestamp: DateTime<Utc>,
    /// User who performed the action
    pub user_id: String,
    /// Optional user email for display
    pub user_email: Option<String>,
    /// Type of action performed
    pub action: AuditAction,
    /// Resource that was affected
    pub resource: AuditResource,
    /// Client IP address
    pub ip_address: String,
    /// User agent string from the client
    pub user_agent: String,
    /// Session ID associated with this action
    pub session_id: Option<String>,
    /// Workspace context
    pub workspace_id: Option<String>,
    /// Additional metadata
    pub metadata: Option<serde_json::Value>,
    /// Whether the action was successful
    pub success: bool,
    /// Error message if action failed
    pub error_message: Option<String>,
}

/// Types of auditable actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    // Authentication events
    Login,
    Logout,
    LoginFailed,
    PasswordChange,
    MfaEnabled,
    MfaDisabled,
    
    // Resource operations
    Create,
    Read,
    Update,
    Delete,
    Export,
    Import,
    
    // Session management
    SessionStart,
    SessionEnd,
    SessionShare,
    
    // Admin operations
    UserInvite,
    UserRemove,
    RoleChange,
    SettingsChange,
    
    // API operations
    ApiKeyCreate,
    ApiKeyRevoke,
    WebhookCreate,
    WebhookDelete,
}

/// Types of resources that can be audited
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResource {
    User { id: String },
    Session { id: String },
    Workspace { id: String },
    ApiKey { id: String },
    Settings { category: String },
    Memory { id: String },
    BrowserTask { id: String },
}

/// Audit log query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogQuery {
    pub user_id: Option<String>,
    pub workspace_id: Option<String>,
    pub action: Option<AuditAction>,
    pub resource_type: Option<String>,
    pub from_timestamp: Option<DateTime<Utc>>,
    pub to_timestamp: Option<DateTime<Utc>>,
    pub success_only: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

/// Trait for audit log storage implementations
#[async_trait::async_trait]
pub trait AuditLogRepository: Send + Sync {
    /// Log an audit event
    async fn log(&self, entry: AuditLog) -> Result<(), AuditError>;
    
    /// Query audit logs with filters
    async fn query(&self, query: AuditLogQuery) -> Result<Vec<AuditLog>, AuditError>;
    
    /// Export audit logs for compliance
    async fn export(&self, query: AuditLogQuery, format: ExportFormat) -> Result<Vec<u8>, AuditError>;
    
    /// Purge old audit logs based on retention policy
    async fn purge(&self, before: DateTime<Utc>) -> Result<u64, AuditError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Siem, // For SIEM integration (CEF/LEEF format)
}

#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Query error: {0}")]
    Query(String),
    #[error("Export error: {0}")]
    Export(String),
}
```

### Retention Policies

| Plan | Retention Period | Export Options |
|------|-----------------|----------------|
| Enterprise | 7 years | JSON, CSV, SIEM |
| Business | 1 year | JSON, CSV |
| Team | 90 days | JSON |

### SIEM Integration

SynMem supports exporting audit logs to popular SIEM platforms:
- Splunk
- Elastic SIEM
- Microsoft Sentinel
- Datadog
- Sumo Logic

---

## 👥 Team Management

### Workspaces

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Workspace for team collaboration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub owner_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settings: WorkspaceSettings,
    pub billing: BillingInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSettings {
    /// SSO configuration for this workspace
    pub sso_config: Option<SsoConfig>,
    /// Whether to enforce SSO for all users
    pub enforce_sso: bool,
    /// Allowed email domains for workspace membership
    pub allowed_domains: Vec<String>,
    /// Default role for new members
    pub default_member_role: UserRole,
    /// Data residency region
    pub data_residency: DataResidency,
    /// Audit log retention in days
    pub audit_log_retention_days: u32,
    /// Whether to allow session sharing
    pub allow_session_sharing: bool,
    /// IP allowlist (empty means all allowed)
    pub ip_allowlist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataResidency {
    Us,
    Eu,
    AsiaPacific,
    Custom { region: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInfo {
    pub plan: BillingPlan,
    pub seats: u32,
    pub billing_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingPlan {
    Team,
    Business,
    Enterprise,
}
```

### Role-Based Access Control (RBAC)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// User roles with hierarchical permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    /// Full access to all features and settings
    Owner,
    /// Can manage users, settings, and all resources
    Admin,
    /// Can create, edit, and delete own resources
    Member,
    /// Can view resources but not modify
    Viewer,
    /// Custom role with specific permissions
    Custom,
}

/// Fine-grained permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // Session permissions
    SessionCreate,
    SessionRead,
    SessionUpdate,
    SessionDelete,
    SessionShare,
    
    // Memory permissions
    MemoryCreate,
    MemoryRead,
    MemoryUpdate,
    MemoryDelete,
    MemoryExport,
    
    // User management
    UserInvite,
    UserRemove,
    UserRoleChange,
    
    // Workspace settings
    SettingsRead,
    SettingsUpdate,
    
    // Billing
    BillingRead,
    BillingUpdate,
    
    // API
    ApiKeyCreate,
    ApiKeyRevoke,
    
    // Audit
    AuditLogRead,
    AuditLogExport,
}

/// Workspace member with role and permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMember {
    pub user_id: String,
    pub workspace_id: Uuid,
    pub role: UserRole,
    pub custom_permissions: Option<HashSet<Permission>>,
    pub joined_at: DateTime<Utc>,
    pub invited_by: Option<String>,
}

impl UserRole {
    /// Get default permissions for a role
    pub fn default_permissions(&self) -> HashSet<Permission> {
        match self {
            UserRole::Owner | UserRole::Admin => {
                // All permissions
                use Permission::*;
                [
                    SessionCreate, SessionRead, SessionUpdate, SessionDelete, SessionShare,
                    MemoryCreate, MemoryRead, MemoryUpdate, MemoryDelete, MemoryExport,
                    UserInvite, UserRemove, UserRoleChange,
                    SettingsRead, SettingsUpdate,
                    BillingRead, BillingUpdate,
                    ApiKeyCreate, ApiKeyRevoke,
                    AuditLogRead, AuditLogExport,
                ].into_iter().collect()
            }
            UserRole::Member => {
                use Permission::*;
                [
                    SessionCreate, SessionRead, SessionUpdate, SessionDelete,
                    MemoryCreate, MemoryRead, MemoryUpdate, MemoryDelete,
                ].into_iter().collect()
            }
            UserRole::Viewer => {
                use Permission::*;
                [SessionRead, MemoryRead].into_iter().collect()
            }
            UserRole::Custom => HashSet::new(),
        }
    }
}
```

### Shared Sessions

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Shared session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSession {
    pub id: Uuid,
    pub session_id: Uuid,
    pub workspace_id: Uuid,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub access_level: SharedAccessLevel,
    pub shared_with: Vec<SharedWith>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharedAccessLevel {
    /// Can view the session
    ReadOnly,
    /// Can view and interact
    ReadWrite,
    /// Can view, interact, and share with others
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharedWith {
    /// Share with specific user
    User { user_id: String },
    /// Share with entire workspace
    Workspace,
    /// Share with specific role
    Role { role: UserRole },
    /// Share via link (optionally password protected)
    Link { 
        token: String, 
        password_hash: Option<String>,
    },
}
```

---

## ✅ Compliance

### SOC 2 Type 1

SynMem Enterprise is designed with SOC 2 compliance in mind:

| Trust Service Criteria | Implementation |
|-----------------------|----------------|
| Security | Encryption at rest (AES-256), TLS 1.3 in transit |
| Availability | 99.9% SLA with multi-region failover |
| Processing Integrity | Comprehensive audit logging |
| Confidentiality | Role-based access control, data isolation |
| Privacy | GDPR-compliant data handling |

### GDPR Compliance

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// GDPR data subject rights support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdprRequest {
    pub id: Uuid,
    pub request_type: GdprRequestType,
    pub user_id: String,
    pub user_email: String,
    pub requested_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: GdprRequestStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GdprRequestType {
    /// Right to access (Article 15)
    DataAccess,
    /// Right to rectification (Article 16)
    DataRectification,
    /// Right to erasure (Article 17)
    DataErasure,
    /// Right to data portability (Article 20)
    DataPortability,
    /// Right to restrict processing (Article 18)
    ProcessingRestriction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GdprRequestStatus {
    Pending,
    InProgress,
    Completed,
    Rejected { reason: String },
}

/// Data Processing Agreement information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessingAgreement {
    pub workspace_id: Uuid,
    pub signed_at: DateTime<Utc>,
    pub signed_by: String,
    pub version: String,
    pub dpa_document_url: String,
}
```

### Data Residency Options

| Region | Data Center Locations | Compliance |
|--------|----------------------|------------|
| US | us-east-1, us-west-2 | SOC 2, HIPAA eligible |
| EU | eu-west-1, eu-central-1 | GDPR, SOC 2 |
| Asia Pacific | ap-northeast-1, ap-southeast-1 | SOC 2 |

---

## 📊 SLA (Service Level Agreement)

### Uptime Guarantee

| Plan | Uptime SLA | Monthly Credit |
|------|------------|----------------|
| Enterprise | 99.9% | Yes |
| Business | 99.5% | Yes |
| Team | 99.0% | No |

### Support Tiers

| Feature | Team | Business | Enterprise |
|---------|------|----------|------------|
| Email Support | ✅ 48h | ✅ 24h | ✅ 4h |
| Chat Support | ❌ | ✅ Business hours | ✅ 24/7 |
| Phone Support | ❌ | ❌ | ✅ 24/7 |
| Dedicated Account Manager | ❌ | ❌ | ✅ |
| Custom Training | ❌ | ❌ | ✅ |
| Priority Bug Fixes | ❌ | ✅ | ✅ |
| Custom Feature Development | ❌ | ❌ | ✅ |

### SLA Credits

```rust
use serde::{Deserialize, Serialize};

/// SLA credit calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaCredit {
    /// Monthly uptime percentage
    pub uptime_percentage: f64,
    /// Credit percentage of monthly bill
    pub credit_percentage: u8,
}

impl SlaCredit {
    pub fn calculate(uptime: f64) -> u8 {
        match uptime {
            u if u >= 99.9 => 0,
            u if u >= 99.0 => 10,
            u if u >= 95.0 => 25,
            _ => 50,
        }
    }
}
```

---

## 🐳 On-Premise Deployment

### Docker Deployment

```yaml
# docker-compose.enterprise.yml
version: '3.8'

services:
  synmem-server:
    image: synmem/enterprise:latest
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://synmem:password@db:5432/synmem
      - REDIS_URL=redis://redis:6379
      - SSO_ENABLED=true
      - ENCRYPTION_KEY=${ENCRYPTION_KEY}
    volumes:
      - ./config:/app/config
      - ./data:/app/data
    depends_on:
      - db
      - redis

  synmem-worker:
    image: synmem/enterprise:latest
    command: worker
    environment:
      - DATABASE_URL=postgres://synmem:password@db:5432/synmem
      - REDIS_URL=redis://redis:6379
    depends_on:
      - db
      - redis

  db:
    image: postgres:15
    environment:
      - POSTGRES_USER=synmem
      - POSTGRES_PASSWORD=${DB_PASSWORD}
      - POSTGRES_DB=synmem
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

### Kubernetes Deployment

```yaml
# helm values for enterprise deployment
replicaCount: 3

image:
  repository: synmem/enterprise
  tag: latest

ingress:
  enabled: true
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
  hosts:
    - host: synmem.your-domain.com
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: synmem-tls
      hosts:
        - synmem.your-domain.com

resources:
  limits:
    cpu: 2000m
    memory: 4Gi
  requests:
    cpu: 500m
    memory: 1Gi

postgresql:
  enabled: true
  auth:
    postgresPassword: "${DB_PASSWORD}"
    database: synmem

redis:
  enabled: true
  architecture: standalone
```

### System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 4 cores | 8+ cores |
| RAM | 8 GB | 16+ GB |
| Storage | 50 GB SSD | 200+ GB SSD |
| Database | PostgreSQL 13+ | PostgreSQL 15+ |
| Cache | Redis 6+ | Redis 7+ |

### Air-Gapped Installation

For environments without internet access:

1. Download offline bundle from enterprise portal
2. Import Docker images: `docker load -i synmem-enterprise-bundle.tar`
3. Configure local registry
4. Deploy using provided scripts

---

## 🔧 Configuration Reference

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `DATABASE_URL` | PostgreSQL connection string | Yes |
| `REDIS_URL` | Redis connection string | Yes |
| `ENCRYPTION_KEY` | Master encryption key (32 bytes, base64) | Yes |
| `SSO_ENABLED` | Enable SSO authentication | No |
| `AUDIT_LOG_ENABLED` | Enable audit logging | No |
| `DATA_RESIDENCY` | Data residency region | No |

---

## 📞 Contact

For enterprise inquiries:
- Email: enterprise@synmem.io
- Sales: sales@synmem.io

---

*Last updated: 2025-11-29*
