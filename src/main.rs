use chrono::Utc;
use rusty_order_service::{
    adapters::{MemoryRepository, OrderService},
    domain::models::{order::OrderVariant, LineItem},
    ports::Service,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let service = OrderService::new(MemoryRepository::new());

    let id = service
        .create(vec![LineItem::new(Uuid::new_v4(), 1)])
        .await
        .expect("Failed to save order");
    println!("Order with id {id} was created");
    match service.get(&id).await.unwrap() {
        OrderVariant::Created(_) => println!("Order {id} was found"),
        _ => panic!("Order {id} is not in the database"),
    };

    service
        .add_line_item(&id, LineItem::new(Uuid::new_v4(), 2))
        .await
        .expect("Failed to add line item to order {id}");
    println!("Added line item to Order {id}");

    if let Err(e) = service.ship(&id, Utc::now(), "tracking-id").await {
        println!("Could not ship Order {id}: {e}");
    }
    service
        .confirm(&id)
        .await
        .expect("Failed to confirm Order {id}");
    println!("Confirmed Order {id}");

    service
        .ship(&id, Utc::now(), "tracking-id")
        .await
        .expect("Failed to ship order");
}
