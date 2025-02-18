use crate::domain::{self, models::Order};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repository {
    /// The concrete storage representation.
    type StorageOrder;

    /// Persists an order to the underlying storage.
    async fn save<T>(&self, order: Order<T>) -> Result<(), domain::Error>
    where
        T: Send,
        Order<T>: Into<Self::StorageOrder> + Send;

    /// Retrieves an order by its ID.
    async fn find<T>(&self, id: &Uuid) -> Result<Order<T>, domain::Error>
    where
        T: Send,
        Order<T>: Send + TryFrom<Self::StorageOrder, Error = domain::Error>;
}
