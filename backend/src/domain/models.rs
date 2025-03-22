#[derive(Clone)]
pub struct UserEntity {
    pub tg_id: i32,
    pub username: String,
    pub first_name: String,
    pub second_name: String,
}

pub struct ActiveEntity {
    pub user_id: u32,
    pub security_id: u32,
    pub bought_price: u32,
    pub count: u32,
}

pub struct NotificationEntity {
    pub portfolio_id: u32,
    pub active_id: u32,
    pub limit_upper: u32,
    pub limit_lower: u32,
    pub limit_type: u8,
}
