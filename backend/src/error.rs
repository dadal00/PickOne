use anyhow::Error as anyhowError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lettre::{
    address::AddressError, error::Error as lettreGeneralError,
    transport::smtp::Error as lettreTransportError,
};
use meilisearch_sdk::errors::Error as meiliError;
use prometheus::Error as prometheusError;
use redis::RedisError;
use scylla::{
    deserialize::DeserializationError,
    errors::{
        ExecutionError, FirstRowError, IntoRowsResultError, NewSessionError, PrepareError,
        RowsError,
    },
};
use serde_json::Error as serdeJsonError;
use std::{env::VarError, io::Error as IOError, num::ParseIntError, string::FromUtf8Error};
use thiserror::Error;
use tokio::task::JoinError;
use tokio_cron_scheduler::JobSchedulerError;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Environment error: {0}")]
    Environment(#[from] VarError),

    #[error("IO error: {0}")]
    IO(#[from] IOError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("Prometheus error: {0}")]
    Prometheus(#[from] prometheusError),

    #[error("ScyllaDB new session error: {0}")]
    ScyllaInit(#[from] NewSessionError),

    #[error("ScyllaDB execution error: {0}")]
    ScyllaExecute(#[from] ExecutionError),

    #[error("ScyllaDB prepare error: {0}")]
    ScyllaPrepare(#[from] PrepareError),

    #[error("ScyllaDB rows result error: {0}")]
    ScyllaRowsResult(#[from] IntoRowsResultError),

    #[error("ScyllaDB first row error: {0}")]
    ScyllaFirstRow(#[from] FirstRowError),

    #[error("ScyllaDB row error: {0}")]
    ScyllaRowsError(#[from] RowsError),

    #[error("ScyllaDB deserialization error: {0}")]
    ScyllaDeserializationError(#[from] DeserializationError),

    #[error("Tokio join error: {0}")]
    TokioJoin(#[from] JoinError),

    #[error("Lettre transport error: {0}")]
    LettreTransport(#[from] lettreTransportError),

    #[error("Lettre address error: {0}")]
    LettreAddress(#[from] AddressError),

    #[error("Lettre general error: {0}")]
    LettreGeneral(#[from] lettreGeneralError),

    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("SerdeJson error: {0}")]
    ToJson(#[from] serdeJsonError),

    #[error("ParseInt error: {0}")]
    ToInt(#[from] ParseIntError),

    #[error("MeiliError error: {0}")]
    MeiliError(#[from] meiliError),

    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhowError),

    #[error("TokioCron error: {0}")]
    TokioCron(#[from] JobSchedulerError),
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
