use diesel::prelude::*;
use rmud::areas::{Direction, Room};
use rmud::connections::Connection as MudConnection;
use rmud::mud::Mud;
use rmud::players::{Character, Player};
use std::str::FromStr;

fn get_database_url() -> String {
    dotenv::dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set.")
}

fn establish_connection(database_url: &str) -> Result<PgConnection, ConnectionError> {
    match PgConnection::establish(&database_url) {
        Ok(value) => Ok(value),
        Err(e) => {
            println!("Could not connect to database ({}).", database_url);
            Err(e)
        }
    }
}

fn new_conn() -> PgConnection {
    let database_url = get_database_url();
    establish_connection(&database_url).unwrap()
}

fn main() {
    println!("Running integration tests...");
    let conn = &mut new_conn();
    conn.begin_test_transaction().unwrap();

    let mud = &mut Mud::new();

    println!("\nTEST: creating rooms");
    let room_1 = Room::create(conn, "A field", "a path through a field");
    let room_2 = Room::create(conn, "A forest", "a path through a forest");

    println!("\nTEST: creating exit");
    room_1.create_exit(conn, "N", room_2.id);

    println!("\nTEST: creating player and character");
    let player = Player::create(conn, "test_player", Box::new(MudConnection {}));
    let mut character = Character::create(conn, "test_char", player, room_1.id);
    println!("player: {}", character.player());
    println!(
        "character: {} ({}) is in room {}",
        character.name,
        character.player(),
        character.room
    );

    println!("\nTEST: look at current room and nearby room");
    let current_room = Room::get(conn, character.room);
    println!("{}", current_room.fmt());

    let peeked_room = current_room.look_exit(conn, Direction::from_str("N").unwrap());
    println!("to the north is {}", peeked_room.title);

    println!("\nTEST: move player north");
    character.move_dir(conn, mud, Direction::from_str("N").unwrap());
    println!(
        "character: {} ({}) is in room {}",
        character.name,
        character.player(),
        character.room
    );


}
