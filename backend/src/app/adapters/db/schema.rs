// @generated automatically by Diesel CLI.

diesel::table! {
    actives (active_id) {
        active_id -> Int4,
        user_id -> Int4,
        security_id -> Int4,
        bought_price -> Int4,
        count -> Int4,
    }
}

diesel::table! {
    notifications (notification_id) {
        notification_id -> Int4,
        portfolio_id -> Int4,
        active_id -> Int4,
        limit_upper -> Int4,
        limit_lower -> Int4,
        limit_type -> Int2,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        tg_id -> Int4,
        username -> Varchar,
        first_name -> Nullable<Varchar>,
        second_name -> Nullable<Varchar>,
    }
}

diesel::joinable!(notifications -> actives (active_id));
diesel::joinable!(notifications -> users (portfolio_id));

diesel::allow_tables_to_appear_in_same_query!(
    actives,
    notifications,
    users,
);
