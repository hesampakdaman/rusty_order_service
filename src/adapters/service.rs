use crate::domain::Error;
use crate::domain::models::order::OrderVariant;
use crate::domain::models::{LineItem, Order};
use crate::ports::{Repository, Service};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct OrderService<R: Repository> {
    repo: R,
}

impl<R: Repository> OrderService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: Repository> Service for OrderService<R> {
    async fn create(&self, items: Vec<LineItem>) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let order = Order::new(id, items, now)?;
        self.repo.save(order.into()).await?;
        Ok(id)
    }

    async fn add_line_item(&self, order_id: &Uuid, item: LineItem) -> Result<(), Error> {
        match self.repo.get(order_id).await? {
            OrderVariant::Created(mut order) => {
                order.add_line_item(item, Utc::now());
                self.repo.save(order.into()).await
            }
            _ => Err(Error::InvalidOrderType(format!(
                "Order {order_id} is not in a modifiable state"
            ))),
        }
    }

    async fn remove_line_item(&self, order_id: &Uuid, item_id: &Uuid) -> Result<(), Error> {
        match self.repo.get(order_id).await? {
            OrderVariant::Created(mut order) => {
                order.remove_item(item_id, Utc::now());

                if order.line_items.is_empty() {
                    return self.repo.save(order.cancel(Utc::now()).into()).await;
                }

                self.repo.save(order.into()).await
            }
            _ => Err(Error::InvalidOrderType(format!(
                "Order {order_id} is not in a modifiable state"
            ))),
        }
    }

    async fn confirm(&self, id: &Uuid) -> Result<(), Error> {
        match self.repo.get(id).await? {
            OrderVariant::Created(order) => self.repo.save(order.confirm(Utc::now()).into()).await,
            _ => Err(Error::InvalidOrderType(format!(
                "Order {id} cannot be confirmed"
            ))),
        }
    }

    async fn cancel(&self, id: &Uuid) -> Result<(), Error> {
        let variant = self.repo.get(id).await?;
        if let OrderVariant::Created(order) = variant {
            self.repo.save(order.cancel(Utc::now()).into()).await
        } else {
            Err(Error::InvalidOrderType(format!(
                "Order {id} cannot be cancelled"
            )))
        }
    }

    async fn ship(
        &self,
        id: &Uuid,
        shipped_at: DateTime<Utc>,
        tracking_id: &str,
    ) -> Result<(), Error> {
        let variant = self.repo.get(id).await?;
        if let OrderVariant::Confirmed(order) = variant {
            let shipped_order = order.ship(shipped_at, tracking_id.to_string());
            self.repo.save(shipped_order.into()).await
        } else {
            Err(Error::InvalidOrderType(format!(
                "Order {id} cannot be cancelled"
            )))
        }
    }

    async fn get(&self, id: &Uuid) -> Result<OrderVariant, Error> {
        self.repo.get(id).await
    }
}
