use crate::domain::value_objects::Price;

#[cfg_attr(not(feature = "production"), derive(fake::Dummy))]
#[derive(Debug, Clone, Default)]
pub struct ActiveEntity {
    pub active_id: u32,
    pub user_id: u32,
    pub security_id: String,
    pub bought_price: Price,
    pub count: u32,
}

impl PartialEq for ActiveEntity {
    fn eq(&self, other: &Self) -> bool {
        self.active_id == other.active_id
    }
}
