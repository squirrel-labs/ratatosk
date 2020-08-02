table! {
    game_types (id) {
        id -> Int4,
        name -> Varchar,
        redirect_uri -> Text,
        capacity -> Nullable<Int4>,
        color -> Bpchar,
        icon -> Bpchar,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        password -> Text,
        salt -> Text,
        game_type_id -> Int4,
    }
}

joinable!(groups -> game_types (game_type_id));

allow_tables_to_appear_in_same_query!(
    game_types,
    groups,
);
