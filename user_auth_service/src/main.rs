mod config_utility;
mod controllers;
mod models;
mod repos;
mod routes;
mod seeds;

use axum::{Extension, Router, routing::get};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;

use config_utility::load_config::load_config;
use seeds::user_seed::seeding_users_data;

use crate::{models::AppState, routes::user_routes::user_routes};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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

    let shared_state = Arc::new(AppState {
        db_pool: pg_pool.clone(),
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/users", user_routes())
        .layer(Extension(shared_state));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    tracing::info!("Server started on port 8080");
    println!("Server started on port 8080");
}

async fn health_check(Extension(app_state): Extension<Arc<AppState>>) -> &'static str {
    let _conn = app_state
        .db_pool
        .try_acquire()
        .expect("Failed to acquire a connection");

    "User Auth Service is up and running!"
}
