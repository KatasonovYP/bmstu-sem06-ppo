use crate::value_objects::Price;

#[cfg_attr(not(feature = "production"), derive(fake::Dummy))]
#[derive(Debug, Clone, Default)]
pub struct NotificationEntity {
    pub notification_id: u32,
    pub portfolio_id: u32,
    pub active_id: u32,
    pub limit_upper: Price,
    pub limit_lower: Price,
}

impl PartialEq for NotificationEntity {
    fn eq(&self, other: &Self) -> bool {
        self.notification_id == other.notification_id
    }
}
