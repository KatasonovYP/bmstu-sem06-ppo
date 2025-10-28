use crate::value_objects::Price;

#[derive(Debug, Clone, Default, fake::Dummy)]
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
