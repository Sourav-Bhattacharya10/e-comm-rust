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

use config_utility::load_config::load_config;
use seeds::user_seed::seeding_users_data;

use crate::{models::app_state::AppState, routes::user_routes::user_routes};
use utility::password_hasher::hash_password;

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
        .route("/passwordhash", get(generate_password_hash))
        .nest("/users", user_routes())
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
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
