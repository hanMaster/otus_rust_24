use std::io;
use std::string::FromUtf8Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailToStartServer(io::Error),
    FailToConnect,
    FailGetInfo,
    FailTurnSocket,
    FailToParseAnswer(FromUtf8Error),
}

//region      --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
//endregion   --- Error Boilerplate

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::FailToStartServer(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::FailToParseAnswer(value)
    }
}
