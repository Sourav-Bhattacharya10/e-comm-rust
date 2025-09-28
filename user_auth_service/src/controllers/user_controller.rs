use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    models::{AppState, user::User},
    repos::{repository_traits::Read, user_repo::UserRepo},
};

pub struct UserController;

impl UserController {
    // pub fn new() -> Self {
    //     UserController
    // }

    // Handlers
    pub async fn get_all_users(
        State(app_state): State<Arc<AppState>>,
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

    pub async fn get_user_by_id(
        Path(id): Path<Uuid>,
        State(app_state): State<Arc<AppState>>,
    ) -> Result<Json<User>, ()> {
        let user_repo = UserRepo {
            pool: app_state.db_pool.clone(),
        };

        let found_user = user_repo.read(id).await;

        match found_user {
            Ok(user) => Ok(Json(user.unwrap())),
            Err(_) => Err(()),
        }
    }

    async fn create_user() -> &'static str {
        "Create user"
    }
}
