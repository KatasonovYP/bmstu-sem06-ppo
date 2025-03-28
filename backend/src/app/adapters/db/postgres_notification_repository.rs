use crate::app::adapters::db::orm_models;
use crate::app::adapters::db::schema::notifications;
use crate::domain::errors::DomainError;
use crate::domain::models::NotificationEntity;
use crate::ports::outbound::db::NotificationRepository;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct PostgresNotificationRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresNotificationRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationRepository for PostgresNotificationRepository {
    async fn get_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError> {
        let connection = &mut self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        notifications::table
            .filter(notifications::notification_id.eq(notification_id as i32))
            .select(orm_models::OrmSelectNotification::as_select())
            .first(connection)
            .optional()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .map(NotificationEntity::from)
            .ok_or(DomainError::RepositoryError(
                "Notification not found".to_string(),
            ))
    }

    async fn create_notification(
        &self,
        notification: NotificationEntity,
    ) -> Result<NotificationEntity, DomainError> {
        let connection = &mut self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let notification_copy = notification.clone();
        let notification_orm = orm_models::OrmInsertNotification::from(notification);
        diesel::insert_into(notifications::table)
            .values(notification_orm)
            .execute(connection)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
            .map(|_| notification_copy)
    }
}
