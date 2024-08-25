use std::collections::HashMap;

use crate::{players::Player, areas::Room};

pub struct Mud {
    pub players: HashMap<String, Player>,
    pub areas: HashMap<i32, Room>
}

impl Mud {
    pub fn new() -> Self {
        Mud{
            players: HashMap::new(),
            areas: HashMap::new()
        }
    }
    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.name().to_string(), player);
    }

    /// broadcast text to all players
    pub fn broadcast(&self, text: &str) {
        for (_, p) in &self.players {
            p.send(text)
        }
    }

    /// broadcast text to a list of players
    pub fn broadcast_players(&self, players: Vec<String>, text: &str) {
        for p in players {
            self.players[&p].send(text);
        }
    }
}

