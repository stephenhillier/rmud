-- Your SQL goes here

CREATE TABLE room (
  id serial primary key,
  title text not null,
  description text not null
);

CREATE TABLE character (
  name text primary key,
  room integer not null references room
);
