use std::sync::Arc;

use shaku::Component;

use crate::{
    errors::DomainError,
    models::SentEntity,
    ports::{
        domain::AbstractSentService,
        storage::SentRepository,
    },
};

#[derive(Clone, Component)]
#[shaku(interface = AbstractSentService)]
pub struct SentService {
    #[shaku(inject)]
    sent_repository: Arc<dyn SentRepository>,
}

impl SentService {
    pub fn new(sent_repository: Arc<dyn SentRepository>) -> Self {
        Self { sent_repository }
    }
}

#[async_trait::async_trait]
impl AbstractSentService for SentService {
    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn get_sent(&self, notification_id: u32) -> Result<SentEntity, DomainError> {
        self.sent_repository.get_sent(notification_id).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn create_sent(&self, sent: &SentEntity) -> Result<SentEntity, DomainError> {
        self.sent_repository.create_sent(sent).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn list_sent(&self) -> Result<Vec<SentEntity>, DomainError> {
        self.sent_repository.list_sent().await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn update_sent(&self, sent: &SentEntity) -> Result<SentEntity, DomainError> {
        self.sent_repository.update_sent(sent).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn delete_sent(&self, notification_id: u32) -> Result<SentEntity, DomainError> {
        self.sent_repository.delete_sent(notification_id).await
    }
}
