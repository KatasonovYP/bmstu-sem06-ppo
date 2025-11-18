use std::sync::Arc;

use domain::{
    errors::DomainError,
    models::NotificationEntity,
    ports::storage::NotificationRepository,
    value_objects::Price,
};
use mongodb::{
    bson::doc,
    options::FindOneAndReplaceOptions,
};
use serde::{
    Deserialize,
    Serialize,
};
use shaku::Component;

use crate::mongodb::{
    client::MongoConnection,
    common::{
        PriceDocument,
        collect_cursor,
        id_to_u32,
        next_sequence,
        repo_error,
    },
};

#[derive(Component)]
#[shaku(interface = NotificationRepository)]
pub struct MongoNotificationRepository {
    #[shaku(inject)]
    connection: Arc<dyn MongoConnection>,
}

impl MongoNotificationRepository {
    const COLLECTION: &'static str = "notifications";

    async fn collection(&self) -> mongodb::Collection<NotificationDocument> {
        self.connection
            .database()
            .await
            .collection(Self::COLLECTION)
    }
}

#[async_trait::async_trait]
impl NotificationRepository for MongoNotificationRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_notification(
        &self,
        notification: &NotificationEntity,
    ) -> Result<NotificationEntity, DomainError> {
        let db = self.connection.database().await;
        let collection = db.collection::<NotificationDocument>(Self::COLLECTION);
        let mut entity = notification.clone();
        if entity.notification_id == 0 {
            entity.notification_id = next_sequence(db.as_ref(), "notifications").await?;
        }
        let document = NotificationDocument::from(&entity);
        collection
            .insert_one(&document, None)
            .await
            .map_err(repo_error)?;
        Ok(entity)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError> {
        self.collection()
            .await
            .find_one(doc! { "notification_id": i64::from(notification_id) }, None)
            .await
            .map_err(repo_error)?
            .map(NotificationEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "Notification".into(),
                id: notification_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_notifications(&self) -> Result<Vec<NotificationEntity>, DomainError> {
        let cursor = self
            .collection()
            .await
            .find(None, None)
            .await
            .map_err(repo_error)?;
        Ok(collect_cursor(cursor)
            .await?
            .into_iter()
            .map(NotificationEntity::try_from)
            .collect::<Result<Vec<_>, _>>()?)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_active_notifications(
        &self,
        active_id: u32,
    ) -> Result<Vec<NotificationEntity>, DomainError> {
        let cursor = self
            .collection()
            .await
            .find(doc! { "active_id": i64::from(active_id) }, None)
            .await
            .map_err(repo_error)?;
        Ok(collect_cursor(cursor)
            .await?
            .into_iter()
            .map(NotificationEntity::try_from)
            .collect::<Result<Vec<_>, _>>()?)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn update_notification(
        &self,
        notification: &NotificationEntity,
    ) -> Result<NotificationEntity, DomainError> {
        let options = FindOneAndReplaceOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();
        self.collection()
            .await
            .find_one_and_replace(
                doc! { "notification_id": i64::from(notification.notification_id) },
                NotificationDocument::from(notification),
                options,
            )
            .await
            .map_err(repo_error)?
            .map(NotificationEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "Notification".into(),
                id: notification.notification_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn delete_notification(
        &self,
        notification_id: u32,
    ) -> Result<NotificationEntity, DomainError> {
        self.collection()
            .await
            .find_one_and_delete(doc! { "notification_id": i64::from(notification_id) }, None)
            .await
            .map_err(repo_error)?
            .map(NotificationEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "Notification".into(),
                id: notification_id.to_string(),
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationDocument {
    pub notification_id: i64,
    pub portfolio_id: i64,
    pub active_id: i64,
    pub limit_upper: PriceDocument,
    pub limit_lower: PriceDocument,
    pub resend_interval_sec: i64,
}

impl TryFrom<NotificationDocument> for NotificationEntity {
    type Error = DomainError;

    fn try_from(value: NotificationDocument) -> Result<Self, Self::Error> {
        Ok(NotificationEntity {
            notification_id: id_to_u32(value.notification_id, "notification_id")?,
            portfolio_id: id_to_u32(value.portfolio_id, "portfolio_id")?,
            active_id: id_to_u32(value.active_id, "active_id")?,
            limit_upper: Price::from(value.limit_upper),
            limit_lower: Price::from(value.limit_lower),
            resend_interval_sec: id_to_u32(value.resend_interval_sec, "resend_interval_sec")?,
        })
    }
}

impl From<&NotificationEntity> for NotificationDocument {
    fn from(value: &NotificationEntity) -> Self {
        Self {
            notification_id: i64::from(value.notification_id),
            portfolio_id: i64::from(value.portfolio_id),
            active_id: i64::from(value.active_id),
            limit_upper: PriceDocument::from(&value.limit_upper),
            limit_lower: PriceDocument::from(&value.limit_lower),
            resend_interval_sec: i64::from(value.resend_interval_sec),
        }
    }
}
