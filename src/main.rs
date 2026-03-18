mod handlers;
mod models;
mod store;

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use axum::http::{header, HeaderValue, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Router};
use clap::Parser;
use notify::{recommended_watcher, EventKind, RecursiveMode, Watcher};
use rust_embed::RustEmbed;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

use store::MessageStore;

#[derive(Parser)]
#[command(name = "cc-msg-viewer", about = "Claude Code message history viewer")]
struct Cli {
    /// Path to history.jsonl
    #[arg(long, env = "HISTORY_PATH")]
    history_file: Option<PathBuf>,

    /// Port to listen on
    #[arg(long, default_value_t = 3001, env = "PORT")]
    port: u16,
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

async fn serve_static(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => match Assets::get("index.html") {
            Some(index) => (
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                index.data,
            )
                .into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
    }
}

fn spawn_file_watcher(path: PathBuf, store: Arc<RwLock<MessageStore>>) {
    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = match recommended_watcher(tx) {
            Ok(w) => w,
            Err(e) => {
                tracing::warn!("File watcher unavailable: {e}");
                return;
            }
        };
        if let Err(e) = watcher.watch(&path, RecursiveMode::NonRecursive) {
            tracing::warn!("Cannot watch {}: {e}", path.display());
            return;
        }
        tracing::info!("Watching {} for changes", path.display());
        for event in rx.into_iter().flatten() {
            if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                match MessageStore::load(&path) {
                    Ok(new_store) => {
                        if let Ok(mut lock) = store.write() {
                            *lock = new_store;
                            tracing::info!("History reloaded from {}", path.display());
                        }
                    }
                    Err(e) => tracing::error!("Reload failed: {e}"),
                }
            }
        }
    });
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("cc_msg_viewer=info".parse()?))
        .init();

    let cli = Cli::parse();

    let history_path = cli.history_file.unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        PathBuf::from(home).join(".claude").join("history.jsonl")
    });

    tracing::info!("Loading history from {}", history_path.display());
    let store = MessageStore::load(&history_path)?;
    let store = Arc::new(RwLock::new(store));

    spawn_file_watcher(history_path, store.clone());

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>()?)
        .allow_methods([axum::http::Method::GET])
        .allow_headers(tower_http::cors::Any);

    let api_router = Router::new()
        .route("/messages", get(handlers::get_messages))
        .route("/projects", get(handlers::get_projects))
        .route("/sessions", get(handlers::get_sessions))
        .route("/stats", get(handlers::get_stats))
        .layer(cors)
        .with_state(store);

    let app = Router::new()
        .nest("/api", api_router)
        .fallback(serve_static);

    let addr = format!("0.0.0.0:{}", cli.port);
    tracing::info!("Listening on {addr}  →  http://localhost:{}", cli.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
