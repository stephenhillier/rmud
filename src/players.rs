use crate::areas::{get_room_at_exit, Direction, Room};
use crate::db;
use crate::mud::Mud;
use crate::{connections::Sender, db::models::Player as PlayerDB};

pub struct Player {
    name: String,
    connection: Box<dyn Sender>,
}

/// a player connected to the MUD
impl Player {
    /// create a new player instance, with a handle to their connection.
    pub fn new(name: &str, connection: Box<dyn Sender>) -> Self {
        Player {
            name: name.into(),
            connection,
        }
    }
    /// from_db converts a player saved in the database into a Player,
    /// adding the connection.
    pub fn from_db(player: PlayerDB, connection: Box<dyn Sender>) -> Self {
        Self::new(&player.name, connection)
    }

    pub fn create(
        db_conn: &mut diesel::PgConnection,
        name: &str,
        connection: Box<dyn Sender>,
    ) -> Self {
        let player_db = db::players::create_player(db_conn, name);
        Self::from_db(player_db, connection)
    }

    /// send text to the player
    pub fn send(&self, text: &str) {
        self.connection.send(text);
    }
    /// return the player's name
    pub fn name(&self) -> &str {
        return &self.name;
    }
}

pub struct Character {
    pub name: String,
    pub player: Player,
    pub room: i32,
}

impl Character {
    pub fn new(name: &str, player: Player, room: i32) -> Character {
        Character {
            name: name.into(),
            player,
            room,
        }
    }
    pub fn from_db(character: db::models::Character, player: Player, room: i32) -> Self {
        Self::new(&character.name, player, room)
    }

    pub fn create(
        db_conn: &mut diesel::PgConnection,
        name: &str,
        player: Player,
        room: i32,
    ) -> Self {
        let player_db = db::players::create_character(db_conn, &player.name, name, &room);
        Self::from_db(player_db, player, room)
    }

    // move to the room located in the direction `dir`.
    pub fn move_dir(&mut self, conn: &mut diesel::PgConnection, server: &Mud, dir: Direction) {
        let new_room = get_room_at_exit(conn, self.room, &dir);
        new_room.broadcast(conn, server, &self.leave_text(dir));
        self.player.send(&new_room.fmt());
    }

    pub fn get_room(&self, conn: &mut diesel::PgConnection) -> Room {
        Room::get(conn, self.room)
    }
    
    pub fn set_room(&mut self, conn: &mut diesel::PgConnection, room: Room) {
        db::players::set_room(conn, &self.name, &room.id);
        self.room = room.id;
    }

    pub fn player(&self) -> String {
        self.player.name.clone()
    }

    fn leave_text(&self, dir: Direction) -> String {
        format!("{} leaves {}", self.name, dir)

    }
}
