use axum::{routing::post, Router};
use rusty_order_service::{
    adapters::{self, http, PostgresRepository}, // Added PostgresRepository
    ports::Repository,                          // Added Repository trait
};
use sqlx::PgPool; // Added PgPool
use std::{env, sync::Arc}; // Added env

#[tokio::main]
async fn main() {
    let repo: Arc<dyn Repository + Send + Sync> = match env::var("DATABASE_URL") {
        Ok(database_url) => {
            println!("DATABASE_URL found, attempting to connect to Postgres: {}", database_url);
            let pool = PgPool::connect(&database_url)
                .await
                .expect("Failed to connect to Postgres");
            println!("Successfully connected to Postgres.");
            Arc::new(PostgresRepository::new(pool))
        }
        Err(_) => {
            println!("DATABASE_URL not found, falling back to MemoryRepository.");
            Arc::new(adapters::MemoryRepository::new())
        }
    };

    let service = Arc::new(adapters::OrderService::new(repo));

    let app = Router::new()
        .route("/orders", post(http::handlers::create))
        .route("/orders/{id}/confirm", post(http::handlers::confirm))
        .with_state(service);

    let addr = "0.0.0.0:3000";
    let lis = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("service started on {addr}");
    axum::serve(lis, app).await.unwrap();
}
