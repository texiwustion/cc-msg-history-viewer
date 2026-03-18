mod handlers;
mod models;
mod store;

use std::path::PathBuf;

use axum::{routing::get, Router};
use axum::http::HeaderValue;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("cc_msg_viewer=info".parse()?))
        .init();

    let history_path = std::env::var("HISTORY_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
            PathBuf::from(home).join(".claude").join("history.jsonl")
        });

    tracing::info!("Loading history from {}", history_path.display());
    let store = store::MessageStore::load(&history_path)?;

    // Only allow the Vite dev server origin; in production, the frontend is
    // served from the same Axum process so CORS is not needed at all.
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>()?)
        .allow_methods([axum::http::Method::GET])
        .allow_headers(tower_http::cors::Any);

    let api_router = Router::new()
        .route("/messages", get(handlers::get_messages))
        .route("/projects", get(handlers::get_projects))
        .route("/sessions", get(handlers::get_sessions))
        .route("/stats", get(handlers::get_stats));

    let app = Router::new()
        .nest("/api", api_router)
        .layer(cors)
        .with_state(store);

    let addr = "0.0.0.0:3001";
    tracing::info!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
