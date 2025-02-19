use std::env::VarError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Env(dotenvy::Error),
    Var(VarError),
    Fetch(reqwest::Error)
}

//region      --- From
impl From<dotenvy::Error> for Error {
    fn from(value: dotenvy::Error) -> Self {
        Error::Env(value)
    }
}
impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        Error::Var(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Fetch(value)
    }
}
//endregion   --- From

//region      --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
//endregion   --- Error Boilerplate
