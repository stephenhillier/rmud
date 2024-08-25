use diesel::prelude::*;
use diesel::PgConnection;

use crate::db::models::{NewRoom, Room, NewExit, Exit };

/// insert a new room
pub fn create_room(
    conn: &mut PgConnection,
    title: &str,
    description: &str,
) -> Room {
    use crate::db::schema::rooms;
    let new_room = NewRoom { title, description };
    diesel::insert_into(rooms::table)
        .values(&new_room)
        .get_result(conn)
        .expect("Error saving room")
}

/// link two rooms
pub fn create_exit(
    conn: &mut PgConnection,
    from_room: i32,
    dir: &str,
    to_room: i32,
    ) {
    use crate::db::schema::exits;
    let new_exit = NewExit { from_room, dir, to_room };
    let _: Exit = diesel::insert_into(exits::table)
        .values(&new_exit)
        .get_result(conn)
        .expect("Error saving exit");
}

/// get a room by id
pub fn get_room(conn: &mut PgConnection, room: i32) -> Room {
    use crate::db::schema::rooms::dsl::*;

    let room_db = rooms
        .filter(id.eq(room))
        .get_result::<Room>(conn)
        .expect("error loading room");
    room_db
}

/// for a given room (by id), return a vec containing possible exits
pub fn get_exits(conn: &mut PgConnection, room: i32) -> Vec<String> {
    use crate::db::schema::exits::dsl::*;

    let exits_db = exits
        .filter(from_room.eq(room))
        .load::<Exit>(conn)
        .expect("error loading exits")
        .iter()
        .map(|x| x.dir.clone())
        .collect();
    exits_db
}

/// get the room located in the direction `dir` of `room`
pub fn get_room_at_exit(conn: &mut PgConnection, room: i32, dir: &str) -> Room {
    use crate::db::schema::rooms::dsl::*;
    use crate::db::schema::exits;

    let target_room = rooms
        .inner_join(exits::table.on(exits::to_room.eq(id)))
        .filter(exits::from_room.eq(room))
        .filter(exits::dir.eq(dir))
        .select((id, title, description))
        .get_result::<Room>(conn)
        .expect("error loading room");
    target_room
}
