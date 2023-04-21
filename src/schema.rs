// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
        avatar -> Text,
        created_at -> Timestamptz,
    }
}
