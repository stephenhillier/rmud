use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W
}

pub struct Room {
    pub id: usize,
    title: String,
    description: String,
    pub exits: HashMap<Direction, usize>
}

impl Room {
    pub fn new(id: usize, title: String, description: String, exits: HashMap<Direction, usize>) -> Self {
        Room{id, title, description, exits}
    }
    pub fn fmt(&self) -> String {
        format!("\
            {}\n\
            {}\n\
            ", self.title, self.description)
    }
}
