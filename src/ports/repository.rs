use crate::domain::{self, models::order::OrderVariant};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repository: Sync + Send {
    /// Persists an order to the underlying storage.
    async fn save(&self, order: OrderVariant) -> Result<(), domain::Error>;

    /// Retrieves an order by its ID.
    async fn get(&self, id: &Uuid) -> Result<OrderVariant, domain::Error>;
}
