use axum::{routing::get, Router};

use crate::controllers::user_controller::UserController;

pub fn user_routes() -> Router {
    Router::new().route("/", get(UserController::get_all_users))
}
