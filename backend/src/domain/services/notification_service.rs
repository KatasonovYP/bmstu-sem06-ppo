use std::sync::Arc;

use async_trait::async_trait;
use shaku::Component;

use crate::domain::{
    errors::DomainError,
    models::NotificationEntity,
    ports::{
        domain::AbstractNotificationService,
        storage::NotificationRepository,
    },
};

#[derive(Clone, Component)]
#[shaku(interface = AbstractNotificationService)]
pub struct NotificationService {
    #[shaku(inject)]
    notification_repository: Arc<dyn NotificationRepository>,
}

impl NotificationService {
    pub fn new(notification_repository: Arc<dyn NotificationRepository>) -> Self {
        Self {
            notification_repository,
        }
    }
}

#[async_trait]
impl AbstractNotificationService for NotificationService {
    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn get_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError> {
        self.notification_repository
            .get_notification(notification_id)
            .await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn create_notification(
        &self,
        notification: NotificationEntity,
    ) -> Result<NotificationEntity, DomainError> {
        self.notification_repository
            .create_notification(notification)
            .await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn list_notifications(&self) -> Result<Vec<NotificationEntity>, DomainError> {
        self.notification_repository.list_notifications().await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn list_active_notifications(
        &self,
        active_id: u32,
    ) -> Result<Vec<NotificationEntity>, DomainError> {
        self.notification_repository
            .list_active_notifications(active_id)
            .await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn update_notification(
        &self,
        notification: NotificationEntity,
    ) -> Result<NotificationEntity, DomainError> {
        self.notification_repository
            .update_notification(notification)
            .await
    }

    #[tracing::instrument(skip(self), err(Debug), ret)]
    async fn delete_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError> {
        self.notification_repository
            .delete_notification(notification_id)
            .await
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
        models::NotificationEntity,
        ports::storage::MockNotificationRepository,
    };

    fn setup_service(repo: Arc<MockNotificationRepository>) -> NotificationService {
        NotificationService {
            notification_repository: repo,
        }
    }

    #[tokio::test]
    async fn should_get_notification_by_id() {
        // Arrange
        let notification_id = 42;
        let mut expected_notification: NotificationEntity = Faker.fake();
        expected_notification.notification_id = notification_id;
        let expected_clone = expected_notification.clone();

        let mut repo = MockNotificationRepository::new();
        repo.expect_get_notification()
            .with(eq(notification_id))
            .returning(move |_| Ok(expected_notification.clone()));

        let service = setup_service(Arc::new(repo));

        // Act
        let result = service.get_notification(notification_id).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().notification_id,
            expected_clone.notification_id
        );
    }

    #[tokio::test]
    async fn should_return_error_when_notification_not_found() {
        // Arrange
        let notification_id = 999;

        let mut repo = MockNotificationRepository::new();
        repo.expect_get_notification()
            .with(eq(notification_id))
            .returning(|id| {
                Err(DomainError::EntityNotFound {
                    entity: "Notification".to_string(),
                    id: id.to_string(),
                })
            });

        let service = setup_service(Arc::new(repo));

        // Act
        let result = service.get_notification(notification_id).await;

        // Assert
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(matches!(error, DomainError::EntityNotFound { .. }));
        }
    }

    #[tokio::test]
    async fn should_create_notification() {
        // Arrange
        let mut notification: NotificationEntity = Faker.fake();
        notification.notification_id = 0; // Предположим, что новое уведомление имеет id = 0

        let mut created_notification = notification.clone();
        created_notification.notification_id = 123; // ID после создания в БД

        let expected_id = created_notification.notification_id;

        let mut repo = MockNotificationRepository::new();
        repo.expect_create_notification()
            .with(eq(notification.clone()))
            .returning(move |_| Ok(created_notification.clone()));

        let service = setup_service(Arc::new(repo));

        // Act
        let result = service.create_notification(notification).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().notification_id, expected_id);
    }

    #[tokio::test]
    async fn should_handle_repository_error_on_create() {
        // Arrange
        let notification: NotificationEntity = Faker.fake();

        let mut repo = MockNotificationRepository::new();
        repo.expect_create_notification()
            .with(eq(notification.clone()))
            .returning(|_| {
                Err(DomainError::RepositoryError(
                    "Database connection error".to_string(),
                ))
            });

        let service = setup_service(Arc::new(repo));

        // Act
        let result = service.create_notification(notification).await;

        // Assert
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(matches!(error, DomainError::RepositoryError(..)));
        }
    }

    #[tokio::test]
    async fn should_list_all_notifications() {
        // Arrange
        let notification1: NotificationEntity = Faker.fake();
        let notification2: NotificationEntity = Faker.fake();
        let expected_notifications = vec![notification1.clone(), notification2.clone()];

        let mut repo = MockNotificationRepository::new();
        repo.expect_list_notifications()
            .returning(move || Ok(expected_notifications.clone()));

        let service = setup_service(Arc::new(repo));

        // Act
        let result = service.list_notifications().await;

        // Assert
        assert!(result.is_ok());
        let notifications = result.unwrap();
        assert_eq!(notifications.len(), 2);
        assert_eq!(
            notifications[0].notification_id,
            notification1.notification_id
        );
        assert_eq!(
            notifications[1].notification_id,
            notification2.notification_id
        );
    }

    #[tokio::test]
    async fn should_handle_repository_error_on_list() {
        // Arrange
        let mut repo = MockNotificationRepository::new();
        repo.expect_list_notifications()
            .returning(|| Err(DomainError::RepositoryError("Database error".to_string())));

        let service = setup_service(Arc::new(repo));

        // Act
        let result = service.list_notifications().await;

        // Assert
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(matches!(error, DomainError::RepositoryError(..)));
        }
    }

    #[tokio::test]
    async fn should_list_active_notifications() {
        // Arrange
        let active_id = 42;
        let notification1: NotificationEntity = Faker.fake();
        let notification2: NotificationEntity = Faker.fake();
        let expected_notifications = vec![notification1.clone(), notification2.clone()];

        let mut repo = MockNotificationRepository::new();
        repo.expect_list_active_notifications()
            .with(eq(active_id))
            .returning(move |_| Ok(expected_notifications.clone()));

        let service = setup_service(Arc::new(repo));

        // Act
        let result = service.list_active_notifications(active_id).await;

        // Assert
        assert!(result.is_ok());
        let notifications = result.unwrap();
        assert_eq!(notifications.len(), 2);
        assert_eq!(
            notifications[0].notification_id,
            notification1.notification_id
        );
        assert_eq!(
            notifications[1].notification_id,
            notification2.notification_id
        );
    }

    #[tokio::test]
    async fn should_return_empty_list_for_nonexistent_active() {
        // Arrange
        let active_id = 999; // несуществующий active_id

        let mut repo = MockNotificationRepository::new();
        repo.expect_list_active_notifications()
            .with(eq(active_id))
            .returning(|_| Ok(Vec::new()));

        let service = setup_service(Arc::new(repo));

        // Act
        let result = service.list_active_notifications(active_id).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn should_handle_repository_error_on_list_active() {
        // Arrange
        let active_id = 42;

        let mut repo = MockNotificationRepository::new();
        repo.expect_list_active_notifications()
            .with(eq(active_id))
            .returning(|_| Err(DomainError::RepositoryError("Database error".to_string())));

        let service = setup_service(Arc::new(repo));

        // Act
        let result = service.list_active_notifications(active_id).await;

        // Assert
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(matches!(error, DomainError::RepositoryError(..)));
        }
    }
}
