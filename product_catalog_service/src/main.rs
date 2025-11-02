mod config_utility;
mod controllers;
mod dtos;
mod models;
mod repos;
mod routes;
mod seeds;
mod traits;
mod utility;

use axum::{Router, extract::State, routing::get};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config_utility::load_config::load_config;
use seeds::product_seed::seeding_products_data;

use crate::{models::app_state::AppState, routes::product_routes::product_routes};

#[tokio::main]
async fn main() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "product_catalog_service=debug,tower_http=debug".into());

    let log_output = std::env::var("LOG_OUTPUT").unwrap_or_else(|_| "file".to_string());

    let _guard = if log_output.to_lowercase() == "console" {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
        None
    } else {
        let file_appender = tracing_appender::rolling::daily("logs", "app.log");
        let (non_blocking_writer, guard) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(non_blocking_writer),
            )
            .init();

        Some(guard)
    };

    let config = load_config().unwrap();
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}?options=-csearch_path={}",
        config.server_user,
        config.server_password,
        config.server_host,
        config.server_port,
        config.server_db_name,
        config.server_db_schema
    );

    let pg_pool = PgPool::connect(db_url.as_str()).await.unwrap();

    seeding_products_data(&pg_pool).await.unwrap();

    let product_repo = Arc::new(repos::product_repo::ProductRepo {
        pool: pg_pool.clone(),
    });

    let shared_state = Arc::new(AppState {
        product_repo,
        db_pool: pg_pool.clone(),
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/products", product_routes())
        .with_state(shared_state)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}

async fn health_check(State(app_state): State<Arc<AppState>>) -> &'static str {
    let _conn = app_state
        .db_pool
        .try_acquire()
        .expect("Failed to acquire a connection");

    "Product Catalog Service is up and running!"
}
