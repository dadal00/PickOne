use crate::{
    api::{
        bot::{chat::start_bot, photo::photo_handler},
        microservices::{
            cdc::{RedisCDCParams, ScyllaCDCParams, start_cdc},
            database::schema::{BOILER_SWAP_KEYSPACE, columns::boiler_swap::items, tables},
        },
        web::{
            handlers::{
                api_token_check, authenticate_handler, delete_handler, forgot_handler,
                resend_handler, verify_handler, visitors_handler,
            },
            models::{METRICS_ROUTE, RedisAction, WebsitePath, WebsiteRoute},
            swap::handlers::post_item_handler,
        },
    },
    error::AppError,
    metrics::{RedisMetricAction, metrics_handler},
    signals::shutdown_signal,
    state::AppState,
};
use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
    middleware,
    routing::{delete, get, post},
};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

mod api;
mod config;
mod error;
mod metrics;
mod signals;
mod state;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    fmt()
        .with_env_filter(
            EnvFilter::from_default_env(), // backend (target) = info (logging level)
        )
        .init();

    info!("Starting server...");

    let (state, meili_reindex_future) = AppState::new().await?;

    start_bot(state.clone()).await?;

    info!("Server configuration");
    info!("rust_port = {}", state.config.server.rust_port);
    info!("svelte_url = {}", state.config.server.svelte_url);

    let origin_state = state.clone();
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin, _req| {
            origin.as_bytes() == origin_state.config.server.svelte_url.as_bytes()
        }))
        .allow_methods([Method::GET, Method::OPTIONS, Method::POST, Method::DELETE])
        .allow_headers([CONTENT_TYPE])
        .max_age(Duration::from_secs(60 * 60));

    let app = Router::new()
        .route(
            &format!(
                "/{}/{}/visitors",
                WebsitePath::Home.as_ref(),
                WebsiteRoute::Api.as_ref()
            ),
            post(visitors_handler),
        )
        .route(
            &format!(
                "/{}/{}/{}",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Authenticate.as_ref()
            ),
            post(authenticate_handler),
        )
        .route(
            &format!(
                "/{}/{}/{}",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Verify.as_ref()
            ),
            post(verify_handler),
        )
        .route(
            &format!(
                "/{}/{}/{}",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Delete.as_ref()
            ),
            delete(delete_handler),
        )
        .route(
            &format!(
                "/{}/{}/{}",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Forgot.as_ref()
            ),
            post(forgot_handler),
        )
        .route(
            &format!(
                "/{}/{}/post-item",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref()
            ),
            post(post_item_handler),
        )
        .route(
            &format!(
                "/{}/{}/{}",
                WebsitePath::BoilerSwap.as_ref(),
                WebsiteRoute::Api.as_ref(),
                WebsiteRoute::Resend.as_ref()
            ),
            post(resend_handler),
        )
        .route(
            &format!("/{}/:id", WebsitePath::Photos.as_ref()),
            get(photo_handler),
        )
        .route(METRICS_ROUTE, get(metrics_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            api_token_check,
        ))
        .layer(cors)
        .with_state(state.clone())
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = format!("0.0.0.0:{}", state.config.server.rust_port);
    info!("Binding to {}", addr);

    let listener = TcpListener::bind(&addr).await?;

    meili_reindex_future.await??;

    let (mut cdc_reader, cdc_future) = start_cdc(
        state.clone(),
        ScyllaCDCParams {
            keyspace: BOILER_SWAP_KEYSPACE.to_string(),
            table: tables::boiler_swap::ITEMS.to_string(),
            id_name: items::ITEM_ID.to_string(),
        },
        WebsitePath::BoilerSwap,
        RedisCDCParams {
            metric: RedisMetricAction::Items.as_ref().to_string(),
            deletion_name: RedisAction::DeletedItem.as_ref().to_string(),
            metric_prefix: RedisAction::Metric.as_ref().to_string(),
        },
    )
    .await?;

    info!("Server running on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    cdc_reader.stop();

    Ok(cdc_future.await?)
}
