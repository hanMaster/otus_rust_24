use charts_rs::CanvasError;
use std::env::VarError;
use std::num::{ParseFloatError, ParseIntError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Env(dotenvy::Error),
    Var(VarError),
    Fetch(reqwest::Error),
    Canvas(CanvasError),
    Io(std::io::Error),
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
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

impl From<CanvasError> for Error {
    fn from(value: CanvasError) -> Self {
        Error::Canvas(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<ParseFloatError> for Error{
    fn from(value: ParseFloatError) -> Self {
        Error::ParseFloat(value)
    }
}

impl From<ParseIntError> for Error{
    fn from(value: ParseIntError) -> Self {
        Error::ParseInt(value)
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
