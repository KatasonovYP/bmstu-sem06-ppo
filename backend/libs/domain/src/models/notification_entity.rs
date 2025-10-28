use crate::value_objects::Price;

#[derive(Debug, Clone, Default, fake::Dummy)]
pub struct NotificationEntity {
    pub notification_id: u32,
    pub portfolio_id: u32,
    pub active_id: u32,
    pub limit_upper: Price,
    pub limit_lower: Price,
    pub resend_interval_sec: u32,
}

impl PartialEq for NotificationEntity {
    fn eq(&self, other: &Self) -> bool {
        self.notification_id == other.notification_id
    }
}
