use crate::db::schema::{characters, players, rooms, exits};
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Player {
    pub name: String,
}

#[derive(Queryable, Debug)]
pub struct Character {
    pub name: String,
    pub player: String,
    pub room: i32,
}

#[derive(Queryable, Debug)]
pub struct Room {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Queryable)]
pub struct Exit {
    pub from_room: i32,
    pub dir: String,
    pub to_room: i32,
}
#[derive(Insertable)]
#[diesel(table_name = exits)]
pub struct NewExit<'a> {
    pub from_room: i32,
    pub dir: &'a str,
    pub to_room: i32,
}

#[derive(Insertable)]
#[diesel(table_name = rooms)]
pub struct NewRoom<'a> {
    pub title: &'a str,
    pub description: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = players)]
pub struct NewPlayer<'a> {
    pub name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = characters)]
pub struct NewCharacter<'a> {
    pub name: &'a str,
    pub player: &'a str,
    pub room: &'a i32,
}
