-- Your SQL goes here


CREATE TABLE rooms (
  id serial primary key,
  title text not null,
  description text not null
);

CREATE TABLE exits (
  from_room integer not null references rooms(id),
  dir text not null check (dir in ('N', 'E', 'S', 'W', 'U', 'D')),
  to_room integer not null references rooms(id),
  PRIMARY KEY(from_room, dir)
);

CREATE TABLE players (
  name text primary key
);

CREATE TABLE characters (
  name text primary key,
  player text not null references players(name),
  room integer not null references rooms(id)
);

