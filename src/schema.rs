// @generated automatically by Diesel CLI.

diesel::table! {
    queue (id) {
        id -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        scheduled_for -> TimestamptzSqlite,
        failed_attempts -> Integer,
        status -> Integer,
        message -> Text,
    }
}
