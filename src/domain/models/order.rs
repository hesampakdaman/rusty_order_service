use crate::domain::models::item::Item;
use crate::domain::Error;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The core Order aggregate is generic over its state, which defaults to `Created`.
#[derive(Debug)]
pub struct Order<State = Created> {
    pub id: Uuid,
    pub items: Vec<Item>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub state: State,
}

/// Marker type for an order that is still open for modifications.
#[derive(Debug)]
pub struct Created;

/// Marker type for an order that has been confirmed (items are frozen).
#[derive(Debug)]
pub struct Confirmed {
    pub confirmed_at: DateTime<Utc>,
}

/// Marker type for an order that has been cancelled.
#[derive(Debug)]
pub struct Cancelled {
    pub cancelled_at: DateTime<Utc>,
}

/// Marker type for an order that has been shipped.
/// It carries the confirmed timestamp, shipped timestamp, and a tracking ID.
#[derive(Debug)]
pub struct Shipped {
    pub confirmed_at: DateTime<Utc>,
    pub shipped_at: DateTime<Utc>,
    pub tracking_id: String,
}

impl Order<Created> {
    /// Creates a new order in the Created state.
    ///
    /// Returns an error if the `items` vector is empty.
    pub fn new(id: Uuid, items: Vec<Item>, now: DateTime<Utc>) -> Result<Self, Error> {
        if items.is_empty() {
            Err(Error::EmptyOrder)
        } else {
            Ok(Self {
                id,
                items,
                created_at: now,
                updated_at: now,
                state: Created,
            })
        }
    }

    /// Adds an item to the order.
    /// Updates the `updated_at` timestamp.
    pub fn add_item(mut self, item: Item, now: DateTime<Utc>) -> Self {
        self.items.push(item);
        self.updated_at = now;
        self
    }

    /// Removes an item from the order by its ID.
    /// Updates the `updated_at` timestamp.
    pub fn remove_item(mut self, item_id: Uuid, now: DateTime<Utc>) -> Self {
        self.items.retain(|i| i.id != item_id);
        self.updated_at = now;
        self
    }

    /// Confirms the order, transitioning it to the Confirmed state.
    /// At this point, modifications to items should be frozen.
    pub fn confirm(self, confirmed_at: DateTime<Utc>) -> Order<Confirmed> {
        Order {
            id: self.id,
            items: self.items,
            created_at: self.created_at,
            updated_at: self.updated_at,
            state: Confirmed { confirmed_at },
        }
    }

    /// Cancels the created order, transitioning it to the Cancelled state.
    pub fn cancel(self, cancelled_at: DateTime<Utc>) -> Order<Cancelled> {
        Order {
            id: self.id,
            items: self.items,
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
            items: self.items,
            created_at: self.created_at,
            updated_at: self.updated_at,
            state: Shipped {
                confirmed_at: self.state.confirmed_at,
                shipped_at,
                tracking_id,
            },
        }
    }

    /// Cancels the confirmed order, transitioning it to the Cancelled state.
    pub fn cancel(self, cancelled_at: DateTime<Utc>) -> Order<Cancelled> {
        Order {
            id: self.id,
            items: self.items,
            created_at: self.created_at,
            updated_at: self.updated_at,
            state: Cancelled { cancelled_at },
        }
    }
}

impl Order<Shipped> {
    /// Returns the tracking ID for the shipped order.
    pub fn tracking_id(&self) -> &str {
        &self.state.tracking_id
    }

    pub fn confirmed_at(&self) -> &DateTime<Utc> {
        &self.state.confirmed_at
    }
}
