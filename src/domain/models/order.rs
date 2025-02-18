use super::LineItem;
use crate::domain::Error;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The core Order aggregate is generic over its state, which defaults to `Created`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order<State = Created> {
    pub id: Uuid,
    pub line_items: Vec<LineItem>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub state: State,
}

/// Marker type for an order that is still open for modifications.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Created;

/// Marker type for an order that has been confirmed (items are frozen).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Confirmed {
    pub confirmed_at: DateTime<Utc>,
}

/// Marker type for an order that has been cancelled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cancelled {
    pub cancelled_at: DateTime<Utc>,
}

/// Marker type for an order that has been shipped.
/// It carries the confirmed timestamp, shipped timestamp, and a tracking ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shipped {
    pub confirmed_at: DateTime<Utc>,
    pub shipped_at: DateTime<Utc>,
    pub tracking_id: String,
}

impl Order<Created> {
    /// Creates a new order in the Created state.
    ///
    /// Returns an error if the `items` vector is empty.
    pub fn new(id: Uuid, items: Vec<LineItem>, now: DateTime<Utc>) -> Result<Self, Error> {
        if items.is_empty() {
            Err(Error::EmptyOrder)
        } else {
            Ok(Self {
                id,
                line_items: items,
                created_at: now,
                updated_at: now,
                state: Created,
            })
        }
    }

    /// Adds an item to the order.
    /// Updates the `updated_at` timestamp.
    pub fn add_line_item(&mut self, item: LineItem, now: DateTime<Utc>) {
        self.line_items.push(item);
        self.updated_at = now;
    }

    /// Removes an item from the order by its ID.
    /// Updates the `updated_at` timestamp.
    pub fn remove_item(&mut self, item_id: &Uuid, now: DateTime<Utc>) {
        self.line_items.retain(|i| &i.id != item_id);
        self.updated_at = now;
    }

    /// Confirms the order, transitioning it to the Confirmed state.
    /// At this point, modifications to items should be frozen.
    pub fn confirm(self, confirmed_at: DateTime<Utc>) -> Order<Confirmed> {
        Order {
            id: self.id,
            line_items: self.line_items,
            created_at: self.created_at,
            updated_at: self.updated_at,
            state: Confirmed { confirmed_at },
        }
    }

    /// Cancels the created order, transitioning it to the Cancelled state.
    pub fn cancel(self, cancelled_at: DateTime<Utc>) -> Order<Cancelled> {
        Order {
            id: self.id,
            line_items: self.line_items,
            created_at: self.created_at,
            updated_at: self.updated_at,
            state: Cancelled { cancelled_at },
        }
    }
}

impl Order<Confirmed> {
    /// Ships the confirmed order, transitioning it to the Shipped state.
    pub fn ship(self, shipped_at: DateTime<Utc>, tracking_id: String) -> Order<Shipped> {
        Order {
            id: self.id,
            line_items: self.line_items,
            created_at: self.created_at,
            updated_at: self.updated_at,
            state: Shipped {
                confirmed_at: self.state.confirmed_at,
                shipped_at,
                tracking_id,
            },
        }
    }

    pub fn confirmed_at(&self) -> DateTime<Utc> {
        self.state.confirmed_at
    }
}

impl Order<Cancelled> {
    pub fn cancelled_at(&self) -> DateTime<Utc> {
        self.state.cancelled_at
    }
}

impl Order<Shipped> {
    /// Returns the tracking ID for the shipped order.
    pub fn tracking_id(&self) -> String {
        self.state.tracking_id.to_string()
    }

    pub fn confirmed_at(&self) -> DateTime<Utc> {
        self.state.confirmed_at
    }

    pub fn shipped_at(&self) -> DateTime<Utc> {
        self.state.shipped_at
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderVariant {
    Created(Order<Created>),
    Confirmed(Order<Confirmed>),
    Shipped(Order<Shipped>),
    Cancelled(Order<Cancelled>),
}

impl OrderVariant {
    pub fn id(&self) -> Uuid {
        match self {
            OrderVariant::Created(order) => order.id,
            OrderVariant::Confirmed(order) => order.id,
            OrderVariant::Shipped(order) => order.id,
            OrderVariant::Cancelled(order) => order.id,
        }
    }
}

impl From<Order<Created>> for OrderVariant {
    fn from(order: Order<Created>) -> Self {
        OrderVariant::Created(order)
    }
}

impl From<Order<Confirmed>> for OrderVariant {
    fn from(order: Order<Confirmed>) -> Self {
        OrderVariant::Confirmed(order)
    }
}

impl From<Order<Shipped>> for OrderVariant {
    fn from(order: Order<Shipped>) -> Self {
        OrderVariant::Shipped(order)
    }
}

impl From<Order<Cancelled>> for OrderVariant {
    fn from(order: Order<Cancelled>) -> Self {
        OrderVariant::Cancelled(order)
    }
}
