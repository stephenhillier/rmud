use crate::connections::Sender;

pub struct Player {
    name: String,
    connection: Box<dyn Sender>,
}

impl Player {
    pub fn new(name: String, connection: Box<dyn Sender>) -> Self {
        Player {
            name: name.into(),
            connection
        }
    }

    pub fn send(&self, text: &str) {
        self.connection.send(text);
    }
    pub fn name(&self) -> &str {
        return &self.name
    }
}

