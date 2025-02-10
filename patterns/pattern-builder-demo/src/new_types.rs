use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq)]
#[error("Износ устройства значение должно быть в диапазоне от 0.0 до 1.0, но передано {0}")]
pub struct HealthError(f32);

/// Износ устройства
/// значение от 0.0 до 1.0
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Health(f32);

impl Health {
    pub fn new(val: f32) -> Result<Self, HealthError> {
        if !(0.0..1.0).contains(&val) {
            Err(HealthError(val))
        } else {
            Ok(Self(val))
        }
    }
}