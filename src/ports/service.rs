use crate::domain::Error;
use crate::domain::models::LineItem;
use crate::domain::models::order::OrderVariant;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait Service: Sync {
    /// Creates a new order in the Created state and returns its ID.
    async fn create(&self, items: Vec<LineItem>) -> Result<Uuid, Error>;

    /// Adds a line item to an existing order.
    async fn add_line_item(&self, order_id: &Uuid, item: LineItem) -> Result<(), Error>;

    /// Removes a line item from an existing order.
    /// Adds a line item to an existing order.
    async fn remove_line_item(&self, order_id: &Uuid, item_id: &Uuid) -> Result<(), Error>;

    // Confirms a created order.
    async fn confirm(&self, id: &Uuid) -> Result<(), Error>;

    /// Cancels an existing order.
    async fn cancel(&self, id: &Uuid) -> Result<(), Error>;

    /// Ships an existing order, providing a shipped timestamp and tracking ID.
    async fn ship(
        &self,
        id: &Uuid,
        shipped_at: DateTime<Utc>,
        tracking_id: &str,
    ) -> Result<(), Error>;

    /// Retrieves an order by its ID.
    async fn get(&self, id: &Uuid) -> Result<OrderVariant, Error>;
}
