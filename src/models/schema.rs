// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        mail -> Text,
        password -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    vote_validators (rowid) {
        rowid -> Integer,
        vote_id -> Integer,
        val_endpoint -> Text,
    }
}

diesel::table! {
    votes (vot_id) {
        vot_id -> Integer,
        name -> Text,
        owner_id -> Integer,
        is_finished -> Bool,
    }
}

diesel::joinable!(vote_validators -> votes (vote_id));
diesel::joinable!(votes -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    vote_validators,
    votes,
);
