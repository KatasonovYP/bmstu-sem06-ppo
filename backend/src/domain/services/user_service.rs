use crate::domain::errors::DomainError;
use crate::domain::models::UserEntity;
use crate::ports::inbound::domain::AbstractUserService;
use crate::ports::outbound::db::UserRepository;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl AbstractUserService for UserService {
    async fn get_user(&self, user_id: u32) -> Result<UserEntity, DomainError> {
        self.user_repository.get_user(user_id).await
    }

    async fn create_user(&self, user: UserEntity) -> Result<UserEntity, DomainError> {
        self.user_repository.create_user(user).await
    }
}
