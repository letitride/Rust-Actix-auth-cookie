table! {
    directories (id) {
        id -> Int8,
        name -> Varchar,
        parent -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Int8,
        user_agent -> Varchar,
        response_time -> Int4,
        timestamp -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    directories,
    logs,
    users,
);
