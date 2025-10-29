use std::sync::Arc;

use shaku::Component;

use crate::{
    errors::DomainError,
    models::{
        ActiveEntity,
        NotificationEntity,
        SentEntity,
        UserEntity,
    },
    ports::{
        domain::{
            AbstractLimitMonitorService,
            AbstractPriceCacheService,
        },
        sender::NotificationSender,
        storage::{
            ActiveRepository,
            NotificationRepository,
            SentRepository,
            UserRepository,
        },
    },
    value_objects::Price,
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
    price_cache_service: Arc<dyn AbstractPriceCacheService>,
    #[shaku(inject)]
    notification_sender: Arc<dyn NotificationSender>,
}

#[derive(Clone, Debug)]
struct Message {
    notification_id: u32,
    message: String,
}

/// Context for processing a single notification
struct NotificationContext<'a> {
    notification: &'a NotificationEntity,
    active: &'a ActiveEntity,
    current_price: Price,
}

/// Result of checking if a notification should be sent
#[derive(Debug)]
struct NotificationCheck {
    should_send: bool,
    _reason: NotificationCheckReason,
}

#[derive(Debug, PartialEq)]
enum NotificationCheckReason {
    IntervalNotPassed,
    WithinLimits,
    LimitExceeded,
    FirstNotification,
}

impl LimitMonitorService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        active_repo: Arc<dyn ActiveRepository>,
        notification_repo: Arc<dyn NotificationRepository>,
        sent_repo: Arc<dyn SentRepository>,
        price_cache_service: Arc<dyn AbstractPriceCacheService>,
        notification_sender: Arc<dyn NotificationSender>,
    ) -> Self {
        Self {
            user_repo,
            active_repo,
            notification_repo,
            sent_repo,
            price_cache_service,
            notification_sender,
        }
    }

    async fn get_user_exeeding_messages(
        &self,
        user: &UserEntity,
    ) -> Result<Vec<Message>, DomainError> {
        let actives = self.active_repo.list_user_actives(user.user_id).await?;

        let mut messages = Vec::new();
        for active in &actives {
            let active_messages = self.process_active_notifications(active).await?;
            messages.extend(active_messages);
        }

        Ok(messages)
    }

    /// Process all notifications for a single active
    async fn process_active_notifications(
        &self,
        active: &ActiveEntity,
    ) -> Result<Vec<Message>, DomainError> {
        let notifications = self
            .notification_repo
            .list_active_notifications(active.active_id)
            .await?;

        let mut messages = Vec::new();

        for notification in &notifications {
            if let Some(message) = self
                .process_single_notification(notification, active)
                .await?
            {
                messages.push(message);
            }
        }

        Ok(messages)
    }

    /// Process a single notification and return a message if it should be sent
    async fn process_single_notification(
        &self,
        notification: &NotificationEntity,
        active: &ActiveEntity,
    ) -> Result<Option<Message>, DomainError> {
        // Check if we should send based on resend interval
        let resend_check = self.check_resend_interval(notification).await?;
        if !resend_check.should_send {
            return Ok(None);
        }

        // Get current price and check limits
        let current_price = self.price_cache_service.get_price(active).await?;

        let context = NotificationContext {
            notification,
            active,
            current_price,
        };

        let limit_check = self.check_price_limits(&context);

        if limit_check.should_send {
            let message = self.create_notification_message(&context);
            Ok(Some(Message {
                notification_id: notification.notification_id,
                message,
            }))
        } else {
            Ok(None)
        }
    }

    /// Check if enough time has passed since the last notification was sent
    async fn check_resend_interval(
        &self,
        notification: &NotificationEntity,
    ) -> Result<NotificationCheck, DomainError> {
        match self.sent_repo.get_sent(notification.notification_id).await {
            Ok(sent) => {
                let should_send = self.is_resend_interval_exceeded(notification, &sent);
                Ok(NotificationCheck {
                    should_send,
                    _reason: if should_send {
                        NotificationCheckReason::LimitExceeded
                    } else {
                        NotificationCheckReason::IntervalNotPassed
                    },
                })
            },
            Err(_) => {
                // No previous send record, so this is the first notification
                Ok(NotificationCheck {
                    should_send: true,
                    _reason: NotificationCheckReason::FirstNotification,
                })
            },
        }
    }

    /// Check if the resend interval has been exceeded
    fn is_resend_interval_exceeded(
        &self,
        notification: &NotificationEntity,
        sent: &SentEntity,
    ) -> bool {
        let now = chrono::Utc::now().naive_utc();
        let time_since_last = now.signed_duration_since(sent.last_message_time);
        time_since_last.num_seconds() >= notification.resend_interval_sec as i64
    }

    /// Check if the current price exceeds the configured limits
    fn check_price_limits(&self, context: &NotificationContext) -> NotificationCheck {
        let is_below_lower = context.current_price < context.notification.limit_lower;
        let is_above_upper = context.current_price > context.notification.limit_upper;

        let should_send = is_below_lower || is_above_upper;

        NotificationCheck {
            should_send,
            _reason: if should_send {
                NotificationCheckReason::LimitExceeded
            } else {
                NotificationCheckReason::WithinLimits
            },
        }
    }

    /// Create a formatted notification message
    fn create_notification_message(&self, context: &NotificationContext) -> String {
        format!(
            "Limit {} - {} is exceeded for security {} with current price: {}",
            context.notification.limit_lower.amount,
            context.notification.limit_upper.amount,
            context.active.security_id,
            context.current_price.amount,
        )
    }

    /// Send messages to a user and update sent records
    async fn send_user_messages(
        &self,
        user: &UserEntity,
        messages: Vec<Message>,
    ) -> Result<(), DomainError> {
        if messages.is_empty() {
            return Ok(());
        }

        let combined_message = self.combine_messages(&messages);

        self.notification_sender
            .send_message(user.chat_id, combined_message)
            .await?;

        self.update_sent_records(messages).await?;

        Ok(())
    }

    /// Combine multiple messages into a single string
    fn combine_messages(&self, messages: &[Message]) -> String {
        messages
            .iter()
            .map(|m| &m.message)
            .fold(String::new(), |mut acc, msg| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(msg);
                acc
            })
    }

    /// Update sent records for all notifications that were sent
    async fn update_sent_records(&self, messages: Vec<Message>) -> Result<(), DomainError> {
        for message in messages {
            self.sent_repo
                .create_sent_now(message.notification_id)
                .await?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl AbstractLimitMonitorService for LimitMonitorService {
    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn send_exeeding_messages(&self) -> Result<(), DomainError> {
        let users = self.user_repo.list_users().await?;

        for user in &users {
            let messages = self.get_user_exeeding_messages(user).await?;
            self.send_user_messages(user, messages).await?;
        }

        Ok(())
    }
}
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
        models::{
            ActiveEntity,
            NotificationEntity,
            SentEntity,
            UserEntity,
        },
        ports::{
            domain::MockAbstractPriceCacheService,
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

    #[tokio::test]
    async fn should_not_trigger_notification_when_price_within_limits() {
        let user: UserEntity = Faker.fake();
        let active: ActiveEntity = Faker.fake();
        let mut notification: NotificationEntity = Faker.fake();

        notification.limit_lower = Price::rub(90.0);
        notification.limit_upper = Price::rub(110.0);
        notification.resend_interval_sec = 3600; // 1 hour

        let current_price = Price::rub(100.0);

        let mut price_cache_service = MockAbstractPriceCacheService::new();
        price_cache_service
            .expect_get_price()
            .with(eq(active.clone()))
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

        let service = LimitMonitorService {
            user_repo: Arc::new(MockUserRepository::new()),
            active_repo: Arc::new(active_repo),
            notification_repo: Arc::new(notification_repo),
            sent_repo: Arc::new(sent_repo),
            price_cache_service: Arc::new(price_cache_service),
            notification_sender: Arc::new(MockNotificationSender::new()),
        };

        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        assert!(triggered.is_empty());
    }

    #[tokio::test]
    async fn should_trigger_notification_when_price_below_lower_limit() {
        let user: UserEntity = Faker.fake();
        let active: ActiveEntity = Faker.fake();
        let mut notification: NotificationEntity = Faker.fake();

        notification.limit_lower = Price::rub(90.0);
        notification.limit_upper = Price::rub(110.0);
        notification.resend_interval_sec = 3600; // 1 hour

        let current_price = Price::rub(85.0);

        let mut price_cache_service = MockAbstractPriceCacheService::new();
        price_cache_service
            .expect_get_price()
            .with(eq(active.clone()))
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

        let service = LimitMonitorService {
            user_repo: Arc::new(MockUserRepository::new()),
            active_repo: Arc::new(active_repo),
            notification_repo: Arc::new(notification_repo),
            sent_repo: Arc::new(sent_repo),
            price_cache_service: Arc::new(price_cache_service),
            notification_sender: Arc::new(MockNotificationSender::new()),
        };

        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        assert_eq!(triggered.len(), 1);
    }

    #[tokio::test]
    async fn should_trigger_notification_when_price_above_upper_limit() {
        let user: UserEntity = Faker.fake();
        let active: ActiveEntity = Faker.fake();
        let mut notification: NotificationEntity = Faker.fake();

        notification.limit_lower = Price::rub(90.0);
        notification.limit_upper = Price::rub(110.0);

        let current_price = Price::rub(115.0);

        let mut price_cache_service = MockAbstractPriceCacheService::new();
        price_cache_service
            .expect_get_price()
            .with(eq(active.clone()))
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

        let service = LimitMonitorService {
            user_repo: Arc::new(MockUserRepository::new()),
            active_repo: Arc::new(active_repo),
            notification_repo: Arc::new(notification_repo),
            sent_repo: Arc::new(sent_repo),
            price_cache_service: Arc::new(price_cache_service),
            notification_sender: Arc::new(MockNotificationSender::new()),
        };

        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        assert_eq!(triggered.len(), 1);
    }

    #[tokio::test]
    async fn should_return_empty_triggered_notifications_when_no_actives() {
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

        let service = LimitMonitorService {
            user_repo: Arc::new(MockUserRepository::new()),
            active_repo: Arc::new(active_repo),
            notification_repo: Arc::new(MockNotificationRepository::new()),
            sent_repo: Arc::new(sent_repo),
            price_cache_service: Arc::new(MockAbstractPriceCacheService::new()),
            notification_sender: Arc::new(MockNotificationSender::new()),
        };

        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

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

        let current_price = Price::rub(115.0);

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

        let mut price_cache_service = MockAbstractPriceCacheService::new();
        price_cache_service
            .expect_get_price()
            .times(2)
            .with(eq(active.clone()))
            .returning(move |_| Ok(current_price.clone()));

        let mut sent_repo = MockSentRepository::new();
        sent_repo
            .expect_get_sent()
            .returning(|_| Err(DomainError::RepositoryError("".into())));
        sent_repo
            .expect_create_sent()
            .returning(|_| Ok(Faker.fake()));

        let service = LimitMonitorService {
            user_repo: Arc::new(MockUserRepository::new()),
            active_repo: Arc::new(active_repo),
            notification_repo: Arc::new(notification_repo),
            sent_repo: Arc::new(sent_repo),
            price_cache_service: Arc::new(price_cache_service),
            notification_sender: Arc::new(MockNotificationSender::new()),
        };

        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        assert_eq!(triggered.len(), 2);
    }

    #[tokio::test]
    async fn should_handle_multiple_users_with_check_users_limits() {
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

        let user1_clone = user1.clone();
        let active1_clone = active1.clone();

        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_list_users()
            .returning(move || Ok(vec![user1.clone(), user2.clone()]));

        let mut active_repo = MockActiveRepository::new();
        active_repo
            .expect_list_user_actives()
            .times(2)
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
            .times(2)
            .returning(move |active_id| {
                if active_id == active1_clone.active_id {
                    Ok(vec![notification1.clone()])
                } else {
                    Ok(vec![notification2.clone()])
                }
            });

        let mut price_cache_service = MockAbstractPriceCacheService::new();
        price_cache_service
            .expect_get_price()
            .times(2)
            .returning(|_| Ok(Price::rub(100.0)));

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

        let service = LimitMonitorService {
            user_repo: Arc::new(user_repo),
            active_repo: Arc::new(active_repo),
            notification_repo: Arc::new(notification_repo),
            sent_repo: Arc::new(sent_repo),
            price_cache_service: Arc::new(price_cache_service),
            notification_sender: Arc::new(notification_sender),
        };

        let result = service.send_exeeding_messages().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_trigger_notification_when_resend_interval_not_passed() {
        let user: UserEntity = Faker.fake();
        let active: ActiveEntity = Faker.fake();
        let mut notification: NotificationEntity = Faker.fake();

        notification.limit_lower = Price::rub(90.0);
        notification.limit_upper = Price::rub(110.0);
        notification.resend_interval_sec = 3600; // 1 hour

        let current_price = Price::rub(85.0);

        let mut price_cache_service = MockAbstractPriceCacheService::new();
        price_cache_service
            .expect_get_price()
            .with(eq(active.clone()))
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
        sent_repo.expect_get_sent().returning(|_| {
            Ok(SentEntity {
                notification_id: 1,
                last_message_time: chrono::Utc::now().naive_utc(), // Just sent
            })
        });

        let service = LimitMonitorService {
            user_repo: Arc::new(MockUserRepository::new()),
            active_repo: Arc::new(active_repo),
            notification_repo: Arc::new(notification_repo),
            sent_repo: Arc::new(sent_repo),
            price_cache_service: Arc::new(price_cache_service),
            notification_sender: Arc::new(MockNotificationSender::new()),
        };

        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        assert!(triggered.is_empty());
    }

    #[tokio::test]
    async fn should_trigger_notification_when_resend_interval_passed() {
        let user: UserEntity = Faker.fake();
        let active: ActiveEntity = Faker.fake();
        let mut notification: NotificationEntity = Faker.fake();

        notification.limit_lower = Price::rub(90.0);
        notification.limit_upper = Price::rub(110.0);
        notification.resend_interval_sec = 3600; // 1 hour

        let current_price = Price::rub(85.0);

        let mut price_cache_service = MockAbstractPriceCacheService::new();
        price_cache_service
            .expect_get_price()
            .with(eq(active.clone()))
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
            .returning(|_| {
                Ok(SentEntity {
                    notification_id: 1,
                    last_message_time: chrono::Utc::now().naive_utc() - chrono::Duration::hours(2), // Sent 2 hours ago
                })
            });
        sent_repo
            .expect_create_sent()
            .returning(|_| Ok(Faker.fake()));

        let service = LimitMonitorService {
            user_repo: Arc::new(MockUserRepository::new()),
            active_repo: Arc::new(active_repo),
            notification_repo: Arc::new(notification_repo),
            sent_repo: Arc::new(sent_repo),
            price_cache_service: Arc::new(price_cache_service),
            notification_sender: Arc::new(MockNotificationSender::new()),
        };

        let triggered = service.get_user_exeeding_messages(&user).await.unwrap();

        assert_eq!(triggered.len(), 1);
    }
}
