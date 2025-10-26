use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use tracing::instrument;

use crate::{
    dtos::{
        create_user_dto::CreateUserDto, deleted_user_dto::DeletedUserDto,
        update_user_dto::UpdateUserDto, user_dto::UserDto, user_is_active_dto::UserIsActiveDto,
    },
    models::{
        app_error::{AppError, CustomResult},
        app_state::AppState,
        paginated_response::PaginatedResponse,
        pagination::Pagination,
        user::User,
        validated_json::ValidatedJson,
    },
    repos::repository_traits::Repository,
    traits::into_dto::IntoDto,
};

pub struct UserController;

impl UserController {
    #[instrument(skip(user_repo))]
    pub async fn check_if_user_exists(
        user_repo: &Arc<dyn Repository<User, Uuid> + Send + Sync>,
        id: Uuid,
    ) -> CustomResult<User> {
        let existing_user = match user_repo.read(id).await {
            Ok(Some(user)) => user,
            Ok(_none) => return Err(AppError::UserNotFound),
            Err(_) => return Err(AppError::DatabaseConnectionFailure),
        };

        Ok(existing_user)
    }

    #[instrument(skip(app_state))]
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

        let user_repo = &app_state.user_repo;

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
            Err(_) => Err(AppError::NoUsersFound),
        }
    }

    #[instrument(skip(app_state))]
    pub async fn get_user_by_id(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> CustomResult<Json<UserDto>> {
        let user_repo = &app_state.user_repo;

        let found_user = Self::check_if_user_exists(user_repo, id).await;

        match found_user {
            Ok(user) => {
                let user_dto = user.into_dto();
                Ok(Json(user_dto))
            }
            Err(err) => return Err(err),
        }
    }

    #[instrument(skip(app_state, create_user_dto))]
    pub async fn create_user(
        State(app_state): State<Arc<AppState>>,
        ValidatedJson(create_user_dto): ValidatedJson<CreateUserDto>,
    ) -> CustomResult<Json<UserDto>> {
        let user_repo = &app_state.user_repo;

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
            Err(_) => Err(AppError::UserCouldNotBeCreated),
        }
    }

    #[instrument(skip(app_state, update_user_dto))]
    pub async fn update_user(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
        ValidatedJson(update_user_dto): ValidatedJson<UpdateUserDto>,
    ) -> CustomResult<Json<UserDto>> {
        let user_repo = &app_state.user_repo;

        let found_user = Self::check_if_user_exists(user_repo, id).await;

        let existing_user = match found_user {
            Ok(user) => user,
            Err(err) => return Err(err),
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
            Err(_) => Err(AppError::UserCouldNotBeUpdated),
        }
    }

    #[instrument(skip(app_state))]
    pub async fn delete_user(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
    ) -> CustomResult<Json<DeletedUserDto>> {
        let user_repo = &app_state.user_repo;

        let found_user = Self::check_if_user_exists(user_repo, id).await;

        let _existing_user = match found_user {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let deleted_user = user_repo.delete(id).await;

        match deleted_user {
            Ok(user) => {
                let deleted_user_dto = user.into_dto();
                Ok(Json(deleted_user_dto))
            }
            Err(_) => Err(AppError::UserCouldNotBeDeleted),
        }
    }

    #[instrument(skip(app_state, user_is_active_dto))]
    pub async fn update_user_is_active(
        State(app_state): State<Arc<AppState>>,
        Path(id): Path<Uuid>,
        Json(user_is_active_dto): Json<UserIsActiveDto>,
    ) -> CustomResult<Json<UserDto>> {
        let user_repo = &app_state.user_repo;

        let found_user = Self::check_if_user_exists(user_repo, id).await;

        let _existing_user = match found_user {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let updated_user_is_active = user_repo
            .update_is_active(id, user_is_active_dto.is_active)
            .await;

        match updated_user_is_active {
            Ok(user) => {
                let updated_user_is_active_dto = user.into_dto();
                Ok(Json(updated_user_is_active_dto))
            }
            Err(_) => Err(AppError::UserCouldNotBeUpdated),
        }
    }
}
