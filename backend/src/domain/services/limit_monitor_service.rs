use std::sync::Arc;

use async_trait::async_trait;
use shaku::Component;
use tokio::time::{
    self,
    Duration,
};

use crate::domain::{
    errors::DomainError,
    models::UserEntity,
    ports::{
        cache::SecurityCurrentPriceCache,
        domain::{
            AbstractLimitMonitorService,
            AbstractPriceRefresherService,
        },
        sender::NotificationSender,
        storage::{
            ActiveRepository,
            NotificationRepository,
            SentRepository,
            UserRepository,
        },
    },
};

#[derive(Clone, Component)]
#[shaku(interface = AbstractLimitMonitorService)]
pub struct LimitMonitorService {
    #[shaku(inject)]
    user_repo: Arc<dyn UserRepository>,
    #[shaku(inject)]
    active_repo: Arc<dyn ActiveRepository>,
    #[shaku(inject)]
    notification_repo: Arc<dyn NotificationRepository>,
    #[shaku(inject)]
    sent_repo: Arc<dyn SentRepository>,
    #[shaku(inject)]
    prices_cache: Arc<dyn SecurityCurrentPriceCache>,
    #[shaku(inject)]
    price_refresher_service: Arc<dyn AbstractPriceRefresherService>,
    #[shaku(inject)]
    notification_sender: Arc<dyn NotificationSender>,
}

#[derive(Clone)]
struct Message {
    notification_id: u32,
    message: String,
}

impl LimitMonitorService {
    async fn get_user_exeeding_messages(
        &self,
        user: &UserEntity,
    ) -> Result<Vec<Message>, DomainError> {
        let mut messages: Vec<Message> = vec![];
        let actives = self.active_repo.list_user_actives(user.user_id).await?;

        for active in &actives {
            let notifications = self
                .notification_repo
                .list_active_notifications(active.active_id)
                .await?;

            for notification in notifications {
                let notification_id = notification.notification_id;
                let sent = self.sent_repo.get_sent(notification_id).await;
                if sent.is_ok() {
                    break;
                }
                let current_price = match self.prices_cache.get_price(&active.security_id).await {
                    Err(_) => self.price_refresher_service.refresh_price(active).await?,
                    Ok(price) => price,
                };
                let is_active_limit_exceeded = notification.limit_lower > current_price
                    || current_price > notification.limit_upper;
                if is_active_limit_exceeded {
                    let message = format!(
                        "Limit {} - {} is exeeded for security {} with current price: {}",
                        notification.limit_lower.amount,
                        notification.limit_upper.amount,
                        active.security_id,
                        current_price.amount,
                    );
                    messages.push(Message {
                        message,
                        notification_id,
                    });
                }
            }
        }
        Ok(messages)
    }
}

#[async_trait]
impl AbstractLimitMonitorService for LimitMonitorService {
    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn send_exeeding_messages(&self) -> Result<(), DomainError> {
        let users = self.user_repo.list_users().await?;
        for user in &users {
            let messages = self.get_user_exeeding_messages(user).await?;
            if !messages.is_empty() {
                let total_message = messages.iter().map(|x| &x.message).fold(String::new(), |acc, s| acc + s + "\n");
                self.notification_sender
                    .send_message(user.chat_id, total_message)
                    .await?;
                for message in messages {
                    self.sent_repo.create_sent_now(message.notification_id).await?;
                }
            }
        }
        Ok(())
    }

    async fn start(&self) -> Result<u32, DomainError> {
        let n_seconds = 5;
        let mut interval = time::interval(Duration::from_secs(n_seconds));

        loop {
            tracing::info!("new check");
            interval.tick().await;
            self.send_exeeding_messages().await?;
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
        models::{
            ActiveEntity,
            NotificationEntity,
            UserEntity,
        },
        ports::{
            cache::MockSecurityCurrentPriceCache,
            domain::MockAbstractPriceRefresherService,
            sender::MockNotificationSender,
            storage::{
                MockActiveRepository,
                MockNotificationRepository,
                MockSentRepository,
                MockUserRepository,
            },
        },
        value_objects::Price,
    };

    fn setup_service(
        user_repo: Arc<MockUserRepository>,
        active_repo: Arc<MockActiveRepository>,
        notification_repo: Arc<MockNotificationRepository>,
        sent_repo: Arc<MockSentRepository>,
        price_refresher_service: Arc<MockAbstractPriceRefresherService>,
        prices_cache: Arc<MockSecurityCurrentPriceCache>,
        notification_sender: Arc<MockNotificationSender>,
    ) -> LimitMonitorService {
        LimitMonitorService {
            user_repo,
            active_repo,
            notification_repo,
            price_refresher_service,
            prices_cache,
            notification_sender,
            sent_repo,
        }
    }

    #[tokio::test]
    async fn should_not_trigger_notification_when_price_within_limits() {
        // Arrange
        let user: UserEntity = Faker.fake();
        let active: ActiveEntity = Faker.fake();
        let mut notification: NotificationEntity = Faker.fake();

        // Устанавливаем лимиты как Price
        notification.limit_lower = Price::rub(90.0);
        notification.limit_upper = Price::rub(110.0);

        let current_price = Price::rub(100.0);

        let mut prices_cache = MockSecurityCurrentPriceCache::new();
        prices_cache
            .expect_get_price()
            .with(eq(active.security_id.clone()))
            .returning(move |_| Ok(current_price.clone()));

        let mut notification_repo = MockNotificationRepository::new();
        notification_repo
            .expect_list_active_notifications()
            .with(eq(active.active_id))
            .returning(move |_| Ok(vec![notification.clone()]));

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_user_actives()
            .with(eq(user.user_id))
            .returning(move |_| Ok(vec![active.clone()]));

        let mut sent_repo = MockSentRepository::new();
        sent_repo
            .expect_get_sent()
            .returning(|_| Err(DomainError::RepositoryError("".into())));

        let service = setup_service(
            Arc::new(MockUserRepository::new()),
            Arc::new(active_repo),
            Arc::new(notification_repo),
            Arc::new(sent_repo),
            Arc::new(MockAbstractPriceRefresherService::new()),
            Arc::new(prices_cache),
            Arc::new(MockNotificationSender::new()),
        );

        // Act
        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        // Assert
        assert!(triggered.is_empty());
    }

    #[tokio::test]
    async fn should_trigger_notification_when_price_below_lower_limit() {
        // Arrange
        let user: UserEntity = Faker.fake();
        let active: ActiveEntity = Faker.fake();
        let mut notification: NotificationEntity = Faker.fake();

        // Устанавливаем лимиты как Price
        notification.limit_lower = Price::rub(90.0);
        notification.limit_upper = Price::rub(110.0);

        let current_price = Price::rub(85.0); // ниже нижнего предела

        let mut prices_cache = MockSecurityCurrentPriceCache::new();
        prices_cache
            .expect_get_price()
            .with(eq(active.security_id.clone()))
            .returning(move |_| Ok(current_price.clone()));

        let mut notification_repo = MockNotificationRepository::new();
        notification_repo
            .expect_list_active_notifications()
            .with(eq(active.active_id))
            .returning(move |_| Ok(vec![notification.clone()]));

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_user_actives()
            .with(eq(user.user_id))
            .returning(move |_| Ok(vec![active.clone()]));

        let mut sent_repo = MockSentRepository::new();
        sent_repo
            .expect_get_sent()
            .returning(|_| Err(DomainError::RepositoryError("".into())));
        sent_repo
            .expect_create_sent()
            .returning(|_| Ok(Faker.fake()));

        let service = setup_service(
            Arc::new(MockUserRepository::new()),
            Arc::new(active_repo),
            Arc::new(notification_repo),
            Arc::new(sent_repo),
            Arc::new(MockAbstractPriceRefresherService::new()),
            Arc::new(prices_cache),
            Arc::new(MockNotificationSender::new()),
        );

        // Act
        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        // Assert
        assert_eq!(triggered.len(), 1);
    }

    #[tokio::test]
    async fn should_trigger_notification_when_price_above_upper_limit() {
        // Arrange
        let user: UserEntity = Faker.fake();
        let active: ActiveEntity = Faker.fake();
        let mut notification: NotificationEntity = Faker.fake();

        // Устанавливаем лимиты как Price
        notification.limit_lower = Price::rub(90.0);
        notification.limit_upper = Price::rub(110.0);

        let current_price = Price::rub(115.0); // выше верхнего предела

        let mut prices_cache = MockSecurityCurrentPriceCache::new();
        prices_cache
            .expect_get_price()
            .with(eq(active.security_id.clone()))
            .returning(move |_| Ok(current_price.clone()));

        let mut notification_repo = MockNotificationRepository::new();
        notification_repo
            .expect_list_active_notifications()
            .with(eq(active.active_id))
            .returning(move |_| Ok(vec![notification.clone()]));

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_user_actives()
            .with(eq(user.user_id))
            .returning(move |_| Ok(vec![active.clone()]));

        let mut sent_repo = MockSentRepository::new();
        sent_repo
            .expect_get_sent()
            .returning(|_| Err(DomainError::RepositoryError("".into())));
        sent_repo
            .expect_create_sent()
            .returning(|_| Ok(Faker.fake()));

        let service = setup_service(
            Arc::new(MockUserRepository::new()),
            Arc::new(active_repo),
            Arc::new(notification_repo),
            Arc::new(sent_repo),
            Arc::new(MockAbstractPriceRefresherService::new()),
            Arc::new(prices_cache),
            Arc::new(MockNotificationSender::new()),
        );

        // Act
        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        // Assert
        assert_eq!(triggered.len(), 1);
    }

    #[tokio::test]
    async fn should_return_empty_triggered_notifications_when_no_actives() {
        // Arrange
        let user: UserEntity = Faker.fake();

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_user_actives()
            .with(eq(user.user_id))
            .returning(|_| Ok(Vec::new()));

        let mut sent_repo = MockSentRepository::new();
        sent_repo
            .expect_get_sent()
            .returning(|_| Err(DomainError::RepositoryError("".into())));

        let service = setup_service(
            Arc::new(MockUserRepository::new()),
            Arc::new(active_repo),
            Arc::new(MockNotificationRepository::new()),
            Arc::new(sent_repo),
            Arc::new(MockAbstractPriceRefresherService::new()),
            Arc::new(MockSecurityCurrentPriceCache::new()),
            Arc::new(MockNotificationSender::new()),
        );

        // Act
        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        // Assert
        assert_eq!(triggered.len(), 0);
    }

    #[tokio::test]
    async fn should_return_triggered_notifications_for_user_with_actives() {
        // Arrange
        let user: UserEntity = Faker.fake();
        let mut active: ActiveEntity = Faker.fake();
        active.active_id = 1;

        let mut notification1: NotificationEntity = Faker.fake();
        notification1.limit_lower = Price::rub(90.0);
        notification1.limit_upper = Price::rub(110.0);

        let mut notification2: NotificationEntity = Faker.fake();
        notification2.limit_lower = Price::rub(95.0);
        notification2.limit_upper = Price::rub(105.0);

        let current_price = Price::rub(115.0); // выше обоих пределов

        // Клонируем active, чтобы избежать проблем с перемещением
        let active_clone = active.clone();

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_user_actives()
            .with(eq(user.user_id))
            .returning(move |_| Ok(vec![active_clone.clone()]));

        let mut notification_repo = MockNotificationRepository::new();
        notification_repo
            .expect_list_active_notifications()
            .with(eq(active.active_id))
            .returning(move |_| Ok(vec![notification1.clone(), notification2.clone()]));

        let mut prices_cache = MockSecurityCurrentPriceCache::new();
        prices_cache
            .expect_get_price()
            .times(2) // два вызова для двух уведомлений
            .with(eq(active.security_id.clone()))
            .returning(move |_| Ok(current_price.clone()));

        let mut sent_repo = MockSentRepository::new();
        sent_repo
            .expect_get_sent()
            .returning(|_| Err(DomainError::RepositoryError("".into())));
        sent_repo
            .expect_create_sent()
            .returning(|_| Ok(Faker.fake()));

        let service = setup_service(
            Arc::new(MockUserRepository::new()),
            Arc::new(active_repo),
            Arc::new(notification_repo),
            Arc::new(sent_repo),
            Arc::new(MockAbstractPriceRefresherService::new()),
            Arc::new(prices_cache),
            Arc::new(MockNotificationSender::new()),
        );

        // Act
        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        // Assert
        assert_eq!(triggered.len(), 2);
    }

    #[tokio::test]
    async fn should_handle_multiple_users_with_check_users_limits() {
        // Arrange
        let user1: UserEntity = Faker.fake();
        let user2: UserEntity = Faker.fake();

        let active1: ActiveEntity = Faker.fake();
        let active2: ActiveEntity = Faker.fake();

        let mut notification1: NotificationEntity = Faker.fake();
        notification1.limit_lower = Price::rub(80.0);
        notification1.limit_upper = Price::rub(120.0);

        let mut notification2: NotificationEntity = Faker.fake();
        notification2.limit_lower = Price::rub(80.0);
        notification2.limit_upper = Price::rub(120.0);

        // Клонируем переменные для использования в нескольких замыканиях
        let user1_clone = user1.clone();
        let active1_clone = active1.clone();

        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_list_users()
            .returning(move || Ok(vec![user1.clone(), user2.clone()]));

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_user_actives()
            .times(2) // два пользователя
            .returning(move |user_id| {
                if user_id == user1_clone.user_id {
                    Ok(vec![active1.clone()])
                } else {
                    Ok(vec![active2.clone()])
                }
            });

        let mut notification_repo = MockNotificationRepository::new();
        notification_repo
            .expect_list_active_notifications()
            .times(2) // два актива
            .returning(move |active_id| {
                if active_id == active1_clone.active_id {
                    Ok(vec![notification1.clone()])
                } else {
                    Ok(vec![notification2.clone()])
                }
            });

        let mut prices_cache = MockSecurityCurrentPriceCache::new();
        prices_cache
            .expect_get_price()
            .times(2)
            .returning(|_| Ok(Price::rub(100.0))); // цена в пределах нормы

        let mut notification_sender = MockNotificationSender::new();
        notification_sender
            .expect_send_message()
            .returning(|_, _| Ok("sent".to_string()));

        let mut sent_repo = MockSentRepository::new();
        sent_repo
            .expect_get_sent()
            .returning(|_| Err(DomainError::RepositoryError("".into())));
        sent_repo
            .expect_create_sent()
            .returning(|_| Ok(Faker.fake()));

        let service = setup_service(
            Arc::new(user_repo),
            Arc::new(active_repo),
            Arc::new(notification_repo),
            Arc::new(sent_repo),
            Arc::new(MockAbstractPriceRefresherService::new()),
            Arc::new(prices_cache),
            Arc::new(notification_sender),
        );

        // Act & Assert
        // Если метод выполнится без ошибок, значит всё прошло успешно
        let result = service.send_exeeding_messages().await;
        assert!(result.is_ok());
    }
}
