use std::sync::Arc;

use async_trait::async_trait;
use shaku::Component;

use crate::domain::{
    errors::DomainError,
    models::ActiveEntity,
    ports::{
        domain::{
            AbstractActiveService,
            AbstractPriceOpsService,
        },
        storage::ActiveRepository,
    },
    value_objects::Price,
};

#[derive(Clone, Component)]
#[shaku(interface = AbstractActiveService)]
pub struct ActiveService {
    #[shaku(inject)]
    active_repository: Arc<dyn ActiveRepository>,
    #[shaku(inject)]
    price_ops_service: Arc<dyn AbstractPriceOpsService>,
}

impl ActiveService {
    pub fn new(
        active_repository: Arc<dyn ActiveRepository>,
        price_ops_service: Arc<dyn AbstractPriceOpsService>,
    ) -> Self {
        Self {
            active_repository,
            price_ops_service,
        }
    }
}

#[async_trait]
impl AbstractActiveService for ActiveService {
    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn get_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError> {
        self.active_repository.get_active(active_id).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn create_active(&self, active: ActiveEntity) -> Result<ActiveEntity, DomainError> {
        self.active_repository.create_active(active).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn list_actives(&self) -> Result<Vec<ActiveEntity>, DomainError> {
        self.active_repository.list_actives().await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn list_user_actives(&self, user_id: u32) -> Result<Vec<ActiveEntity>, DomainError> {
        self.active_repository.list_user_actives(user_id).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn update_active(&self, active: ActiveEntity) -> Result<ActiveEntity, DomainError> {
        self.active_repository.update_active(active).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn delete_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError> {
        self.active_repository.delete_active(active_id).await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn sum_user_actives_bought_price(&self, user_id: u32) -> Result<Price, DomainError> {
        let user_actives = self.active_repository.list_user_actives(user_id).await?;
        self.price_ops_service
            .get_actives_bought_price(user_actives)
            .await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn sum_user_actives_current_price(&self, user_id: u32) -> Result<Price, DomainError> {
        let user_actives = self.active_repository.list_user_actives(user_id).await?;
        self.price_ops_service
            .get_actives_current_price(user_actives)
            .await
    }
}

#[cfg(not(feature = "production"))]
#[cfg(test)]
mod tests {
    use fake::{
        Fake,
        Faker,
    };
    use mockall::predicate::eq;

    use super::*;
    use crate::domain::ports::{
        domain::MockAbstractPriceOpsService,
        storage::MockActiveRepository,
    };

    #[tokio::test]
    async fn should_return_required_active() {
        let mut mock_active_repository = MockActiveRepository::new();
        let mock_price_ops_service = MockAbstractPriceOpsService::new();

        let mock_active = Faker.fake::<ActiveEntity>();

        let expected_active = mock_active.clone();

        mock_active_repository
            .expect_get_active()
            .with(eq(0))
            .returning(move |_| Ok(mock_active.clone()));

        let active_service = ActiveService::new(
            Arc::new(mock_active_repository),
            Arc::new(mock_price_ops_service),
        );

        assert_eq!(
            active_service.get_active(0).await.unwrap(),
            expected_active.clone(),
        );
    }

    #[tokio::test]
    async fn should_create_active() {
        let mut mock_active_repository = MockActiveRepository::new();
        let mock_price_ops_service = MockAbstractPriceOpsService::new();

        let input_active: ActiveEntity = Faker.fake();

        let expected_active: ActiveEntity = Faker.fake();

        let mock_active = expected_active.clone();

        mock_active_repository
            .expect_create_active()
            .with(eq(input_active.clone()))
            .returning(move |_| Ok(mock_active.clone()));

        let active_service = ActiveService::new(
            Arc::new(mock_active_repository),
            Arc::new(mock_price_ops_service),
        );

        assert_eq!(
            active_service.create_active(input_active).await.unwrap(),
            expected_active,
        );
    }
}
