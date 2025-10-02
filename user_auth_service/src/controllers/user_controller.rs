use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    dtos::{create_user_dto::CreateUserDto, user_dto::UserDto},
    models::{app_state::AppState, user::User},
    repos::{
        repository_traits::{Create, Read},
        user_repo::UserRepo,
    },
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

    pub async fn create_user(
        State(app_state): State<Arc<AppState>>,
        Json(create_user_dto): Json<CreateUserDto>,
    ) -> Result<Json<UserDto>, ()> {
        let user_repo = UserRepo {
            pool: app_state.db_pool.clone(),
        };

        let new_user = User {
            id: Uuid::new_v4(),
            username: create_user_dto.username,
            email: create_user_dto.email,
            password_hash: create_user_dto.password_hash,
            role: create_user_dto.role,
            is_active: true,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        };

        let created_user = user_repo.create(new_user).await;

        match created_user {
            Ok(user) => {
                let user_dto = user.into_dto();
                Ok(Json(user_dto))
            }
            Err(_) => Err(()),
        }
    }
}
