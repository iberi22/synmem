//! Install tracking entity.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Record of a scraper installation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallRecord {
    /// Unique identifier
    pub id: Uuid,
    /// ID of the installed package
    pub package_id: Uuid,
    /// Username of the installer
    pub user: String,
    /// Version installed
    pub version: String,
    /// Date of installation
    pub installed_at: DateTime<Utc>,
    /// Whether the install is currently active
    pub is_active: bool,
    /// Date of uninstallation (if applicable)
    pub uninstalled_at: Option<DateTime<Utc>>,
}

impl InstallRecord {
    /// Creates a new install record.
    pub fn new(package_id: Uuid, user: String, version: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            package_id,
            user,
            version,
            installed_at: Utc::now(),
            is_active: true,
            uninstalled_at: None,
        }
    }

    /// Marks the installation as uninstalled.
    pub fn uninstall(&mut self) {
        self.is_active = false;
        self.uninstalled_at = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_record() {
        let record = InstallRecord::new(Uuid::new_v4(), "alice".to_string(), "1.0.0".to_string());

        assert!(record.is_active);
        assert!(record.uninstalled_at.is_none());
    }

    #[test]
    fn test_uninstall() {
        let mut record =
            InstallRecord::new(Uuid::new_v4(), "bob".to_string(), "2.0.0".to_string());

        record.uninstall();

        assert!(!record.is_active);
        assert!(record.uninstalled_at.is_some());
    }
}
