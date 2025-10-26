use std::sync::Arc;

use shaku::Component;

use crate::{
    errors::DomainError,
    models::UserEntity,
    ports::{
        domain::AbstractUserService,
        storage::UserRepository,
    },
};

#[derive(Clone, Component)]
#[shaku(interface = AbstractUserService)]
pub struct UserService {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl AbstractUserService for UserService {
    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn get_user(&self, user_id: u32) -> Result<UserEntity, DomainError> {
        self.user_repository.get_user(user_id).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn get_user_by_tg_id(&self, tg_id: i64) -> Result<UserEntity, DomainError> {
        self.user_repository.get_user_by_tg_id(tg_id).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn create_user(&self, user: &UserEntity) -> Result<UserEntity, DomainError> {
        self.user_repository.create_user(user).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn list_users(&self) -> Result<Vec<UserEntity>, DomainError> {
        self.user_repository.list_users().await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn update_user(&self, user: &UserEntity) -> Result<UserEntity, DomainError> {
        self.user_repository.update_user(user).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn delete_user(&self, user_id: u32) -> Result<UserEntity, DomainError> {
        self.user_repository.delete_user(user_id).await
    }
}

#[cfg(test)]
mod tests {
    use fake::{
        Fake,
        Faker,
    };
    use mockall::predicate::eq;

    use super::*;
    use crate::ports::storage::MockUserRepository;

    #[tokio::test]
    async fn test_create_user() {
        let input_user: UserEntity = Faker.fake();

        let expected_user: UserEntity = Faker.fake();

        let mock_user = expected_user.clone();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_create_user()
            .with(eq(input_user.clone()))
            .returning(move |_| Ok(mock_user.clone()));
        let user_service = UserService::new(Arc::new(user_repository));

        let result = user_service.create_user(&input_user).await.unwrap();

        assert_eq!(result, expected_user);
    }

    #[tokio::test]
    async fn test_get_user() {
        let mock_user: UserEntity = Faker.fake();
        let expect_user = mock_user.clone();

        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_get_user()
            .with(eq(0))
            .returning(move |_| Ok(mock_user.clone()));
        let user_service = UserService::new(Arc::new(user_repository));

        let result_user = user_service.get_user(0).await.unwrap();

        assert_eq!(result_user, expect_user);
    }
}
