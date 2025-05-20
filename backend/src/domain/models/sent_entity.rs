use chrono::NaiveDateTime;

#[cfg_attr(not(feature = "production"), derive(fake::Dummy))]
#[derive(Debug, Clone, Default)]
pub struct SentEntity {
    pub notification_id: u32,
    pub last_message_time: NaiveDateTime,
}

impl PartialEq for SentEntity {
    fn eq(&self, other: &Self) -> bool {
        self.notification_id == other.notification_id
    }
}
