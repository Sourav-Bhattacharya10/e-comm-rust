use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
    response::IntoResponse
};
use uuid::Uuid;

use crate::{
    dtos::{create_product_dto::CreateProductDto, update_product_dto::UpdateProductDto, product_dto::ProductDto},
    models::{app_state::AppState, pagination::Pagination, app_error::AppError, paginated_response::PaginatedResponse},
    repos::repository_traits::Repository,
    traits::to_dto::ToDto,
};

pub async fn create_product(
    State(app_state): State<Arc<AppState>>,
    Json(create_product_dto): Json<CreateProductDto>,
) -> Result<impl IntoResponse, AppError> {
    let product = app_state
        .product_repo
        .create(&create_product_dto)
        .await?;

    Ok((StatusCode::CREATED, Json(product.to_dto())))
}

pub async fn get_products(
    State(app_state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> Result<Json<PaginatedResponse<ProductDto>>, AppError> {
    let paginated_response = app_state
        .product_repo
        .get_all(&pagination)
        .await?;

    let product_dtos = paginated_response
        .data
        .iter()
        .map(|product| product.to_dto())
        .collect();

    Ok(Json(PaginatedResponse {
        page: paginated_response.page,
        per_page: paginated_response.per_page,
        total: paginated_response.total,
        total_pages: paginated_response.total_pages,
        data: product_dtos,
    }))
}

pub async fn get_product(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProductDto>, AppError> {
    let product = app_state
        .product_repo
        .get_by_id(id)
        .await?;

    Ok(Json(product.to_dto()))
}

pub async fn update_product(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(update_product_dto): Json<UpdateProductDto>,
) -> Result<Json<ProductDto>, AppError> {
    let product = app_state
        .product_repo
        .update(id, &update_product_dto)
        .await?;

    Ok(Json(product.to_dto()))
}

pub async fn delete_product(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    app_state
        .product_repo
        .delete(id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
