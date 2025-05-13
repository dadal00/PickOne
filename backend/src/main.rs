use crate::{
    config::Config,
    error::AppError,
    metrics::{metrics_handler, Metrics},
    save::{load, save},
    signals::shutdown_signal,
    state::{AppState, Counters},
    websocket::websocket_handler,
};
use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, Method},
    routing::get,
    Router,
};
use std::{
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
};
use tokio::{net::TcpListener, sync::broadcast, time::interval};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

mod config;
mod error;
mod metrics;
mod save;
mod signals;
mod state;
mod websocket;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    fmt()
        .with_env_filter(
            EnvFilter::from_default_env(), // backend (target) = info (logging level)
        )
        .init();

    info!("Starting server...");

    let (broadcast_tx, _) = broadcast::channel(100);
    let state = Arc::new(AppState {
        metrics: Metrics::default(),
        counters: Counters::default(),
        concurrent_users: AtomicUsize::new(0),
        total_users: AtomicUsize::new(0),
        config: Config::load()?,
        broadcast_tx,
    });

    info!("Server configuration");
    info!("state_path = {}", state.config.state_path);
    info!("rust_port = {}", state.config.rust_port);
    info!("svelte_url = {}", state.config.svelte_url);

    load(State(state.clone()));

    let save_state = state.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60 * 30));
        loop {
            interval.tick().await;
            if let Err(e) = save(State(save_state.clone())).await {
                error!("Failed to save state: {}", e);
            }
        }
    });

    let origin_state = state.clone();
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin, _req| {
            info!("{}", origin.to_str().unwrap_or_default());
            origin.as_bytes() == origin_state.config.svelte_url.as_bytes()
        }))
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE])
        .max_age(Duration::from_secs(60 * 60));

    let app = Router::new()
        .route("/api/ws", get(websocket_handler))
        .route("/metrics", get(metrics_handler))
        .layer(cors)
        .with_state(state.clone());

    let addr = format!("0.0.0.0:{}", state.config.rust_port);
    info!("Binding to {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    info!("Server running on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    if let Err(e) = save(State(state.clone())).await {
        error!("Failed to save state: {}", e);
    }
    info!("Server shutdown complete");
    Ok(())
}
