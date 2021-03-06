table! {
    match_records (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Text>,
        finished_at -> Timestamp,
        game_id -> Integer,
        cpu_level -> Integer,
        moves -> Integer,
        result -> Integer,
    }
}

table! {
    users (user_id) {
        user_id -> Text,
        password -> Text,
    }
}

joinable!(match_records -> users (user_id));

allow_tables_to_appear_in_same_query!(
    match_records,
    users,
);
