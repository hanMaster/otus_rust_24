use std::array::TryFromSliceError;
use std::io;
use std::net::AddrParseError;
use std::string::FromUtf8Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IncorrectAddress,
    FailToStartServer(io::Error),
    FailToConnect,
    FailGetInfo,
    FailTurnSocket,
    FailToParseAnswer,
    FailToParsePower,
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
    fn from(_: FromUtf8Error) -> Self {
        Error::FailToParseAnswer
    }
}

impl From<AddrParseError> for Error {
    fn from(_: AddrParseError) -> Self {
        Error::IncorrectAddress
    }
}

impl From<TryFromSliceError> for Error {
    fn from(_: TryFromSliceError) -> Self {
        Error::FailToParsePower
    }
}
