use crate::domain::{self, models::order::OrderVariant};
use crate::ports::{self};
use async_trait::async_trait;
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

#[derive(Default)]
pub struct MemoryRepository {
    orders: Mutex<HashMap<Uuid, OrderVariant>>,
}

impl MemoryRepository {
    pub fn new() -> Self {
        Self {
            orders: Mutex::new(HashMap::default()),
        }
    }
}

#[async_trait]
impl ports::Repository for MemoryRepository {
    async fn save(&self, order: OrderVariant) -> Result<(), domain::Error> {
        let mut orders = self.orders.lock().unwrap();
        orders.insert(order.id(), order);
        Ok(())
    }

    async fn get(&self, id: &Uuid) -> Result<OrderVariant, domain::Error> {
        let orders = self
            .orders
            .lock()
            .map_err(|e| domain::Error::RepositoryBackendFailure(e.to_string()))?;

        orders
            .get(id)
            .ok_or_else(|| domain::Error::OrderNotFound(id.to_string()))
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::Repository;
    use chrono::Utc;
    use domain::models::{LineItem, Order};
    use uuid::Uuid;

    #[tokio::test]
    async fn save() {
        let repo = MemoryRepository::new();

        // Given: A new order
        let order = Order::new(
            Uuid::new_v4(),
            vec![LineItem::new(Uuid::new_v4(), 2)],
            Utc::now(),
        )
        .expect("Failed to create order");

        // When: Saving the order
        let actual = repo.save(order.into()).await;

        // Then: We expect no errors
        assert!(actual.is_ok());
    }

    #[tokio::test]
    async fn get() {
        let repo = MemoryRepository::new();

        // Given: A saved order
        let id = Uuid::new_v4();
        let expected: OrderVariant =
            Order::new(id, vec![LineItem::new(Uuid::new_v4(), 2)], Utc::now())
                .expect("Failed to create order")
                .into();

        repo.save(expected.clone())
            .await
            .expect("Failed to save order");

        // When: find is called with order's id
        let actual = repo.get(&id).await.expect("Failed to find order");

        // Then: The order should be found
        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn order_not_found() {
        let repo = MemoryRepository::new();

        // Given: An order ID that does not exist
        let id = Uuid::new_v4();

        // When: The order is to be fetched
        let actual = repo.get(&id).await;

        // Then: We expect an error
        assert!(matches!(actual, Err(domain::Error::OrderNotFound(_))));
    }
}
