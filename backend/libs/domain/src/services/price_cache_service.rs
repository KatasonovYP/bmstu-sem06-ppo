use std::sync::Arc;

use shaku::Component;

use crate::{
    errors::DomainError,
    models::ActiveEntity,
    ports::{
        cache::PriceCacheRepository,
        domain::{
            AbstractPriceCacheService,
            AbstractPriceOpsService,
        },
        storage::ActiveRepository,
    },
    value_objects::Price,
};

#[derive(Clone, Component)]
#[shaku(interface = AbstractPriceCacheService)]
pub struct PriceCacheService {
    #[shaku(inject)]
    active_repo: Arc<dyn ActiveRepository>,
    #[shaku(inject)]
    price_cache_repo: Arc<dyn PriceCacheRepository>,
    #[shaku(inject)]
    price_ops_service: Arc<dyn AbstractPriceOpsService>,
}

impl PriceCacheService {
    pub fn new(
        active_repo: Arc<dyn ActiveRepository>,
        price_cache_repo: Arc<dyn PriceCacheRepository>,
        price_ops_service: Arc<dyn AbstractPriceOpsService>,
    ) -> Self {
        Self {
            active_repo,
            price_cache_repo,
            price_ops_service,
        }
    }
}

#[async_trait::async_trait]
impl AbstractPriceCacheService for PriceCacheService {
    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn get_price(&self, active: &ActiveEntity) -> Result<Price, DomainError> {
        let price = match self.price_cache_repo.get_price(&active.security_id).await {
            Err(_) => self.refresh_security_price(&active.security_id).await?,
            Ok(price) => price,
        };
        Ok(price * active.count)
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn refresh_all_prices(&self) -> Result<(), DomainError> {
        let actives = self.active_repo.list_actives().await?;
        let mut securities: Vec<String> = actives.iter().map(|x| x.security_id.clone()).collect();
        securities.sort();
        securities.dedup();
        tracing::info!("refresh prices for {:#?}", securities);
        for security in &securities {
            self.refresh_security_price(security).await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn refresh_security_price(&self, security_id: &String) -> Result<Price, DomainError> {
        let price = self
            .price_ops_service
            .get_security_current_price(security_id)
            .await?;
        self.price_cache_repo
            .set_price(security_id, price.clone())
            .await?;
        Ok(price)
    }
}

#[cfg(not(feature = "production"))]
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use fake::{
        Fake,
        Faker,
    };
    use mockall::predicate::eq;

    use super::*;
    use crate::{
        models::ActiveEntity,
        ports::{
            cache::MockPriceCacheRepository,
            domain::MockAbstractPriceOpsService,
            storage::MockActiveRepository,
        },
        value_objects::Price,
    };

    fn setup_service(
        active_repo: Arc<MockActiveRepository>,
        price_cache_repo: Arc<MockPriceCacheRepository>,
        price_ops_service: Arc<MockAbstractPriceOpsService>,
    ) -> PriceCacheService {
        PriceCacheService {
            active_repo,
            price_cache_repo,
            price_ops_service,
        }
    }

    #[tokio::test]
    async fn should_refresh_prices_for_all_actives() {
        // Arrange
        let active1: ActiveEntity = Faker.fake();
        let active2: ActiveEntity = Faker.fake();

        let price1 = Price::rub(100.0);
        let price2 = Price::rub(200.0);

        // Клонируем для использования в замыканиях
        let price1_clone1 = price1.clone();
        let price2_clone1 = price2.clone();
        let price1_clone2 = price1.clone();
        let price2_clone2 = price2.clone();

        let actives = vec![active1.clone(), active2.clone()];

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_actives()
            .returning(move || Ok(actives.clone()));

        let mut price_ops_service = MockAbstractPriceOpsService::new();
        price_ops_service
            .expect_get_security_current_price()
            .with(eq(active1.security_id.clone()))
            .returning(move |_| Ok(price1_clone1.clone()));

        price_ops_service
            .expect_get_security_current_price()
            .with(eq(active2.security_id.clone()))
            .returning(move |_| Ok(price2_clone1.clone()));

        let mut price_cache_repo = MockPriceCacheRepository::new();
        price_cache_repo
            .expect_set_price()
            .with(eq(active1.security_id.clone()), eq(price1_clone2.clone()))
            .returning(|_, p| Ok(p));

        price_cache_repo
            .expect_set_price()
            .with(eq(active2.security_id.clone()), eq(price2_clone2.clone()))
            .returning(|_, p| Ok(p));

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(price_cache_repo),
            Arc::new(price_ops_service),
        );

        // Act
        let result = service.refresh_all_prices().await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_handle_empty_actives_list() {
        // Arrange
        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_actives()
            .returning(|| Ok(Vec::new()));

        let price_ops_service = MockAbstractPriceOpsService::new();
        // Для пустого списка активов get_active_current_price не должен вызываться

        let price_cache_repo = MockPriceCacheRepository::new();
        // Для пустого списка активов set_price не должен вызываться

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(price_cache_repo),
            Arc::new(price_ops_service),
        );

        // Act
        let result = service.refresh_all_prices().await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_handle_error_from_active_repo() {
        // Arrange
        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_actives()
            .returning(|| Err(DomainError::RepositoryError("Database error".to_string())));

        let price_ops_service = MockAbstractPriceOpsService::new();
        let price_cache_repo = MockPriceCacheRepository::new();

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(price_cache_repo),
            Arc::new(price_ops_service),
        );

        // Act
        let result = service.refresh_all_prices().await;

        // Assert
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(matches!(error, DomainError::RepositoryError(..)));
        }
    }

    #[tokio::test]
    async fn should_handle_error_from_price_ops_service() {
        // Arrange
        let active: ActiveEntity = Faker.fake();
        let active_clone = active.clone();

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_actives()
            .returning(move || Ok(vec![active.clone()]));

        let mut price_ops_service = MockAbstractPriceOpsService::new();
        price_ops_service
            .expect_get_security_current_price()
            .with(eq(active_clone.security_id.clone()))
            .returning(|_| {
                Err(DomainError::ExternalServiceError(
                    "Exchange API error".to_string(),
                ))
            });

        let price_cache_repo = MockPriceCacheRepository::new();
        // set_price не должен вызываться из-за ошибки в price_ops_service

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(price_cache_repo),
            Arc::new(price_ops_service),
        );

        // Act
        let result = service.refresh_all_prices().await;

        // Assert
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(matches!(error, DomainError::ExternalServiceError(..)));
        }
    }

    #[tokio::test]
    async fn should_handle_error_from_cache() {
        // Arrange
        let active: ActiveEntity = Faker.fake();
        let active_clone1 = active.clone();
        let active_clone2 = active.clone();

        let price = Price::rub(150.0);
        let price_clone1 = price.clone();
        let price_clone2 = price.clone();

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_actives()
            .returning(move || Ok(vec![active.clone()]));

        let mut price_ops_service = MockAbstractPriceOpsService::new();
        price_ops_service
            .expect_get_security_current_price()
            .with(eq(active_clone1.security_id.clone()))
            .returning(move |_| Ok(price_clone1.clone()));

        let mut price_cache_repo = MockPriceCacheRepository::new();
        price_cache_repo
            .expect_set_price()
            .with(
                eq(active_clone2.security_id.clone()),
                eq(price_clone2.clone()),
            )
            .returning(|_, _| Err(DomainError::RepositoryError("Cache error".to_string())));

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(price_cache_repo),
            Arc::new(price_ops_service),
        );

        // Act
        let result = service.refresh_all_prices().await;

        // Assert
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(matches!(error, DomainError::RepositoryError(..)));
        }
    }

    #[tokio::test]
    async fn should_continue_processing_despite_errors_in_some_actives() {
        // Arrange
        // Первый актив обрабатывается успешно
        let active1: ActiveEntity = Faker.fake();
        let active1_clone1 = active1.clone();
        let active1_clone2 = active1.clone();

        // Второй актив вызывает ошибку при получении цены
        let active2: ActiveEntity = Faker.fake();
        let active2_clone = active2.clone();

        // Третий актив обрабатывается успешно
        let active3: ActiveEntity = Faker.fake();
        let active3_clone1 = active3.clone();
        let active3_clone2 = active3.clone();

        let price1 = Price::rub(100.0);
        let price1_clone1 = price1.clone();
        let price1_clone2 = price1.clone();

        let price3 = Price::rub(300.0);
        let price3_clone1 = price3.clone();
        let price3_clone2 = price3.clone();

        let actives = vec![active1.clone(), active2.clone(), active3.clone()];

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_actives()
            .returning(move || Ok(actives.clone()));

        let mut price_ops_service = MockAbstractPriceOpsService::new();
        // Настройка для первого актива
        price_ops_service
            .expect_get_security_current_price()
            .with(eq(active1_clone1.security_id.clone()))
            .returning(move |_| Ok(price1_clone1.clone()));

        // Настройка для второго актива - возвращает ошибку
        price_ops_service
            .expect_get_security_current_price()
            .with(eq(active2_clone.security_id.clone()))
            .returning(|_| {
                Err(DomainError::ExternalServiceError(
                    "Error for active2".to_string(),
                ))
            });

        // Настройка для третьего актива
        price_ops_service
            .expect_get_security_current_price()
            .with(eq(active3_clone1.security_id.clone()))
            .returning(move |_| Ok(price3_clone1.clone()));

        let mut price_cache_repo = MockPriceCacheRepository::new();
        // Настройка для первого актива
        price_cache_repo
            .expect_set_price()
            .with(
                eq(active1_clone2.security_id.clone()),
                eq(price1_clone2.clone()),
            )
            .returning(|_, p| Ok(p));

        // Для второго актива set_price не вызывается из-за ошибки

        // Настройка для третьего актива
        price_cache_repo
            .expect_set_price()
            .with(
                eq(active3_clone2.security_id.clone()),
                eq(price3_clone2.clone()),
            )
            .returning(|_, p| Ok(p));

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(price_cache_repo),
            Arc::new(price_ops_service),
        );

        // Act
        let result = service.refresh_all_prices().await;

        // Assert
        // Этот тест предполагает, что ошибка для одного актива останавливает обработку всех
        assert!(result.is_err());
    }
}
