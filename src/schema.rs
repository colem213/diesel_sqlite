// @generated automatically by Diesel CLI.

diesel::table! {
    queue (id) {
        id -> Text,
        created_at -> Text,
        updated_at -> Text,
        scheduled_for -> Text,
        failed_attempts -> Integer,
        status -> Integer,
        message -> Text,
    }
}
