mod config_utility;
mod models;
mod repos;
mod seeds;

use sqlx::PgPool;
use tokio;

use config_utility::load_config::load_config;
use repos::{repository_traits::Read, user_repo::UserRepo};
use seeds::user_seed::seeding_users_data;

#[tokio::main]
async fn main() {
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

    let user_repo = UserRepo { pool: pg_pool };
    let users = user_repo.read_all().await.unwrap();
    println!("Users: {:?}", users);
}
