use crate::domain::errors::DomainError;
use crate::domain::models::ActiveEntity;
use crate::ports::outbound::db::ActiveRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct ActiveService {
    active_repository: Arc<dyn ActiveRepository>,
}

impl ActiveService {
    pub fn new(active_repository: Arc<dyn ActiveRepository>) -> Self {
        Self { active_repository }
    }

    pub async fn get_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError> {
        self.active_repository.get_active(active_id).await
    }

    pub async fn create_active(&self, active: ActiveEntity) -> Result<ActiveEntity, DomainError> {
        self.active_repository.create_active(active).await
    }
}
