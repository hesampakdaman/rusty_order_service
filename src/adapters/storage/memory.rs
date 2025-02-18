use crate::domain;
use std::{collections::HashMap, sync::Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    adapters::storage::model::PersistedOrder,
    domain::models::Order,
    ports::{self},
};

#[derive(Default)]
pub struct MemoryRepository {
    orders: Mutex<HashMap<Uuid, PersistedOrder>>,
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
    type StorageOrder = PersistedOrder;

    async fn save<T>(&self, order: Order<T>) -> Result<(), domain::Error>
    where
        T: Send,
        Order<T>: Into<PersistedOrder>,
    {
        let mut orders = self.orders.lock().unwrap();
        orders.insert(order.id, order.into());
        Ok(())
    }

    async fn find<T>(&self, id: &Uuid) -> Result<Order<T>, domain::Error>
    where
        T: Send,
        Order<T>: TryFrom<Self::StorageOrder, Error = domain::Error>,
    {
        let orders = self
            .orders
            .lock()
            .map_err(|e| domain::Error::RepositoryBackendFailure(e.to_string()))?;

        let storage_order = orders
            .get(id)
            .ok_or_else(|| domain::Error::OrderNotFound(id.to_string()))?;

        storage_order.clone().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::order::Created;
    use crate::domain::models::{self, Order};
    use crate::ports::Repository;
    use chrono::Utc;
    use domain::models::order::Confirmed;
    use uuid::Uuid;

    #[tokio::test]
    async fn save_order() {
        let repo = MemoryRepository::new();

        // Given: A new order
        let order = Order::new(
            Uuid::new_v4(),
            vec![models::LineItem::new(Uuid::new_v4(), 2)],
            Utc::now(),
        )
        .expect("Failed to create order");

        // When: Saving the order
        let actual = repo.save(order.clone()).await;

        // Then: We expect no errors
        assert!(actual.is_ok());
    }

    #[tokio::test]
    async fn find_order() {
        let repo = MemoryRepository::new();

        // Given: A saved order
        let id = Uuid::new_v4();
        let expected = Order::new(
            id,
            vec![models::LineItem::new(Uuid::new_v4(), 2)],
            Utc::now(),
        )
        .expect("Failed to create order");
        repo.save(expected.clone())
            .await
            .expect("Failed to save order");

        // When: find is called with order's id
        let actual: Order<Created> = repo.find(&id).await.expect("Failed to find order");

        // Then: The order should be found
        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn find_order_not_found() {
        let repo = MemoryRepository::new();

        // Given: An order ID that does not exist
        let id = Uuid::new_v4();

        // When: The order is to be fetched
        let result: Result<Order<Created>, _> = repo.find(&id).await;

        // Then: We expect an error
        assert!(matches!(result, Err(domain::Error::OrderNotFound(_))));
    }

    #[tokio::test]
    async fn find_order_wrong_state() {
        let repo = MemoryRepository::new();

        // Given: A saved order in the Created state.
        let id = Uuid::new_v4();
        let order = Order::new(
            id,
            vec![models::LineItem::new(Uuid::new_v4(), 2)],
            Utc::now(),
        )
        .expect("Failed to create order");
        repo.save(order).await.expect("Failed to save order");

        // When: We attempt to find the order expecting it to be Confirmed.
        let result: Result<Order<Confirmed>, _> = repo.find(&id).await;

        // Then: The conversion should fail with an InvalidOrderType error.
        assert!(matches!(result, Err(domain::Error::InvalidOrderType(_))));
    }
}
