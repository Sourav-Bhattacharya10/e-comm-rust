use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    dtos::user_dto::UserDto,
    models::{app_state::AppState, user::User},
    repos::{repository_traits::Read, user_repo::UserRepo},
    traits::into_dto::IntoDto,
};

pub struct UserController;

impl UserController {
    // pub fn new() -> Self {
    //     UserController
    // }

    // Handlers
    pub async fn get_all_users(
        State(app_state): State<Arc<AppState>>,
    ) -> Result<Json<Vec<UserDto>>, ()> {
        let user_repo = UserRepo {
            pool: app_state.db_pool.clone(),
        };
        let users = user_repo.read_all().await;
        match users {
            Ok(users) => {
                let users_dto = users.into_iter().map(|u| u.into_dto()).collect();
                Ok(Json(users_dto))
            }
            Err(_) => Err(()),
        }
    }

    pub async fn get_user_by_id(
        Path(id): Path<Uuid>,
        State(app_state): State<Arc<AppState>>,
    ) -> Result<Json<UserDto>, ()> {
        let user_repo = UserRepo {
            pool: app_state.db_pool.clone(),
        };

        let found_user = user_repo.read(id).await;

        match found_user {
            Ok(user) => {
                let user_dto = user.unwrap().into_dto();
                Ok(Json(user_dto))
            }
            Err(_) => Err(()),
        }
    }

    async fn create_user() -> &'static str {
        "Create user"
    }
}
