use diesel::prelude::*;

use crate::db::models::{Character, NewCharacter, NewPlayer, Player};

pub fn create_player(conn: &mut PgConnection, name: &str) -> Player {
    use crate::db::schema::players;
    let new_player = NewPlayer { name };
    diesel::insert_into(players::table)
        .values(&new_player)
        .get_result(conn)
        .expect("Error saving player")
}

pub fn get_player(conn: &mut PgConnection, player_name: &str) -> Player {
    use crate::db::schema::players::dsl::*;
    let player = players
        .filter(name.eq(player_name))
        .get_result::<Player>(conn)
        .expect("error loading player");
    println!("loading player {}", &player.name);
    player
}

pub fn create_character(
    conn: &mut PgConnection,
    player: &str,
    name: &str,
    room: &i32,
) -> Character {
    use crate::db::schema::characters;
    let new_character = NewCharacter { name, player, room };
    diesel::insert_into(characters::table)
        .values(&new_character)
        .get_result(conn)
        .expect("Error saving character")
}

pub fn get_character(conn: &mut PgConnection, char_name: &str) -> Character {
    use crate::db::schema::characters::dsl::*;
    let character = characters
        .filter(name.eq(char_name))
        .get_result::<Character>(conn)
        .expect("error loading character");

    println!("loading char {}", &character.name);
    character
}

pub fn get_characters_in_room(conn: &mut PgConnection, room_id: i32) -> Vec<Character> {
    use crate::db::schema::characters::dsl::*;
    let chars_present = characters
        .filter(room.eq(room_id))
        .load::<Character>(conn)
        .expect("error loading character");

    println!("loading chars {:?}", &chars_present);
    chars_present
}

/// updates the room for a character
pub fn set_room(conn: &mut PgConnection, char_name: &str, new_room: &i32) {
    use crate::db::schema::characters::dsl::*;
    diesel::update(characters.find(char_name)).set(room.eq(new_room))
        .get_result::<Character>(conn)
        .expect("error updating character");
    println!("updated {} to room {}", char_name, new_room);
}
