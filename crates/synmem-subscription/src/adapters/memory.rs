//! In-memory storage adapter for testing and development.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::{Customer, License, Subscription, SubscriptionEvent, SubscriptionStatus};
use crate::error::{Result, SubscriptionError};
use crate::ports::storage::SubscriptionStoragePort;

/// In-memory storage for subscriptions.
///
/// Useful for testing and development. Not suitable for production.
#[derive(Debug, Default)]
pub struct InMemoryStorage {
    customers: RwLock<HashMap<Uuid, Customer>>,
    customers_by_email: RwLock<HashMap<String, Uuid>>,
    customers_by_stripe_id: RwLock<HashMap<String, Uuid>>,
    subscriptions: RwLock<HashMap<Uuid, Subscription>>,
    licenses: RwLock<HashMap<String, License>>,
    events: RwLock<Vec<SubscriptionEvent>>,
}

impl InMemoryStorage {
    /// Creates a new in-memory storage.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl SubscriptionStoragePort for InMemoryStorage {
    async fn create_customer(&self, customer: &Customer) -> Result<()> {
        let mut customers = self.customers.write().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        let mut by_email = self.customers_by_email.write().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;

        customers.insert(customer.id, customer.clone());
        by_email.insert(customer.email.clone(), customer.id);

        if let Some(stripe_id) = &customer.stripe_customer_id {
            let mut by_stripe = self.customers_by_stripe_id.write().map_err(|e| {
                SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
            })?;
            by_stripe.insert(stripe_id.clone(), customer.id);
        }

        Ok(())
    }

    async fn get_customer(&self, id: Uuid) -> Result<Option<Customer>> {
        let customers = self.customers.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        Ok(customers.get(&id).cloned())
    }

    async fn get_customer_by_email(&self, email: &str) -> Result<Option<Customer>> {
        let by_email = self.customers_by_email.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        let customers = self.customers.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;

        Ok(by_email.get(email).and_then(|id| customers.get(id).cloned()))
    }

    async fn get_customer_by_stripe_id(&self, stripe_id: &str) -> Result<Option<Customer>> {
        let by_stripe = self.customers_by_stripe_id.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        let customers = self.customers.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;

        Ok(by_stripe
            .get(stripe_id)
            .and_then(|id| customers.get(id).cloned()))
    }

    async fn update_customer(&self, customer: &Customer) -> Result<()> {
        let mut customers = self.customers.write().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;

        if !customers.contains_key(&customer.id) {
            return Err(SubscriptionError::CustomerNotFound(
                customer.id.to_string(),
            ));
        }

        // Update stripe ID index if changed
        if let Some(stripe_id) = &customer.stripe_customer_id {
            let mut by_stripe = self.customers_by_stripe_id.write().map_err(|e| {
                SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
            })?;
            by_stripe.insert(stripe_id.clone(), customer.id);
        }

        customers.insert(customer.id, customer.clone());
        Ok(())
    }

    async fn create_subscription(&self, subscription: &Subscription) -> Result<()> {
        let mut subscriptions = self.subscriptions.write().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        subscriptions.insert(subscription.id, subscription.clone());
        Ok(())
    }

    async fn get_subscription(&self, id: Uuid) -> Result<Option<Subscription>> {
        let subscriptions = self.subscriptions.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        Ok(subscriptions.get(&id).cloned())
    }

    async fn get_customer_subscriptions(&self, customer_id: Uuid) -> Result<Vec<Subscription>> {
        let subscriptions = self.subscriptions.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        Ok(subscriptions
            .values()
            .filter(|s| s.customer_id == customer_id)
            .cloned()
            .collect())
    }

    async fn get_active_subscription(&self, customer_id: Uuid) -> Result<Option<Subscription>> {
        let subscriptions = self.subscriptions.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        Ok(subscriptions
            .values()
            .find(|s| {
                s.customer_id == customer_id
                    && matches!(
                        s.status,
                        SubscriptionStatus::Active | SubscriptionStatus::Trialing
                    )
            })
            .cloned())
    }

    async fn update_subscription(&self, subscription: &Subscription) -> Result<()> {
        let mut subscriptions = self.subscriptions.write().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;

        if !subscriptions.contains_key(&subscription.id) {
            return Err(SubscriptionError::SubscriptionNotFound(
                subscription.id.to_string(),
            ));
        }

        subscriptions.insert(subscription.id, subscription.clone());
        Ok(())
    }

    async fn store_license(&self, license: &License) -> Result<()> {
        let mut licenses = self.licenses.write().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        licenses.insert(license.key.clone(), license.clone());
        Ok(())
    }

    async fn get_license(&self, key: &str) -> Result<Option<License>> {
        let licenses = self.licenses.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        Ok(licenses.get(key).cloned())
    }

    async fn get_customer_licenses(&self, _customer_id: Uuid) -> Result<Vec<License>> {
        // In a real implementation, licenses would be associated with customers
        // For now, return all licenses
        let licenses = self.licenses.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        Ok(licenses.values().cloned().collect())
    }

    async fn revoke_license(&self, key: &str) -> Result<()> {
        let mut licenses = self.licenses.write().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        licenses.remove(key);
        Ok(())
    }

    async fn record_event(&self, event: &SubscriptionEvent) -> Result<()> {
        let mut events = self.events.write().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;
        events.push(event.clone());
        Ok(())
    }

    async fn get_customer_events(
        &self,
        customer_id: Uuid,
        limit: Option<u32>,
    ) -> Result<Vec<SubscriptionEvent>> {
        let events = self.events.read().map_err(|e| {
            SubscriptionError::Storage(format!("Failed to acquire lock: {e}"))
        })?;

        let mut customer_events: Vec<_> = events
            .iter()
            .filter(|e| e.customer_id == customer_id)
            .cloned()
            .collect();

        // Sort by timestamp descending
        customer_events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        if let Some(limit) = limit {
            customer_events.truncate(limit as usize);
        }

        Ok(customer_events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{SubscriptionEventType, Tier};
    use chrono::{Duration, Utc};

    #[tokio::test]
    async fn test_customer_crud() {
        let storage = InMemoryStorage::new();

        let customer = Customer::new("test@example.com".to_string());
        storage.create_customer(&customer).await.unwrap();

        // Get by ID
        let found = storage.get_customer(customer.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().email, "test@example.com");

        // Get by email
        let found = storage
            .get_customer_by_email("test@example.com")
            .await
            .unwrap();
        assert!(found.is_some());

        // Not found
        let found = storage
            .get_customer_by_email("other@example.com")
            .await
            .unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_subscription_crud() {
        let storage = InMemoryStorage::new();

        let customer = Customer::new("test@example.com".to_string());
        storage.create_customer(&customer).await.unwrap();

        let now = Utc::now();
        let subscription = Subscription::new(customer.id, Tier::Pro, now, now + Duration::days(30));
        storage.create_subscription(&subscription).await.unwrap();

        // Get by ID
        let found = storage.get_subscription(subscription.id).await.unwrap();
        assert!(found.is_some());

        // Get active subscription
        let active = storage
            .get_active_subscription(customer.id)
            .await
            .unwrap();
        assert!(active.is_some());
        assert_eq!(active.unwrap().tier, Tier::Pro);
    }

    #[tokio::test]
    async fn test_events() {
        let storage = InMemoryStorage::new();

        let customer_id = Uuid::new_v4();

        let event1 = SubscriptionEvent::new(
            customer_id,
            SubscriptionEventType::Created,
            None,
            Some(Tier::Free),
        );
        let event2 = SubscriptionEvent::new(
            customer_id,
            SubscriptionEventType::Upgraded,
            Some(Tier::Free),
            Some(Tier::Pro),
        );

        storage.record_event(&event1).await.unwrap();
        storage.record_event(&event2).await.unwrap();

        let events = storage
            .get_customer_events(customer_id, None)
            .await
            .unwrap();
        assert_eq!(events.len(), 2);

        // Test limit
        let events = storage
            .get_customer_events(customer_id, Some(1))
            .await
            .unwrap();
        assert_eq!(events.len(), 1);
    }
}
