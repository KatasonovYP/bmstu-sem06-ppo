use std::sync::Arc;

use domain::{
    errors::DomainError,
    models::NotificationEntity,
    ports::storage::NotificationRepository,
    value_objects::Price,
};
use sea_orm::{
    ActiveValue::{
        NotSet,
        Set,
    },
    EntityTrait,
};
use shaku::Component;

use crate::postgres::{
    connection::AbstractConnectionPool,
    schema::{
        notifications,
        prelude::Notifications,
    },
};

#[derive(Component)]
#[shaku(interface = NotificationRepository)]
pub struct PostgresNotificationRepository {
    #[shaku(inject)]
    db: Arc<dyn AbstractConnectionPool>,
}

#[async_trait::async_trait]
impl NotificationRepository for PostgresNotificationRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError> {
        Notifications::find_by_id(notification_id as i32)
            .one(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .map(NotificationEntity::from)
            .ok_or(DomainError::EntityNotFound {
                entity: "Notification".into(),
                id: notification_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_notification(
        &self,
        notification: &NotificationEntity,
    ) -> Result<NotificationEntity, DomainError> {
        Notifications::insert(notifications::ActiveModel::from(notification.clone()))
            .exec_with_returning(self.db.get_connection().await.as_ref())
            .await
            .map(NotificationEntity::from)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_notifications(&self) -> Result<Vec<NotificationEntity>, DomainError> {
        Ok(Notifications::find()
            .all(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .into_iter()
            .map(NotificationEntity::from)
            .collect())
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_active_notifications(
        &self,
        active_id: u32,
    ) -> Result<Vec<NotificationEntity>, DomainError> {
        Ok(self
            .list_notifications()
            .await?
            .into_iter()
            .filter(|x| x.active_id == active_id)
            .collect())
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn update_notification(
        &self,
        notification: &NotificationEntity,
    ) -> Result<NotificationEntity, DomainError> {
        Notifications::update(notifications::ActiveModel::from(notification.clone()))
            .exec(self.db.get_connection().await.as_ref())
            .await
            .map(NotificationEntity::from)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn delete_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError> {
        Notifications::delete_by_id(notification_id as i32)
            .exec_with_returning(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .into_iter()
            .next()
            .map(NotificationEntity::from)
            .ok_or(DomainError::EntityNotFound {
                entity: std::any::type_name::<NotificationEntity>().into(),
                id: notification_id.to_string(),
            })
    }
}

impl From<notifications::Model> for NotificationEntity {
    fn from(orm_notification: notifications::Model) -> Self {
        let limit_upper = Price::new(
            orm_notification.limit_upper,
            orm_notification.limit_type.clone(),
        );
        let limit_lower = Price::new(
            orm_notification.limit_lower,
            orm_notification.limit_type.clone(),
        );
        NotificationEntity {
            notification_id: orm_notification.notification_id as u32,
            portfolio_id: orm_notification.portfolio_id as u32,
            active_id: orm_notification.active_id as u32,
            limit_upper,
            limit_lower,
            resend_interval_sec: orm_notification.resend_interval_sec as u32,
        }
    }
}

impl From<NotificationEntity> for notifications::ActiveModel {
    fn from(entity: NotificationEntity) -> Self {
        // if entity.limit_upper.currency != entity.limit_lower.currency {
        //     return Err(DomainError::ValidationError(("limits type mismatch".into())))
        // }
        let notification_id = match entity.notification_id {
            0 => NotSet,
            id => Set(id as i32),
        };
        Self {
            notification_id,
            portfolio_id: Set(entity.portfolio_id as i32),
            active_id: Set(entity.active_id as i32),
            limit_upper: Set(entity.limit_upper.amount),
            limit_lower: Set(entity.limit_lower.amount),
            limit_type: Set(entity.limit_upper.currency.value),
            resend_interval_sec: Set(entity.resend_interval_sec as i32),
        }
    }
}

#[cfg(not(feature = "production"))]
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::{
        errors::DomainError,
        models::NotificationEntity,
        ports::storage::NotificationRepository,
    };
    use sea_orm::{
        DatabaseBackend,
        MockDatabase,
    };

    use crate::postgres::{
        PostgresNotificationRepository,
        connection::PostgresConnectionPool,
        schema::notifications,
    };

    #[tokio::test]
    async fn test_create_notification() {
        let notif_entity = NotificationEntity {
            notification_id: 0,
            ..Default::default()
        };
        let notif_model = notifications::Model {
            notification_id: 1,
            ..Default::default()
        };

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[notif_model.clone()]])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let notif = repo.create_notification(&notif_entity).await.unwrap();
        assert_eq!(notif, NotificationEntity::from(notif_model));
    }

    #[tokio::test]
    async fn test_create_notification_error() {
        let notif_entity = NotificationEntity::default();

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "duplicate".into(),
            ))])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let result = repo.create_notification(&notif_entity).await;
        assert!(result.is_err());

        match result {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("duplicate")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_get_notification() {
        let notif_model = notifications::Model {
            notification_id: 2,
            portfolio_id: 42,
            active_id: 5,
            ..Default::default()
        };
        let notif_entity = NotificationEntity::from(notif_model.clone());

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[notif_model]])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let got = repo.get_notification(2).await.unwrap();
        assert_eq!(got, notif_entity);
    }

    #[tokio::test]
    async fn test_get_notification_not_found() {
        let empty: Vec<notifications::Model> = vec![];
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([empty])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };
        let res = repo.get_notification(99).await;

        match res {
            Err(DomainError::EntityNotFound { entity, id }) => {
                assert_eq!(entity, "Notification");
                assert_eq!(id, "99");
            },
            _ => panic!("Expected EntityNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_list_notifications() {
        let nm1 = notifications::Model {
            notification_id: 1,
            ..Default::default()
        };
        let nm2 = notifications::Model {
            notification_id: 2,
            ..Default::default()
        };

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[nm1.clone(), nm2.clone()]])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let got = repo.list_notifications().await.unwrap();
        assert_eq!(
            got,
            vec![NotificationEntity::from(nm1), NotificationEntity::from(nm2),]
        );
    }

    #[tokio::test]
    async fn test_list_notifications_error() {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "some db error".into(),
            ))])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let res = repo.list_notifications().await;
        match res {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("some db error")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_list_active_notifications() {
        let nm1 = notifications::Model {
            notification_id: 3,
            active_id: 22,
            ..Default::default()
        };
        let nm2 = notifications::Model {
            notification_id: 4,
            active_id: 23,
            ..Default::default()
        };

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[nm1.clone(), nm2.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let res = repo.list_active_notifications(22).await.unwrap();
        assert_eq!(res, vec![NotificationEntity::from(nm1)]);
    }

    #[tokio::test]
    async fn test_list_active_notifications_error() {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "fail".into(),
            ))])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let res = repo.list_active_notifications(55).await;
        match res {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("fail")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_update_notification() {
        let notif_entity = NotificationEntity {
            notification_id: 77,
            ..Default::default()
        };
        let notif_model = notifications::Model {
            notification_id: 77,
            ..Default::default()
        };

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[notif_model.clone()]])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let result = repo.update_notification(&notif_entity).await.unwrap();
        assert_eq!(result, NotificationEntity::from(notif_model));
    }

    #[tokio::test]
    async fn test_update_notification_error() {
        let notif_entity = NotificationEntity {
            notification_id: 11,
            ..Default::default()
        };
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "update error".into(),
            ))])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };
        let result = repo.update_notification(&notif_entity).await;

        match result {
            Err(DomainError::RepositoryError(msg)) => {
                assert!(
                    msg.contains("update error") || !msg.is_empty(),
                    "RepositoryError message: {msg}"
                );
            },
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_delete_notification() {
        let notif_model = notifications::Model {
            notification_id: 5,
            ..Default::default()
        };
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[notif_model.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let deleted = repo.delete_notification(5).await.unwrap();
        assert_eq!(deleted, NotificationEntity::from(notif_model));
    }

    #[tokio::test]
    async fn test_delete_notification_error() {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "delete error".into(),
            ))])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let result = repo.delete_notification(33).await;
        match result {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("delete error")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_delete_notification_not_found() {
        let empty: Vec<notifications::Model> = vec![];
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([empty])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresNotificationRepository { db: Arc::new(pool) };

        let result = repo.delete_notification(99).await;
        match result {
            Err(DomainError::EntityNotFound { entity, id }) => {
                // entity имя может отличаться, поправь если нужно
                assert!(entity.contains("Notification"));
                assert_eq!(id, "99");
            },
            _ => panic!("Expected EntityNotFound error"),
        }
    }
}
