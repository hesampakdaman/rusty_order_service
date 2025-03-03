use super::utils;
use crate::{domain::models, ports};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::sync::Arc;

pub async fn create(
    State(service): State<Arc<dyn ports::Service + Send + Sync>>,
    Json(payload): Json<CreateOrderRequest>,
) -> impl IntoResponse {
    let items = payload.items;
    match service.create(items).await {
        Ok(order_id) => (StatusCode::OK, Json(order_id)).into_response(),
        Err(err) => utils::map_domain_error(&err).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub items: Vec<models::LineItem>,
}
