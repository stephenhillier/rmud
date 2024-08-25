use diesel::PgConnection;
use serde::Deserialize;

use crate::{db, mud::Mud};
use core::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
    U,
    D,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input.to_lowercase().as_str() {
            "n" | "north" => Ok(Direction::N),
            "e" | "east" => Ok(Direction::E),
            "s" | "south" => Ok(Direction::S),
            "w" | "west" => Ok(Direction::W),
            "u" | "up" => Ok(Direction::U),
            "d" | "down" => Ok(Direction::D),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::N => write!(f, "N"),
            Direction::E => write!(f, "E"),
            Direction::S => write!(f, "S"),
            Direction::W => write!(f, "W"),
            Direction::U => write!(f, "U"),
            Direction::D => write!(f, "D"),
        }
    }
}

#[derive(Deserialize)]
pub struct Room {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub exits: Vec<Direction>,
}

impl Room {
    pub fn new(id: i32, title: String, description: String, exits: Vec<Direction>) -> Self {
        Room {
            id,
            title,
            description,
            exits,
        }
    }
    pub fn from_db(room_db: db::models::Room, exits: Vec<String>) -> Self {
        let exits = exits
            .iter()
            .map(|x| Direction::from_str(&x).unwrap())
            .collect();
        Self::new(room_db.id, room_db.title, room_db.description, exits)
    }
    pub fn create(conn: &mut PgConnection, title: &str, description: &str) -> Self {
        let room_db = db::areas::create_room(conn, title, description);
        Self::from_db(room_db, Vec::new())
    }
    pub fn broadcast(&self, conn: &mut PgConnection, server: &Mud, text: &str) {
        let chars = db::players::get_characters_in_room(conn, self.id);
        let players = chars.into_iter().map(|x| x.player).collect();
        server.broadcast_players(players, text);
    }
    pub fn create_exit(self: &Self, conn: &mut PgConnection, dir: &str, to_room: i32) {
        db::areas::create_exit(conn, self.id, dir, to_room)
    }

    pub fn get(conn: &mut PgConnection, id: i32) -> Self {
        let room_db = db::areas::get_room(conn, id);
        let exits_db = db::areas::get_exits(conn, id);
        Self::from_db(room_db, exits_db)
    }


    /// returns the room in the direction of exit
    pub fn look_exit(self: Self, conn: &mut PgConnection, dir: Direction) -> Room {
        let room_db = db::areas::get_room_at_exit(conn, self.id, dir.to_string().as_str());
        Self::from_db(room_db, Vec::new())
    }

    pub fn fmt(&self) -> String {
        format!(
            "\n\
            {}\n\n\
            {}\n\n\
            Exits: {:?}\n\
            ",
            self.title, self.description, self.exits
        )
    }
}

pub fn get_room_at_exit(conn: &mut PgConnection, from_room: i32, dir: &Direction) -> Room {
    let new_room = db::areas::get_room_at_exit(conn, from_room, dir.to_string().as_str());
    let exits = db::areas::get_exits(conn, new_room.id);
    Room::from_db(new_room, exits)
}
