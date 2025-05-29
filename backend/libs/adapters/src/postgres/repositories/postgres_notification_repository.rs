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
        notification: NotificationEntity,
    ) -> Result<NotificationEntity, DomainError> {
        Notifications::insert(notifications::ActiveModel::from(notification))
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
        notification: NotificationEntity,
    ) -> Result<NotificationEntity, DomainError> {
        Notifications::update(notifications::ActiveModel::from(notification))
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
        }
    }
}
