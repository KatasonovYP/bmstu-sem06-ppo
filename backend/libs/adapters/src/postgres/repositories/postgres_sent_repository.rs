use std::sync::Arc;

use domain::{
    errors::DomainError,
    models::SentEntity,
    ports::storage::SentRepository,
};
use sea_orm::{
    ActiveValue::Set,
    EntityTrait,
};
use shaku::Component;

use crate::postgres::{
    connection::AbstractConnectionPool,
    schema::{
        prelude::Sent,
        sent,
    },
};

#[derive(Component)]
#[shaku(interface = SentRepository)]
pub struct PostgresSentRepository {
    #[shaku(inject)]
    db: Arc<dyn AbstractConnectionPool>,
}

#[async_trait::async_trait]
impl SentRepository for PostgresSentRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_sent(&self, notification_id: u32) -> Result<SentEntity, DomainError> {
        Sent::find_by_id(notification_id as i32)
            .one(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .map(SentEntity::from)
            .ok_or(DomainError::EntityNotFound {
                entity: "Sent".into(),
                id: notification_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_sent(&self, sent: SentEntity) -> Result<SentEntity, DomainError> {
        Sent::insert(sent::ActiveModel::from(sent))
            .exec_with_returning(self.db.get_connection().await.as_ref())
            .await
            .map(SentEntity::from)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_sent_now(&self, notification_id: u32) -> Result<SentEntity, DomainError> {
        self.create_sent(SentEntity {
            notification_id,
            last_message_time: chrono::Utc::now().naive_utc(),
        })
        .await
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_sent(&self) -> Result<Vec<SentEntity>, DomainError> {
        Ok(Sent::find()
            .all(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .into_iter()
            .map(SentEntity::from)
            .collect())
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn update_sent(&self, sent: SentEntity) -> Result<SentEntity, DomainError> {
        Sent::update(sent::ActiveModel::from(sent))
            .exec(self.db.get_connection().await.as_ref())
            .await
            .map(SentEntity::from)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn delete_sent(&self, notification_id: u32) -> Result<SentEntity, DomainError> {
        Sent::delete_by_id(notification_id as i32)
            .exec_with_returning(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .into_iter()
            .next()
            .map(SentEntity::from)
            .ok_or(DomainError::EntityNotFound {
                entity: std::any::type_name::<SentEntity>().into(),
                id: notification_id.to_string(),
            })
    }
}

impl From<sent::Model> for SentEntity {
    fn from(orm_sent: sent::Model) -> Self {
        SentEntity {
            notification_id: orm_sent.notification_id as u32,
            last_message_time: orm_sent.last_message_time.naive_utc(),
        }
    }
}

impl From<SentEntity> for sent::ActiveModel {
    fn from(entity: SentEntity) -> Self {
        let datetime = chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(
            entity.last_message_time,
            chrono::FixedOffset::east_opt(0).unwrap(),
        );

        Self {
            notification_id: Set(entity.notification_id as i32),
            last_message_time: Set(datetime),
        }
    }
}
