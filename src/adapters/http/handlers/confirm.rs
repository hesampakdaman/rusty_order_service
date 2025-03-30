use super::utils;
use crate::ports;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn confirm(
    State(service): State<Arc<dyn ports::Service + Send + Sync>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    service
        .confirm(&id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|err| utils::map_domain_error(&err))
}
