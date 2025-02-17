// src/domain/errors.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Order must have at least one item")]
    EmptyOrder,
    #[error("Invalid state transition: {0}")]
    InvalidTransition(String),
}
