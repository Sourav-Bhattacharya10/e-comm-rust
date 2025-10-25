use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::ops::Deref;
use validator::Validate;

use crate::models::app_error::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T> Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: Validate + DeserializeOwned + Send,
{
    type Rejection = AppError;

    fn from_request(
        req: Request,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let Json(value) = Json::<T>::from_request(req, state)
                .await
                .map_err(|err| AppError::RequestPayloadNotValid(err.to_string()))?;

            value
                .validate()
                .map_err(|err| AppError::RequestPayloadNotValid(err.to_string()))?;

            Ok(ValidatedJson(value))
        }
    }
}
