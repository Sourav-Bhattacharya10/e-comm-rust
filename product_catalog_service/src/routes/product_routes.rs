use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::{controllers::product_controller, models::app_state::AppState};

pub fn product_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            post(product_controller::create_product).get(product_controller::get_products),
        )
        .route(
            "/{id}",
            get(product_controller::get_product)
                .put(product_controller::update_product)
                .delete(product_controller::delete_product),
        )
}
