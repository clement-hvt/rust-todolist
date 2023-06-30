// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
