use std::collections::HashMap;

use crate::{players::Player, areas::Room};

pub struct Mud {
    players: Vec<Player>,
    pub areas: HashMap<usize, Room>
}

impl Mud {
    pub fn new() -> Self {
        Mud{
            players: Vec::new(),
            areas: HashMap::new()
        }
    }
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn broadcast(&self, text: &str) {
        for p in &self.players {
            p.send(text)
        }
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }
}

