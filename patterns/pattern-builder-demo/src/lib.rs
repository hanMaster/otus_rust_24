use crate::new_types::{Health, HealthError};

mod new_types;

#[derive(Debug, PartialEq)]
pub struct Socket {
    title: String,
    is_on: bool,
    power: f32,
    health: Health,
}

impl Socket {
    pub fn builder() -> SocketBuilder {
        SocketBuilder::default()
    }
}

#[derive(Default)]
pub struct SocketBuilder {
    title: String,
    is_on: bool,
    power: f32,
    health: Health,
}

impl SocketBuilder {
    pub fn new() -> SocketBuilder {
        Self::default()
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_health(mut self, health: f32) -> Result<Self, HealthError> {
        let health = Health::new(health)?;
        self.health = health;
        Ok(self)
    }

    pub fn build(self) -> Socket {
        Socket {
            title: self.title,
            is_on: self.is_on,
            power: self.power,
            health: self.health,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_builder() {
        let health = Health::new(0.9).unwrap();
        let manual_socket = Socket {
            title: "Kitchen socket".to_string(),
            is_on: false,
            power: 0.0,
            health,
        };
        let socket = Socket::builder()
            .with_title("Kitchen socket".to_string())
            .with_health(0.9)
            .unwrap()
            .build();
        assert_eq!(manual_socket, socket);
    }

    #[test]
    fn test_new_invalid_health() {
        let raw_health = 3.6;
        let result = Health::new(raw_health);
        assert!(result.is_err());
    }
}
