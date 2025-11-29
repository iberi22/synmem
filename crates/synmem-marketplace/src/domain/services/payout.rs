//! Payout service - manages revenue distribution and payouts.

use crate::domain::entities::{PayoutRecord, PayoutStatus};
use crate::ports::outbound::{PayoutGateway, ScraperRepository};
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur in payout operations.
#[derive(Debug, Error)]
pub enum PayoutError {
    #[error("Payout not found: {0}")]
    PayoutNotFound(Uuid),
    #[error("Package not found: {0}")]
    PackageNotFound(Uuid),
    #[error("Payout already processed: {0}")]
    AlreadyProcessed(Uuid),
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    #[error("Payment gateway error: {0}")]
    GatewayError(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

/// Result type for payout operations.
pub type PayoutResult<T> = Result<T, PayoutError>;

/// Payout service trait.
#[async_trait]
pub trait PayoutService: Send + Sync {
    /// Records a sale and creates a pending payout.
    async fn record_sale(
        &self,
        package_id: Uuid,
        buyer: &str,
        amount: f64,
    ) -> PayoutResult<PayoutRecord>;

    /// Gets a payout by ID.
    async fn get_payout(&self, id: Uuid) -> PayoutResult<PayoutRecord>;

    /// Lists pending payouts for a creator.
    async fn list_pending_payouts(&self, creator: &str) -> PayoutResult<Vec<PayoutRecord>>;

    /// Lists all payouts for a creator.
    async fn list_creator_payouts(&self, creator: &str) -> PayoutResult<Vec<PayoutRecord>>;

    /// Processes a pending payout.
    async fn process_payout(&self, id: Uuid) -> PayoutResult<PayoutRecord>;

    /// Processes all pending payouts for a creator.
    async fn process_creator_payouts(&self, creator: &str) -> PayoutResult<Vec<PayoutRecord>>;

    /// Gets total earnings for a creator.
    async fn get_total_earnings(&self, creator: &str) -> PayoutResult<f64>;

    /// Gets pending earnings for a creator.
    async fn get_pending_earnings(&self, creator: &str) -> PayoutResult<f64>;

    /// Cancels a pending payout.
    async fn cancel_payout(&self, id: Uuid, reason: &str) -> PayoutResult<PayoutRecord>;
}

/// Default implementation of the payout service.
pub struct DefaultPayoutService<R: ScraperRepository, G: PayoutGateway> {
    repository: R,
    gateway: G,
    min_payout_amount: f64,
}

impl<R: ScraperRepository, G: PayoutGateway> DefaultPayoutService<R, G> {
    /// Creates a new payout service.
    pub fn new(repository: R, gateway: G) -> Self {
        Self {
            repository,
            gateway,
            min_payout_amount: 1.00, // $1 minimum payout
        }
    }

    /// Creates a payout service with a custom minimum payout amount.
    pub fn with_min_payout(repository: R, gateway: G, min_amount: f64) -> Self {
        Self {
            repository,
            gateway,
            min_payout_amount: min_amount,
        }
    }
}

#[async_trait]
impl<R: ScraperRepository + Send + Sync, G: PayoutGateway + Send + Sync> PayoutService
    for DefaultPayoutService<R, G>
{
    async fn record_sale(
        &self,
        package_id: Uuid,
        _buyer: &str,
        amount: f64,
    ) -> PayoutResult<PayoutRecord> {
        if amount <= 0.0 {
            return Err(PayoutError::InvalidAmount(
                "Amount must be positive".to_string(),
            ));
        }

        // Get the package to find the creator
        let package = self
            .repository
            .get_by_id(package_id)
            .await
            .map_err(|e| PayoutError::RepositoryError(e.to_string()))?
            .ok_or(PayoutError::PackageNotFound(package_id))?;

        let payout = PayoutRecord::from_sale(package.author.clone(), package_id, amount);

        self.repository
            .save_payout(&payout)
            .await
            .map_err(|e| PayoutError::RepositoryError(e.to_string()))?;

        Ok(payout)
    }

    async fn get_payout(&self, id: Uuid) -> PayoutResult<PayoutRecord> {
        self.repository
            .get_payout(id)
            .await
            .map_err(|e| PayoutError::RepositoryError(e.to_string()))?
            .ok_or(PayoutError::PayoutNotFound(id))
    }

    async fn list_pending_payouts(&self, creator: &str) -> PayoutResult<Vec<PayoutRecord>> {
        self.repository
            .list_creator_payouts(creator)
            .await
            .map_err(|e| PayoutError::RepositoryError(e.to_string()))
            .map(|payouts| {
                payouts
                    .into_iter()
                    .filter(|p| p.status == PayoutStatus::Pending)
                    .collect()
            })
    }

    async fn list_creator_payouts(&self, creator: &str) -> PayoutResult<Vec<PayoutRecord>> {
        self.repository
            .list_creator_payouts(creator)
            .await
            .map_err(|e| PayoutError::RepositoryError(e.to_string()))
    }

    async fn process_payout(&self, id: Uuid) -> PayoutResult<PayoutRecord> {
        let mut payout = self.get_payout(id).await?;

        if payout.is_finalized() {
            return Err(PayoutError::AlreadyProcessed(id));
        }

        if payout.creator_amount < self.min_payout_amount {
            return Err(PayoutError::InvalidAmount(format!(
                "Minimum payout amount is ${:.2}",
                self.min_payout_amount
            )));
        }

        payout.start_processing();

        // Process through payment gateway
        match self
            .gateway
            .process_payout(&payout.creator, payout.creator_amount)
            .await
        {
            Ok(transaction_id) => {
                payout.complete(transaction_id);
            }
            Err(e) => {
                payout.fail(&e.to_string());
            }
        }

        self.repository
            .save_payout(&payout)
            .await
            .map_err(|e| PayoutError::RepositoryError(e.to_string()))?;

        Ok(payout)
    }

    async fn process_creator_payouts(&self, creator: &str) -> PayoutResult<Vec<PayoutRecord>> {
        let pending = self.list_pending_payouts(creator).await?;
        let mut results = Vec::new();

        for payout in pending {
            // Process each payout, collecting successful ones
            // Failed payouts will remain in pending state for retry
            if let Ok(processed) = self.process_payout(payout.id).await {
                results.push(processed);
            }
        }

        Ok(results)
    }

    async fn get_total_earnings(&self, creator: &str) -> PayoutResult<f64> {
        let payouts = self.list_creator_payouts(creator).await?;
        let total = payouts
            .iter()
            .filter(|p| p.status == PayoutStatus::Completed)
            .map(|p| p.creator_amount)
            .sum();
        Ok(total)
    }

    async fn get_pending_earnings(&self, creator: &str) -> PayoutResult<f64> {
        let payouts = self.list_pending_payouts(creator).await?;
        let total = payouts.iter().map(|p| p.creator_amount).sum();
        Ok(total)
    }

    async fn cancel_payout(&self, id: Uuid, reason: &str) -> PayoutResult<PayoutRecord> {
        let mut payout = self.get_payout(id).await?;

        if payout.is_finalized() {
            return Err(PayoutError::AlreadyProcessed(id));
        }

        payout.cancel(Some(reason));

        self.repository
            .save_payout(&payout)
            .await
            .map_err(|e| PayoutError::RepositoryError(e.to_string()))?;

        Ok(payout)
    }
}
