use axum::{Router, routing::post};
use rusty_order_service::adapters;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let service = Arc::new(adapters::OrderService::new(
        adapters::MemoryRepository::new(),
    ));

    let app = Router::new()
        .route("/orders", post(adapters::http::handlers::create))
        .with_state(service);

    let addr = "0.0.0.0:3000";
    let lis = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("service started on {addr}");
    axum::serve(lis, app).await.unwrap();
}
