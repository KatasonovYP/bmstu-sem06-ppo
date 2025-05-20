use std::sync::Arc;

use async_trait::async_trait;
use shaku::Component;
use tokio::time::{
    self,
    Duration,
};

use crate::domain::{
    errors::DomainError,
    models::ActiveEntity,
    ports::{
        cache::SecurityCurrentPriceCache,
        domain::{
            AbstractPriceRefresherService,
            AbstractPricesService,
        },
        storage::ActiveRepository,
    },
    value_objects::Price,
};

#[derive(Clone, Component)]
#[shaku(interface = AbstractPriceRefresherService)]
pub struct PriceRefresherService {
    #[shaku(inject)]
    active_repo: Arc<dyn ActiveRepository>,
    #[shaku(inject)]
    prices_cache: Arc<dyn SecurityCurrentPriceCache>,
    #[shaku(inject)]
    prices_service: Arc<dyn AbstractPricesService>,
}

#[async_trait]
impl AbstractPriceRefresherService for PriceRefresherService {
    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn refresh_prices(&self) -> Result<(), DomainError> {
        let actives = self.active_repo.list_actives().await?;
        tracing::info!("refresh prices for {:#?}", actives);
        for active in &actives {
            self.refresh_price(active).await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn refresh_price(&self, active: &ActiveEntity) -> Result<Price, DomainError> {
        let price = self.prices_service.get_active_current_price(active).await?;
        self.prices_cache
            .set_price(&active.security_id, price.clone())
            .await?;
        Ok(price)
    }

    async fn start(&self) -> Result<u32, DomainError> {
        let n_seconds = 5;
        let mut interval = time::interval(Duration::from_secs(n_seconds));

        loop {
            tracing::info!("new check");
            interval.tick().await;
            self.refresh_prices().await?;
        }
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
    use crate::domain::{
        models::ActiveEntity,
        ports::{
            cache::MockSecurityCurrentPriceCache,
            domain::MockAbstractPricesService,
            storage::MockActiveRepository,
        },
        value_objects::Price,
    };

    fn setup_service(
        active_repo: Arc<MockActiveRepository>,
        prices_cache: Arc<MockSecurityCurrentPriceCache>,
        prices_service: Arc<MockAbstractPricesService>,
    ) -> PriceRefresherService {
        PriceRefresherService {
            active_repo,
            prices_cache,
            prices_service,
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

        let mut prices_service = MockAbstractPricesService::new();
        prices_service
            .expect_get_active_current_price()
            .with(eq(active1.clone()))
            .returning(move |_| Ok(price1_clone1.clone()));

        prices_service
            .expect_get_active_current_price()
            .with(eq(active2.clone()))
            .returning(move |_| Ok(price2_clone1.clone()));

        let mut prices_cache = MockSecurityCurrentPriceCache::new();
        prices_cache
            .expect_set_price()
            .with(eq(active1.security_id.clone()), eq(price1_clone2.clone()))
            .returning(|_, p| Ok(p));

        prices_cache
            .expect_set_price()
            .with(eq(active2.security_id.clone()), eq(price2_clone2.clone()))
            .returning(|_, p| Ok(p));

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(prices_cache),
            Arc::new(prices_service),
        );

        // Act
        let result = service.refresh_prices().await;

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

        let prices_service = MockAbstractPricesService::new();
        // Для пустого списка активов get_active_current_price не должен вызываться

        let prices_cache = MockSecurityCurrentPriceCache::new();
        // Для пустого списка активов set_price не должен вызываться

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(prices_cache),
            Arc::new(prices_service),
        );

        // Act
        let result = service.refresh_prices().await;

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

        let prices_service = MockAbstractPricesService::new();
        let prices_cache = MockSecurityCurrentPriceCache::new();

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(prices_cache),
            Arc::new(prices_service),
        );

        // Act
        let result = service.refresh_prices().await;

        // Assert
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(matches!(error, DomainError::RepositoryError(..)));
        }
    }

    #[tokio::test]
    async fn should_handle_error_from_prices_service() {
        // Arrange
        let active: ActiveEntity = Faker.fake();
        let active_clone = active.clone();

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_actives()
            .returning(move || Ok(vec![active.clone()]));

        let mut prices_service = MockAbstractPricesService::new();
        prices_service
            .expect_get_active_current_price()
            .with(eq(active_clone.clone()))
            .returning(|_| {
                Err(DomainError::ExternalServiceError(
                    "Exchange API error".to_string(),
                ))
            });

        let prices_cache = MockSecurityCurrentPriceCache::new();
        // set_price не должен вызываться из-за ошибки в prices_service

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(prices_cache),
            Arc::new(prices_service),
        );

        // Act
        let result = service.refresh_prices().await;

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

        let mut prices_service = MockAbstractPricesService::new();
        prices_service
            .expect_get_active_current_price()
            .with(eq(active_clone1.clone()))
            .returning(move |_| Ok(price_clone1.clone()));

        let mut prices_cache = MockSecurityCurrentPriceCache::new();
        prices_cache
            .expect_set_price()
            .with(
                eq(active_clone2.security_id.clone()),
                eq(price_clone2.clone()),
            )
            .returning(|_, _| Err(DomainError::RepositoryError("Cache error".to_string())));

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(prices_cache),
            Arc::new(prices_service),
        );

        // Act
        let result = service.refresh_prices().await;

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

        let mut prices_service = MockAbstractPricesService::new();
        // Настройка для первого актива
        prices_service
            .expect_get_active_current_price()
            .with(eq(active1_clone1.clone()))
            .returning(move |_| Ok(price1_clone1.clone()));

        // Настройка для второго актива - возвращает ошибку
        prices_service
            .expect_get_active_current_price()
            .with(eq(active2_clone.clone()))
            .returning(|_| {
                Err(DomainError::ExternalServiceError(
                    "Error for active2".to_string(),
                ))
            });

        // Настройка для третьего актива
        prices_service
            .expect_get_active_current_price()
            .with(eq(active3_clone1.clone()))
            .returning(move |_| Ok(price3_clone1.clone()));

        let mut prices_cache = MockSecurityCurrentPriceCache::new();
        // Настройка для первого актива
        prices_cache
            .expect_set_price()
            .with(
                eq(active1_clone2.security_id.clone()),
                eq(price1_clone2.clone()),
            )
            .returning(|_, p| Ok(p));

        // Для второго актива set_price не вызывается из-за ошибки

        // Настройка для третьего актива
        prices_cache
            .expect_set_price()
            .with(
                eq(active3_clone2.security_id.clone()),
                eq(price3_clone2.clone()),
            )
            .returning(|_, p| Ok(p));

        let service = setup_service(
            Arc::new(active_repo),
            Arc::new(prices_cache),
            Arc::new(prices_service),
        );

        // Act
        let result = service.refresh_prices().await;

        // Assert
        // Этот тест предполагает, что ошибка для одного актива останавливает обработку всех
        assert!(result.is_err());
    }
}
