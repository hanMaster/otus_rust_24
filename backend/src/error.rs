use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;
use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),
    // -- ModelManager
    Model(model::Error),
}

// region:    --- Froms
impl From<model::Error> for Error {
    fn from(value: model::Error) -> Self {
        Error::Model(value)
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
        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        // Insert the Error into the response
        response.extensions_mut().insert(self);
        response
    }
}
