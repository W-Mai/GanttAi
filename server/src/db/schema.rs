// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Uuid,
        title -> Varchar,
        content -> Text,
        start_time -> Timestamptz,
        end_time -> Nullable<Timestamptz>,
        parent_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}