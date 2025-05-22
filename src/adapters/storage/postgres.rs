use crate::domain::{self, models::order::OrderVariant};
use crate::ports;
use async_trait::async_trait;
use serde_json; // For potential error handling or specific serialization needs
use sqlx::{self, types::Json, PgPool};
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresRepository {
    pub pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ports::Repository for PostgresRepository {
    async fn save(&self, order: OrderVariant) -> Result<(), domain::Error> {
        let order_id = order.id();
        let order_json = Json(order); // sqlx::types::Json will handle serialization

        sqlx::query!(
            r#"
            INSERT INTO orders (id, data)
            VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET data = $2
            "#,
            order_id,
            order_json as _ // Cast to satisfy query! macro if types are opaque
        )
        .execute(&self.pool)
        .await
        .map_err(|e| domain::Error::RepositoryBackendFailure(e.into()))?;

        Ok(())
    }

    async fn get(&self, id: &Uuid) -> Result<OrderVariant, domain::Error> {
        #[derive(sqlx::FromRow)]
        struct OrderRow {
            data: Json<OrderVariant>,
        }

        let result = sqlx::query_as!(
            OrderRow,
            r#"
            SELECT data FROM orders WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => Ok(row.data.0), // Extract OrderVariant from Json
            Err(sqlx::Error::RowNotFound) => Err(domain::Error::OrderNotFound(*id)),
            Err(e) => Err(domain::Error::RepositoryBackendFailure(e.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{LineItem, Order};
    use crate::ports::Repository;
    use chrono::Utc;
    use sqlx::PgPool;
    use std::env;
    use uuid::Uuid;

    async fn setup_repository() -> PostgresRepository {
        dotenv::dotenv().ok(); // Load .env file if present, for local testing
        let database_url = env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set for integration tests");
        
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to TEST_DATABASE_URL");

        // Clean the table before each test
        sqlx::query("TRUNCATE TABLE orders CASCADE")
            .execute(&pool)
            .await
            .expect("Failed to truncate orders table");
        
        PostgresRepository::new(pool)
    }

    #[tokio::test]
    async fn save_and_get_order() {
        let repo = setup_repository().await;
        let order_id = Uuid::new_v4();
        let now = Utc::now();
        let item = LineItem::new(Uuid::new_v4(), 10);
        let order = Order::new(order_id, vec![item.clone()], now).unwrap();
        let order_variant: OrderVariant = order.into();

        let save_result = repo.save(order_variant.clone()).await;
        assert!(save_result.is_ok());

        let get_result = repo.get(&order_id).await;
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap(), order_variant);
    }

    #[tokio::test]
    async fn get_non_existent_order() {
        let repo = setup_repository().await;
        let random_id = Uuid::new_v4();

        let get_result = repo.get(&random_id).await;
        assert!(matches!(get_result, Err(domain::Error::OrderNotFound(id)) if id == random_id));
    }

    #[tokio::test]
    async fn save_duplicate_order_updates() {
        let repo = setup_repository().await;
        let order_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Create and save initial order
        let item1 = LineItem::new(Uuid::new_v4(), 5);
        let order1_created = Order::new(order_id, vec![item1.clone()], now).unwrap();
        let order1_variant: OrderVariant = order1_created.clone().into();
        
        let save_result1 = repo.save(order1_variant.clone()).await;
        assert!(save_result1.is_ok());

        // Confirm the order (updates its state and updated_at)
        let later = now + chrono::Duration::seconds(10);
        let order2_confirmed = order1_created.confirm(later);
        let order2_variant: OrderVariant = order2_confirmed.into();
        
        let save_result2 = repo.save(order2_variant.clone()).await;
        assert!(save_result2.is_ok());

        let get_result = repo.get(&order_id).await;
        assert!(get_result.is_ok());
        let retrieved_order = get_result.unwrap();
        
        // Check that the retrieved order is the updated (confirmed) one
        assert_eq!(retrieved_order.id(), order_id);
        match retrieved_order {
            OrderVariant::Confirmed(o) => {
                assert_eq!(o.id, order_id);
                assert_eq!(o.state.confirmed_at, later);
            }
            _ => panic!("Order was not in Confirmed state after update"),
        }
        assert_eq!(retrieved_order, order2_variant); // Overall equality check
    }
}
