// src/domain/errors.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Order must have at least one item")]
    EmptyOrder,
    #[error("Invalid state transition: {0}")]
    InvalidTransition(String),
    #[error("No order found: {0}")]
    OrderNotFound(String),
    #[error("Invalid order type")]
    InvalidOrderType(String),
    #[error("Missing field: {0}")]
    MissingField(String),
    #[error("No order found: {0}")]
    RepositoryBackendFailure(String),
}
