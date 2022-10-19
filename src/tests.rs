use std::collections::HashMap;

use crate::areas::{Direction, Room};
use crate::connections::Connection;
use crate::mud::Mud;
use crate::players::Player;

#[test]
fn test_add_player() {
    let mut mud = Mud::new();
    let p1 = Player::new(String::from("test"), Connection::new());
    mud.add_player(p1);
    mud.broadcast("test");

    assert_eq!(mud.players()[0].name(), "test");
}

#[test]
fn test_room_fmt() {
    let title = String::from("a room");
    let description = String::from("description");
    let exits = HashMap::new();

    let room = Room::new(1, title.clone(), description.clone(), exits);
    println!("{}", room.fmt());

    assert_eq!(room.fmt().contains(&title), true);
    assert_eq!(room.fmt().contains(&description), true);
}

#[test]
fn test_two_rooms() {
    let mut mud = Mud::new();
    let room1_id = 1;
    let room2_id = 2;
    let mut room1 = Room::new(room1_id, "room1".into(), "room one".into(), HashMap::new());
    let mut room2 = Room::new(room2_id, "room2".into(), "room two".into(), HashMap::new());

    room1.exits.insert(Direction::N, room2.id);
    room2.exits.insert(Direction::S, room1.id);

    mud.areas.insert(room1.id, room1);
    mud.areas.insert(room2.id, room2);

    // test assertion stub until there is more complex functionality
    assert_eq!(
        *mud.areas
            .get(&room1_id)
            .expect("room 1 should exist")
            .exits
            .get(&Direction::N)
            .expect("direction N should exist"),
        room2_id
    )
}
