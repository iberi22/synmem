//! Payout record entity for revenue distribution.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a payout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    /// Payout is pending processing
    Pending,
    /// Payout is being processed
    Processing,
    /// Payout completed successfully
    Completed,
    /// Payout failed
    Failed,
    /// Payout was cancelled
    Cancelled,
}

impl Default for PayoutStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Record of a payout to a scraper creator.
///
/// Revenue model: 70% to creator, 30% to SynMem platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutRecord {
    /// Unique identifier
    pub id: Uuid,
    /// Creator username
    pub creator: String,
    /// Package ID that generated the revenue
    pub package_id: Uuid,
    /// Sale amount (full price paid by user)
    pub sale_amount: f64,
    /// Creator payout amount (70%)
    pub creator_amount: f64,
    /// Platform fee (30%)
    pub platform_fee: f64,
    /// Payout status
    pub status: PayoutStatus,
    /// Date the sale occurred
    pub sale_date: DateTime<Utc>,
    /// Date the payout was processed
    pub processed_at: Option<DateTime<Utc>>,
    /// Transaction ID from payment processor
    pub transaction_id: Option<String>,
    /// Optional notes
    pub notes: Option<String>,
}

impl PayoutRecord {
    /// Revenue share percentage for creators.
    pub const CREATOR_SHARE: f64 = 0.70;
    /// Revenue share percentage for the platform.
    pub const PLATFORM_SHARE: f64 = 0.30;

    /// Creates a new payout record from a sale.
    pub fn from_sale(creator: String, package_id: Uuid, sale_amount: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            creator,
            package_id,
            sale_amount,
            creator_amount: sale_amount * Self::CREATOR_SHARE,
            platform_fee: sale_amount * Self::PLATFORM_SHARE,
            status: PayoutStatus::Pending,
            sale_date: Utc::now(),
            processed_at: None,
            transaction_id: None,
            notes: None,
        }
    }

    /// Marks the payout as processing.
    pub fn start_processing(&mut self) {
        self.status = PayoutStatus::Processing;
    }

    /// Marks the payout as completed.
    pub fn complete(&mut self, transaction_id: String) {
        self.status = PayoutStatus::Completed;
        self.processed_at = Some(Utc::now());
        self.transaction_id = Some(transaction_id);
    }

    /// Marks the payout as failed.
    pub fn fail(&mut self, reason: &str) {
        self.status = PayoutStatus::Failed;
        self.processed_at = Some(Utc::now());
        self.notes = Some(reason.to_string());
    }

    /// Cancels the payout.
    pub fn cancel(&mut self, reason: Option<&str>) {
        self.status = PayoutStatus::Cancelled;
        self.processed_at = Some(Utc::now());
        if let Some(r) = reason {
            self.notes = Some(r.to_string());
        }
    }

    /// Returns true if the payout is finalized (completed, failed, or cancelled).
    pub fn is_finalized(&self) -> bool {
        matches!(
            self.status,
            PayoutStatus::Completed | PayoutStatus::Failed | PayoutStatus::Cancelled
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payout_creation() {
        let payout = PayoutRecord::from_sale("alice".to_string(), Uuid::new_v4(), 10.00);

        // Verify 70/30 split
        assert_eq!(payout.creator_amount, 7.00);
        assert_eq!(payout.platform_fee, 3.00);
        assert_eq!(payout.status, PayoutStatus::Pending);
        assert!(!payout.is_finalized());
    }

    #[test]
    fn test_payout_lifecycle() {
        let mut payout = PayoutRecord::from_sale("bob".to_string(), Uuid::new_v4(), 20.00);

        payout.start_processing();
        assert_eq!(payout.status, PayoutStatus::Processing);
        assert!(!payout.is_finalized());

        payout.complete("txn_123".to_string());
        assert_eq!(payout.status, PayoutStatus::Completed);
        assert_eq!(payout.transaction_id, Some("txn_123".to_string()));
        assert!(payout.is_finalized());
    }

    #[test]
    fn test_payout_failure() {
        let mut payout = PayoutRecord::from_sale("charlie".to_string(), Uuid::new_v4(), 5.00);

        payout.fail("Payment processor error");
        assert_eq!(payout.status, PayoutStatus::Failed);
        assert!(payout.is_finalized());
        assert!(payout.notes.is_some());
    }

    #[test]
    fn test_revenue_share_constants() {
        assert_eq!(PayoutRecord::CREATOR_SHARE + PayoutRecord::PLATFORM_SHARE, 1.0);
    }
}
