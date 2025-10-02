cargo add axum tokio sqlx uuid chrono serde serde_json serde_derive argon2 jsonwebtoken tracing tracing-subscriber
cargo add sqlx --features postgres,runtime-tokio-rustls,uuid,chrono


[package]
name = "user_auth_service"
version = "0.1.0"
edition = "2024"

[dependencies]
argon2 = "0.5.3"
axum = "0.8.4"
chrono = { version = "0.4.42", features = ["serde"] }
jsonwebtoken = "9.3.1"
serde = { version = "1.0.221", features = ["derive"] }
serde_derive = "1.0.221"
serde_json = "1.0.144"
sqlx = { version = "0.8.6", features = ["postgres", "runtime-tokio-rustls", "uuid", "chrono"] }
tokio = "1.47.1"
tracing = "0.1.41"
tracing-subscriber = "0.3.20"
uuid = { version = "1.18.1", features = ["serde"]}


cargo install sqlx-cli --no-default-features --features postgres

sqlx migrate add create_users

-- migrations/20230913120000_create_users.sql
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'user',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

123

DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres?options=-csearch_path=sqlx" sqlx migrate run

DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres?options=-csearch_path=sqlx" cargo sqlx prepare

APP_ENV=development cargo run
