use axum::{
    http::{header::InvalidHeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde_json::Error as jsonError;
use std::{env::VarError, io::Error as IOError};
use thiserror::Error;
use tracing::{dispatcher::SetGlobalDefaultError, error};
use tracing_subscriber::filter::ParseError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Environment error: {0}")]
    Environment(#[from] VarError),

    #[error("Network error: {0}")]
    Network(#[from] IOError),

    #[error("Invalid header value: {0}")]
    HeaderValue(#[from] InvalidHeaderValue),

    #[error("JSON serialization error: {0}")]
    Json(#[from] jsonError),

    #[error("Tracing filter parse error: {0}")]
    TracingFilterParse(#[from] ParseError),

    #[error("Tracing subscriber error: {0}")]
    TracingSubscriber(#[from] SetGlobalDefaultError),

    #[error("Configuration error: {0}")]
    Config(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = {
            error!("Server error: {}", self);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        };

        (status, message).into_response()
    }
}
