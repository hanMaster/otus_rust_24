use thiserror::Error;

pub type Result<T> = core::result::Result<T, SmartHouseError>;

#[derive(Error, Debug, Clone)]
pub enum SmartHouseError {
    #[error("Room name incorrect")]
    InvalidName,
}
