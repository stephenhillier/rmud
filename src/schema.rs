// @generated automatically by Diesel CLI.

diesel::table! {
    character (name) {
        name -> Text,
        room -> Int4,
    }
}

diesel::table! {
    room (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
    }
}

diesel::joinable!(character -> room (room));

diesel::allow_tables_to_appear_in_same_query!(
    character,
    room,
);
