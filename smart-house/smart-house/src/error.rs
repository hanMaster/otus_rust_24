use std::fmt::{Display, Formatter};

pub type Result<T> = core::result::Result<T, SmartHouseError>;

#[derive(Debug, Clone)]
pub enum SmartHouseError {
    InvalidName,
}

impl Display for SmartHouseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for SmartHouseError {}
