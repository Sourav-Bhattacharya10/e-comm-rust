use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::{
    dtos::{
        create_user_dto::CreateUserDto, delete_user_dto::DeleteUserDto,
        update_user_dto::UpdateUserDto, user_dto::UserDto, user_is_active_dto::UserIsActiveDto,
    },
    models::{
        app_error::{AppError, CustomResult},
        app_state::AppState,
        paginated_response::PaginatedResponse,
        pagination::Pagination,
        user::User,
    },
    repos::{
        repository_traits::{Create, Delete, Read, Update},
        user_repo::UserRepo,
    },
    traits::into_dto::IntoDto,
};

pub struct UserController;

impl UserController {
    pub async fn get_all_users(
        State(app_state): State<Arc<AppState>>,
        Query(pagination): Query<Pagination>,
    ) -> CustomResult<Json<PaginatedResponse<UserDto>>> {
        let page = pagination.page.unwrap_or(1);
        let limit = pagination.limit.unwrap_or(3);
        let offset = (page - 1) * limit;
        let username = pagination.username.unwrap_or(String::from(""));
        let username_option = if username == "" { None } else { Some(username) };
        let order_by = pagination.order_by.unwrap_or(String::from("id"));

        let user_repo = UserRepo {
            pool: app_state.db_pool.clone(),
        };

        let users = user_repo
            .read_all(username_option, limit, offset, &order_by)
            .await;
        match users {
            Ok(users) => {
                let total_count_result = user_repo.count_total().await;
                let total_count = match total_count_result {
                    Ok(count) => count,
                    Err(_) => 0,
                };

                let users_dto = users.into_iter().map(|u| u.into_dto()).collect();

                return Ok(Json(PaginatedResponse {
                    data: users_dto,
                    limit: limit,
                    page: page,
                    order_by: order_by,
                    total: total_count,
                }));
            }
            Err(_) => Err(AppError::NO_USERS_FOUND),
        }
    }

    pub async fn get_user_by_id(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> CustomResult<Json<UserDto>> {
        let user_repo = UserRepo {
            pool: app_state.db_pool.clone(),
        };

        let found_user = user_repo.read(id).await;

        match found_user {
            Ok(Some(user)) => {
                let user_dto = user.into_dto();
                Ok(Json(user_dto))
            }
            Ok(None) => return Err(AppError::USER_NOT_FOUND),
            Err(_) => return Err(AppError::DATABASE_CONNECTION_FAILURE),
        }
    }

    pub async fn create_user(
        State(app_state): State<Arc<AppState>>,
        Json(create_user_dto): Json<CreateUserDto>,
    ) -> CustomResult<Json<UserDto>> {
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
            Err(_) => Err(AppError::USER_COULD_NOT_BE_CREATED),
        }
    }

    pub async fn update_user(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
        Json(update_user_dto): Json<UpdateUserDto>,
    ) -> CustomResult<Json<UserDto>> {
        let user_repo = UserRepo {
            pool: app_state.db_pool.clone(),
        };

        let existing_user = match user_repo.read(id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(AppError::USER_NOT_FOUND),
            Err(_) => return Err(AppError::DATABASE_CONNECTION_FAILURE),
        };

        let update_user = User {
            id: existing_user.id,
            username: update_user_dto.username,
            email: update_user_dto.email,
            password_hash: existing_user.password_hash,
            role: update_user_dto.role,
            is_active: update_user_dto.is_active,
            created_at: existing_user.created_at,
            updated_at: Some(chrono::Utc::now()),
        };

        let updated_user = user_repo.update(existing_user.id, update_user).await;

        match updated_user {
            Ok(user) => {
                let user_dto = user.into_dto();
                Ok(Json(user_dto))
            }
            Err(_) => Err(AppError::USER_COULD_NOT_BE_UPDATED),
        }
    }

    pub async fn delete_user(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> CustomResult<Json<DeleteUserDto>> {
        let user_repo = UserRepo {
            pool: app_state.db_pool.clone(),
        };

        let _existing_user = match user_repo.read(id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(AppError::USER_NOT_FOUND),
            Err(_) => return Err(AppError::DATABASE_CONNECTION_FAILURE),
        };

        let deleted_user = user_repo.delete(id).await;

        match deleted_user {
            Ok(user) => {
                let delete_user_dto = user.into_dto();
                Ok(Json(delete_user_dto))
            }
            Err(_) => Err(AppError::USER_COULD_NOT_BE_DELETED),
        }
    }

    pub async fn update_user_is_active(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
        Json(user_is_active_dto): Json<UserIsActiveDto>,
    ) -> CustomResult<Json<UserDto>> {
        let user_repo = UserRepo {
            pool: app_state.db_pool.clone(),
        };

        let _existing_user = match user_repo.read(id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(AppError::USER_NOT_FOUND),
            Err(_) => return Err(AppError::DATABASE_CONNECTION_FAILURE),
        };

        let updated_user_is_active = user_repo
            .update_is_active(id, user_is_active_dto.is_active)
            .await;

        match updated_user_is_active {
            Ok(user) => {
                let updated_user_is_active_dto = user.into_dto();
                Ok(Json(updated_user_is_active_dto))
            }
            Err(_) => Err(AppError::USER_COULD_NOT_BE_UPDATED),
        }
    }
}
