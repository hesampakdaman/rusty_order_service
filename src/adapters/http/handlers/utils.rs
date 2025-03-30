use crate::domain::errors::Error;
use axum::http::StatusCode;

pub(crate) fn map_domain_error(err: &Error) -> (StatusCode, String) {
    match err {
        Error::EmptyOrder => (
            StatusCode::BAD_REQUEST,
            "Order must have at least one item".into(),
        ),
        Error::InvalidTransition(details) => (
            StatusCode::BAD_REQUEST,
            format!("Invalid state transition: {}", details),
        ),
        Error::OrderNotFound(details) => (
            StatusCode::NOT_FOUND,
            format!("No order found: {}", details),
        ),
        Error::InvalidOrderType(details) => (
            StatusCode::CONFLICT,
            format!("Invalid order type: {}", details),
        ),
        Error::MissingField(field) => {
            (StatusCode::BAD_REQUEST, format!("Missing field: {}", field))
        }
        Error::RepositoryBackendFailure(details) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Repository failure: {}", details),
        ),
    }
}
