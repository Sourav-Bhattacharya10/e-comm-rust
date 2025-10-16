use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Default, Serialize)]
pub enum AppError {
    #[default]
    DATABASE_CONNECTION_FAILURE,
    NO_USERS_FOUND,
    USER_NOT_FOUND,
    REQUEST_PAYLOAD_NOT_VALID,
    USER_COULD_NOT_BE_CREATED,
    USER_COULD_NOT_BE_UPDATED,
    USER_COULD_NOT_BE_DELETED,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, body) = match self {
            AppError::USER_NOT_FOUND => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorResponse {
                    code: AppError::USER_NOT_FOUND,
                    cause: "User not found".to_string(),
                    ..Default::default()
                },
            ),
            AppError::NO_USERS_FOUND => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorResponse {
                    code: AppError::NO_USERS_FOUND,
                    cause: "No users found in database".to_string(),
                    ..Default::default()
                },
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorResponse {
                    code: AppError::DATABASE_CONNECTION_FAILURE,
                    cause: "Could not connect to database".to_string(),
                    ..Default::default()
                },
            ),
        };

        let serialized_json_body = Json(body);
        (status_code, serialized_json_body).into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "something went wrong")
    }
}

impl Error for AppError {}

#[derive(Default, Serialize)]
pub struct AppErrorResponse {
    pub code: AppError,
    pub cause: String,
    pub stacktrace: Option<String>,
}

pub type CustomResult<T> = Result<T, AppError>;
