use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{controllers::user_controller::UserController, models::app_state::AppState};

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(UserController::get_all_users).post(UserController::create_user),
        )
        .route(
            "/{id}",
            get(UserController::get_user_by_id).put(UserController::update_user),
        )
}
