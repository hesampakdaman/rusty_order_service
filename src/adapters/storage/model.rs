use crate::domain;
use crate::domain::models::order::{Cancelled, Confirmed, Created, Order, Shipped};
use crate::domain::models::LineItem;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A flattened representation of an Order for persistence.
#[derive(Clone)]
pub struct PersistedOrder {
    pub(super) id: Uuid,
    pub(super) items: Vec<LineItem>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
    /// The order state as a string, e.g., "created", "confirmed", "shipped", "cancelled"
    pub(super) state: String,
    // Optional fields for state-specific data.
    pub(super) confirmed_at: Option<DateTime<Utc>>,
    pub(super) shipped_at: Option<DateTime<Utc>>,
    pub(super) tracking_id: Option<String>,
    pub(super) cancelled_at: Option<DateTime<Utc>>,
}

impl From<Order<Created>> for PersistedOrder {
    fn from(order: Order<Created>) -> Self {
        PersistedOrder {
            id: order.id,
            items: order.line_items,
            created_at: order.created_at,
            updated_at: order.updated_at,
            state: "created".to_string(),
            confirmed_at: None,
            shipped_at: None,
            tracking_id: None,
            cancelled_at: None,
        }
    }
}

impl From<Order<Confirmed>> for PersistedOrder {
    fn from(order: Order<Confirmed>) -> Self {
        PersistedOrder {
            id: order.id,
            items: order.line_items,
            created_at: order.created_at,
            updated_at: order.updated_at,
            state: "confirmed".to_string(),
            confirmed_at: Some(order.state.confirmed_at),
            shipped_at: None,
            tracking_id: None,
            cancelled_at: None,
        }
    }
}

impl From<Order<Shipped>> for PersistedOrder {
    fn from(order: Order<Shipped>) -> Self {
        PersistedOrder {
            id: order.id,
            items: order.line_items,
            created_at: order.created_at,
            updated_at: order.updated_at,
            state: "shipped".to_string(),
            confirmed_at: Some(order.state.confirmed_at),
            shipped_at: Some(order.state.shipped_at),
            tracking_id: Some(order.state.tracking_id),
            cancelled_at: None,
        }
    }
}

impl From<Order<Cancelled>> for PersistedOrder {
    fn from(order: Order<Cancelled>) -> Self {
        PersistedOrder {
            id: order.id,
            items: order.line_items,
            created_at: order.created_at,
            updated_at: order.updated_at,
            state: "cancelled".to_string(),
            confirmed_at: None,
            shipped_at: None,
            tracking_id: None,
            cancelled_at: Some(order.state.cancelled_at),
        }
    }
}

impl TryFrom<PersistedOrder> for Order<Created> {
    type Error = domain::Error;

    fn try_from(storage: PersistedOrder) -> Result<Self, Self::Error> {
        if storage.state != "created" {
            return Err(Self::Error::InvalidOrderType(storage.state));
        }
        Ok(Order {
            id: storage.id,
            line_items: storage.items,
            created_at: storage.created_at,
            updated_at: storage.updated_at,
            state: Created,
        })
    }
}

impl TryFrom<PersistedOrder> for Order<Confirmed> {
    type Error = domain::Error;

    fn try_from(storage: PersistedOrder) -> Result<Self, Self::Error> {
        if storage.state != "confirmed" {
            return Err(domain::Error::InvalidOrderType(storage.state));
        }
        let confirmed_at = storage
            .confirmed_at
            .ok_or_else(|| domain::Error::MissingField("confirmed_at".to_string()))?;
        Ok(Order {
            id: storage.id,
            line_items: storage.items,
            created_at: storage.created_at,
            updated_at: storage.updated_at,
            state: Confirmed { confirmed_at },
        })
    }
}

impl TryFrom<PersistedOrder> for Order<Shipped> {
    type Error = domain::Error;

    fn try_from(storage: PersistedOrder) -> Result<Self, Self::Error> {
        if storage.state != "shipped" {
            return Err(domain::Error::InvalidOrderType(storage.state));
        }
        let confirmed_at = storage
            .confirmed_at
            .ok_or_else(|| domain::Error::MissingField("confirmed_at".to_string()))?;
        let shipped_at = storage
            .shipped_at
            .ok_or_else(|| domain::Error::MissingField("shipped_at".to_string()))?;
        let tracking_id = storage
            .tracking_id
            .ok_or_else(|| domain::Error::MissingField("tracking_id".to_string()))?;
        Ok(Order {
            id: storage.id,
            line_items: storage.items,
            created_at: storage.created_at,
            updated_at: storage.updated_at,
            state: Shipped {
                confirmed_at,
                shipped_at,
                tracking_id,
            },
        })
    }
}

impl TryFrom<PersistedOrder> for Order<Cancelled> {
    type Error = domain::Error;

    fn try_from(storage: PersistedOrder) -> Result<Self, Self::Error> {
        if storage.state != "cancelled" {
            return Err(domain::Error::InvalidOrderType(storage.state));
        }
        let cancelled_at = storage
            .cancelled_at
            .ok_or_else(|| domain::Error::MissingField("cancelled_at".to_string()))?;
        Ok(Order {
            id: storage.id,
            line_items: storage.items,
            created_at: storage.created_at,
            updated_at: storage.updated_at,
            state: Cancelled { cancelled_at },
        })
    }
}
