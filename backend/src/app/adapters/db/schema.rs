use diesel::prelude::*;

table! {
    users (user_id) {
        user_id -> Int4,
        tg_id -> Int4,
        username -> Varchar,
        first_name -> Varchar,
        second_name -> Varchar,
        created_at -> Timestamp,
    }
}
