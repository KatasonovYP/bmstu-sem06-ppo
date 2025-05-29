use std::sync::Arc;

use domain::{
    models::SentEntity,
    ports::domain::AbstractSentService,
};

use crate::commands::SentCommands;

pub struct CliSentController {
    sent_service: Arc<dyn AbstractSentService>,
}

impl CliSentController {
    pub fn new(sent_service: Arc<dyn AbstractSentService>) -> Self {
        Self { sent_service }
    }

    pub async fn get_sent(&self, notification_id: &u32) {
        self.sent_service.get_sent(*notification_id).await.unwrap();
    }

    pub async fn create_sent(&self, command: &SentCommands) {
        let entity = SentEntity::try_from(command).unwrap();
        self.sent_service.create_sent(entity).await.unwrap();
    }

    pub async fn update_sent(&self, command: &SentCommands) {
        let entity = SentEntity::try_from(command).unwrap();
        self.sent_service.update_sent(entity).await.unwrap();
    }

    pub async fn list_sent(&self) {
        self.sent_service.list_sent().await.unwrap();
    }

    pub async fn delete_sent(&self, notification_id: &u32) {
        self.sent_service
            .delete_sent(*notification_id)
            .await
            .unwrap();
    }
}
