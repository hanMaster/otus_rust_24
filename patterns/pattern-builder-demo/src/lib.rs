#[derive(Debug, PartialEq)]
pub struct Socket {
    title: String,
    is_on: bool,
    power: f32,
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
}

impl SocketBuilder {
    pub fn new() -> SocketBuilder {
        Self::default()
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn build(self) -> Socket {
        Socket {
            title: self.title,
            is_on: self.is_on,
            power: self.power,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_builder() {
        let manual_socket = Socket {
            title: "Kitchen socket".to_string(),
            is_on: false,
            power: 0.0,
        };
        let socket = Socket::builder().with_title("Kitchen socket".to_string()).build();
        assert_eq!(manual_socket, socket);
    }
}
