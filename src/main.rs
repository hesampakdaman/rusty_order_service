use chrono::prelude::*;
use rusty_order_service::{
    adapters::MemoryRepository,
    domain::models::{self, order::Created, Order},
    ports::Repository,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Create an instance of the repository.
    let repo = MemoryRepository::new();
    let id = Uuid::new_v4();

    // Create an order in the "Created" state.
    let order = Order::new(
        id,
        vec![models::LineItem {
            id: Uuid::new_v4(),
            quantity: 1,
        }],
        Utc::now(),
    )
    .expect("Failed to create order");

    // Confirm the order.
    // let order = order.confirm(Utc::now());

    // Save the order using the repository.
    repo.save(order).await.expect("Failed to save order");
    let order: Order<Created> = repo.find(&id).await.unwrap();

    println!("Order saved successfully!");
}
