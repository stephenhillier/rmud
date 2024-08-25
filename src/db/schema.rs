// @generated automatically by Diesel CLI.

diesel::table! {
    characters (name) {
        name -> Text,
        player -> Text,
        room -> Int4,
    }
}

diesel::table! {
    exits (from_room, dir) {
        from_room -> Int4,
        dir -> Text,
        to_room -> Int4,
    }
}

diesel::table! {
    players (name) {
        name -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
    }
}

diesel::joinable!(characters -> players (player));
diesel::joinable!(characters -> rooms (room));

diesel::allow_tables_to_appear_in_same_query!(
    characters,
    exits,
    players,
    rooms,
);
