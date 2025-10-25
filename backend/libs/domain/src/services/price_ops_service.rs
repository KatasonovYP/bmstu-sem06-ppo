use std::sync::Arc;

use shaku::Component;

use crate::{
    errors::DomainError,
    models::ActiveEntity,
    ports::{
        domain::AbstractPriceOpsService,
        exchange::ExchangeRepository,
    },
    value_objects::{
        Currency,
        Price,
    },
};

#[derive(Clone, Component)]
#[shaku(interface = AbstractPriceOpsService)]
pub struct PriceOpsService {
    #[shaku(inject)]
    exchange_repository: Arc<dyn ExchangeRepository>,
}

impl PriceOpsService {
    pub fn new(exchange_repository: Arc<dyn ExchangeRepository>) -> Self {
        Self {
            exchange_repository,
        }
    }
}

#[async_trait::async_trait]
impl AbstractPriceOpsService for PriceOpsService {
    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn get_actives_bought_price(
        &self,
        actives: Vec<ActiveEntity>,
    ) -> Result<Price, DomainError> {
        let mut total = Price::zero(Currency::rub());

        for active in &actives {
            total = (total + active.clone().bought_price * active.count)?;
        }

        Ok(total)
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn get_actives_current_price(
        &self,
        actives: Vec<ActiveEntity>,
    ) -> Result<Price, DomainError> {
        let mut total = Price::zero(Currency::rub());

        for active in &actives {
            let current_price = self.get_active_current_price(active).await?;
            total = (total + current_price)?;
        }

        Ok(total)
    }

    async fn get_security_current_price(&self, security_id: &str) -> Result<Price, DomainError> {
        let trades = self.exchange_repository.get_trades(security_id).await?;

        let security_current_price = trades
            .first()
            .ok_or(DomainError::EntityNotFound {
                entity: "security".into(),
                id: security_id.to_string(),
            })?
            .price;
        Ok(Price::rub(security_current_price))
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn get_active_current_price(&self, active: &ActiveEntity) -> Result<Price, DomainError> {
        let security_current_price = self.get_security_current_price(&active.security_id).await?;
        let active_current_price = security_current_price * active.count;
        Ok(active_current_price)
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn get_price_delta(&self, active: &ActiveEntity) -> Result<Price, DomainError> {
        let current_price = self.get_active_current_price(active).await?;
        current_price - active.bought_price.clone()
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
    use crate::{
        models::TradeEntity,
        ports::exchange::MockExchangeRepository,
        value_objects::Price,
    };

    #[tokio::test]
    async fn should_return_bought_price_zero_when_no_actives() {
        let expected_sum = Price::rub(0.);

        let mock_exchange_repository = MockExchangeRepository::new();

        let result = PriceOpsService::new(Arc::new(mock_exchange_repository))
            .get_actives_bought_price(vec![])
            .await
            .unwrap();

        assert_eq!(result, expected_sum);
    }

    #[tokio::test]
    async fn should_return_current_price_zero_when_no_actives() {
        let expected_price = Price::rub(0.);

        let mock_exchange_repository = MockExchangeRepository::new();

        let result = PriceOpsService::new(Arc::new(mock_exchange_repository))
            .get_actives_current_price(vec![])
            .await
            .unwrap();

        assert_eq!(result, expected_price);
    }

    #[tokio::test]
    async fn should_sum_bought_price_actives() {
        let mut actives = fake::vec![ActiveEntity; 2];

        actives[0].bought_price = Price::rub(100.);
        actives[0].count = 2;

        actives[1].bought_price = Price::rub(30.);
        actives[1].count = 3;

        let expected_sum = Price::rub(100. * 2. + 30. * 3.);

        let mock_exchange_repository = MockExchangeRepository::new();

        let result = PriceOpsService::new(Arc::new(mock_exchange_repository))
            .get_actives_bought_price(actives)
            .await
            .unwrap();

        assert_eq!(result, expected_sum);
    }

    #[tokio::test]
    async fn should_calculate_current_price_for_active() {
        let mut mock_active: ActiveEntity = Faker.fake();
        mock_active.count = 10;

        let mut mock_trade: TradeEntity = Faker.fake();
        mock_trade.price = 160.;

        let expected_price = Price::rub(10. * 160.);

        let mut mock_exchange_repository = MockExchangeRepository::new();

        mock_exchange_repository
            .expect_get_trades()
            .with(eq(mock_active.security_id.clone()))
            .returning(move |_| Ok(vec![mock_trade.clone()]));

        let active_service = PriceOpsService::new(Arc::new(mock_exchange_repository));

        let result = active_service
            .get_active_current_price(&mock_active)
            .await
            .unwrap();

        assert_eq!(result, expected_price);
    }

    #[tokio::test]
    async fn should_error_when_no_trades_available() {
        let mock_active: ActiveEntity = Faker.fake();

        let mut mock_exchange_repository = MockExchangeRepository::new();

        mock_exchange_repository
            .expect_get_trades()
            .with(eq(mock_active.security_id.clone()))
            .returning(|_| Ok(vec![])); // Empty trades list

        let active_service = PriceOpsService::new(Arc::new(mock_exchange_repository));

        let result = active_service.get_active_current_price(&mock_active).await;

        assert!(result.is_err());
        if let Err(error) = result {
            assert!(matches!(error, DomainError::EntityNotFound { .. }));
        }
    }

    #[tokio::test]
    async fn should_sum_current_price_actives() {
        let mut apple_active: ActiveEntity = Faker.fake();
        apple_active.security_id = "AAPL".into();
        apple_active.count = 10;

        let mut msft_active: ActiveEntity = Faker.fake();
        msft_active.security_id = "MSFT".into();
        msft_active.count = 5;

        let actives = vec![apple_active.clone(), msft_active.clone()];

        let mut apple_trade: TradeEntity = Faker.fake();
        apple_trade.price = 160.;

        let mut msft_trade: TradeEntity = Faker.fake();
        msft_trade.price = 270.;

        let expected_price = Price::rub(160. * 10. + 270. * 5.);

        let mut mock_exchange_repository = MockExchangeRepository::new();

        mock_exchange_repository
            .expect_get_trades()
            .with(eq(apple_active.security_id))
            .returning(move |_| Ok(vec![apple_trade.clone()]));

        mock_exchange_repository
            .expect_get_trades()
            .with(eq(msft_active.security_id))
            .returning(move |_| Ok(vec![msft_trade.clone()]));

        let price_ops_service = PriceOpsService::new(Arc::new(mock_exchange_repository));

        let result = price_ops_service
            .get_actives_current_price(actives)
            .await
            .unwrap();

        assert_eq!(result, expected_price);
    }

    #[tokio::test]
    async fn should_call_get_trades_for_each_active_london_style() {
        // arrange
        use mockall::{
            Sequence,
            predicate::eq,
        };

        use crate::ports::exchange::MockExchangeRepository;

        // Два актива с разными security_id
        let apple_active = ActiveEntity {
            security_id: "AAPL".to_string(),
            count: 1,
            bought_price: Price::rub(0.),
            ..Faker.fake()
        };
        let msft_active = ActiveEntity {
            security_id: "MSFT".to_string(),
            count: 1,
            bought_price: Price::rub(0.),
            ..Faker.fake()
        };

        let apple_trade = TradeEntity {
            price: 100.,
            ..Faker.fake()
        };
        let msft_trade = TradeEntity {
            price: 200.,
            ..Faker.fake()
        };

        let mut seq = Sequence::new();

        let mut mock_repo = MockExchangeRepository::new();

        mock_repo
            .expect_get_trades()
            .with(eq("AAPL".to_string()))
            .times(1)
            .in_sequence(&mut seq)
            .return_once(move |_| Ok(vec![apple_trade.clone()]));

        mock_repo
            .expect_get_trades()
            .with(eq("MSFT".to_string()))
            .times(1)
            .in_sequence(&mut seq)
            .return_once(move |_| Ok(vec![msft_trade.clone()]));

        let service = PriceOpsService::new(Arc::new(mock_repo));

        // act
        let _ = service
            .get_actives_current_price(vec![apple_active.clone(), msft_active.clone()])
            .await
            .unwrap();
    }
}
