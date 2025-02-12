use crate::model;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),
    // -- ModelManager
    Model(model::Error),
    // -- Sqlx
    Sql(Arc<sqlx::Error>),
}

// region:    --- Froms

impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Error::Model(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::Sql(Arc::new(value))
    }
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("->> {:<12} - {self:?}", "INTO_RES");

        match self {
            Error::Sql(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something goes wrong".to_string(),
            )
                .into_response(),
        }
    }
}
