use std::sync::Arc;

use axum::{extract::Extension, Json};

use crate::{
    models::{user::User, AppState},
    repos::{repository_traits::Read, user_repo::UserRepo},
};

pub struct UserController;

impl UserController {
    pub fn new() -> Self {
        UserController
    }

    // Handlers
    pub async fn get_all_users(
        Extension(app_state): Extension<Arc<AppState>>,
    ) -> Result<Json<Vec<User>>, ()> {
        let user_repo = UserRepo {
            pool: app_state.db_pool.clone(),
        };
        let users = user_repo.read_all().await;
        match users {
            Ok(users) => Ok(Json(users)),
            Err(_) => Err(()),
        }
    }

    async fn get_user_by_id() -> &'static str {
        "User by ID"
    }

    async fn create_user() -> &'static str {
        "Create user"
    }
}