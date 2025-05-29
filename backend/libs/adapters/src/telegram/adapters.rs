use std::sync::Arc;

use domain::{
    errors::DomainError,
    ports::sender::NotificationSender,
};
use shaku::Component;
use teloxide::prelude::{
    Bot,
    ChatId,
    Requester,
};

#[derive(Clone, Component)]
#[shaku(interface = NotificationSender)]
pub struct TelegramNotificationSender {
    bot: Arc<Bot>,
}

impl TelegramNotificationSender {
    pub fn new(bot: Arc<Bot>) -> Self {
        Self { bot }
    }
}

#[async_trait::async_trait]
impl NotificationSender for TelegramNotificationSender {
    async fn send_message(&self, chat_id: i64, message: String) -> Result<String, DomainError> {
        self.bot
            .send_message(ChatId(chat_id), message.clone())
            .await
            .unwrap();
        Ok(message)
    }
}
