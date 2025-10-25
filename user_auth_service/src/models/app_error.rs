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
    DatabaseConnectionFailure,
    NoUsersFound,
    UserNotFound,
    RequestPayloadNotValid(String),
    UserCouldNotBeCreated,
    UserCouldNotBeUpdated,
    UserCouldNotBeDeleted,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, body) = match self {
            AppError::UserNotFound => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorResponse {
                    code: String::from("UserNotFound"),
                    cause: "User not found".to_string(),
                    ..Default::default()
                },
            ),
            AppError::NoUsersFound => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorResponse {
                    code: String::from("NoUsersFound"),
                    cause: "No users found in database".to_string(),
                    ..Default::default()
                },
            ),
            AppError::UserCouldNotBeCreated => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorResponse {
                    code: String::from("UserCouldNotBeCreated"),
                    cause: "User could not be created".to_string(),
                    ..Default::default()
                },
            ),
            AppError::UserCouldNotBeUpdated => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorResponse {
                    code: String::from("UserCouldNotBeUpdated"),
                    cause: "User could not be updated".to_string(),
                    ..Default::default()
                },
            ),
            AppError::UserCouldNotBeDeleted => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorResponse {
                    code: String::from("UserCouldNotBeDeleted"),
                    cause: "User could not be deleted".to_string(),
                    ..Default::default()
                },
            ),
            AppError::RequestPayloadNotValid(err_string) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                AppErrorResponse {
                    code: String::from("RequestPayloadNotValidUnprocessableEntity"),
                    cause: err_string,
                    ..Default::default()
                },
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorResponse {
                    code: String::from("DatabaseConnectionFailure"),
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
    pub code: String,
    pub cause: String,
    pub stacktrace: Option<String>,
}

pub type CustomResult<T> = Result<T, AppError>;
