mod config_utility;
mod controllers;
mod dtos;
mod models;
mod repos;
mod routes;
mod seeds;
mod traits;
mod utility;

use axum::{
    Router,
    extract::{Query, State},
    routing::get,
};
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use tokio::net::TcpListener;

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config_utility::load_config::load_config;
use seeds::user_seed::seeding_users_data;

use crate::{models::app_state::AppState, routes::user_routes::user_routes};
use utility::password_hasher::hash_password;

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "user_auth_service=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json().with_writer(non_blocking_writer))
        .init();

    tokio::spawn(async move {
        let _ = _guard;
    });

    let config = load_config().unwrap();
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}?options=-csearch_path={}",
        config.server_user,
        config.server_password,
        config.server_host,
        config.server_port.to_string(),
        config.server_db_name,
        config.server_db_schema
    );

    let pg_pool = PgPool::connect(db_url.as_str()).await.unwrap();

    seeding_users_data(&pg_pool).await.unwrap();

    let user_repo = Arc::new(repos::user_repo::UserRepo {
        pool: pg_pool.clone(),
    });

    let shared_state = Arc::new(AppState {
        user_repo,
        db_pool: pg_pool.clone(),
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/passwordhash", get(generate_password_hash))
        .nest("/users", user_routes())
        .with_state(shared_state)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
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

    "User Auth Service is up and running!"
}

async fn generate_password_hash(
    State(_app_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> String {
    let password_hash = hash_password(&params.get("password").unwrap()).unwrap();

    password_hash
}
