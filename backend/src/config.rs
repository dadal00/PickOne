use crate::error::AppError;
use std::{env, fs::read_to_string};
use tracing::{info, warn};

pub const MAX_BYTES: u8 = 10;

#[derive(Debug, Clone)]
pub struct Config {
    pub rust_port: u16,
    pub svelte_url: String,
    pub state_path: String,
    pub max_connections_per_ip: u8,
    pub hash_salt: String,
    pub timeout_min: u8,
    pub msg_delay_ms: u8,
}

impl Config {
    pub fn load() -> Result<Self, AppError> {
        let rust_port = var("RUST_PORT")
            .inspect_err(|_| {
                info!("RUST_PORT not set, using default");
            })
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid RUST_PORT value".into()))?;

        let svelte_url = var("SVELTE_URL")
            .inspect_err(|_| {
                info!("SVELTE_URL not set, using default");
            })
            .unwrap_or_else(|_| "http://localhost:5173".into());

        let state_path = var("RUST_STATE_PATH")
            .inspect_err(|_| {
                info!("RUST_STATE_PATH not set, using default");
            })
            .unwrap_or_else(|_| "/saved_state.json".into());

        let max_connections_per_ip = var("RUST_MAX_CONNECTIONS_PER_IP")
            .inspect_err(|_| {
                info!("RUST_MAX_CONNECTIONS_PER_IP not set, using default");
            })
            .unwrap_or_else(|_| "5".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid RUST_MAX_CONNECTIONS_PER_IP value".into()))?;

        let hash_salt = read_hash_salt()
            .inspect_err(|_| {
                info!("RUST_HASH_SALT not set, using default");
            })
            .unwrap_or_else(|_| "WeAreInTroubleGoodnessGracious".into());

        let timeout_min = var("RUST_TIMEOUT_MIN")
            .inspect_err(|_| {
                info!("RUST_TIMEOUT_MIN not set, using default");
            })
            .unwrap_or_else(|_| "5".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid RUST_TIMEOUT_MIN value".into()))?;

        let msg_delay_ms = var("RUST_MSG_DELAY_MS")
            .inspect_err(|_| {
                info!("RUST_MSG_DELAY_MS not set, using default");
            })
            .unwrap_or_else(|_| "100".into())
            .parse()
            .map_err(|_| AppError::Config("Invalid RUST_MSG_DELAY_MS value".into()))?;

        Ok(Self {
            rust_port,
            svelte_url,
            state_path,
            max_connections_per_ip,
            hash_salt,
            timeout_min,
            msg_delay_ms,
        })
    }
}

fn var(key: &str) -> Result<String, AppError> {
    env::var(key).map_err(|e| {
        warn!("Environment variable {} not found, using default", key);
        AppError::Environment(e)
    })
}

fn read_hash_salt() -> Result<String, AppError> {
    read_to_string("/run/secrets/RUST_HASH_SALT")
        .map(|s| s.trim().to_string())
        .map_err(|e| {
            warn!("Failed to read RUST_HASH_SALT from file: {}", e);
            AppError::IO(e)
        })
}
