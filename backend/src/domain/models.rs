use super::value_objects;

#[derive(Clone)]
pub struct UserEntity {
    pub tg_id: i32,
    pub username: value_objects::Username,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
}

#[derive(Clone)]
pub struct ActiveEntity {
    pub user_id: i32,
    pub security_id: i32,
    pub bought_price: i32,
    pub count: i32,
}

#[derive(Clone)]
pub struct NotificationEntity {
    pub portfolio_id: i32,
    pub active_id: i32,
    pub limit_upper: i32,
    pub limit_lower: i32,
    pub limit_type: i16,
}

// #[derive(Clone)]
// pub struct SecurityEntity {
//     pub name: String,
//     pub title: String,
//     pub value: String,
//     pub property_type: String,
//     pub sort_order: i64,
//     pub is_hidden: i64,
//     pub precision: Option<i64>,
// }