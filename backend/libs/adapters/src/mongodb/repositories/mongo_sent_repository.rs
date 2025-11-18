use std::sync::Arc;

use chrono::Utc;
use domain::{
    errors::DomainError,
    models::SentEntity,
    ports::storage::SentRepository,
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
        bson_to_naive,
        collect_cursor,
        id_to_u32,
        naive_to_bson,
        repo_error,
    },
};

#[derive(Component)]
#[shaku(interface = SentRepository)]
pub struct MongoSentRepository {
    #[shaku(inject)]
    connection: Arc<dyn MongoConnection>,
}

impl MongoSentRepository {
    const COLLECTION: &'static str = "sent";

    async fn collection(&self) -> mongodb::Collection<SentDocument> {
        self.connection
            .database()
            .await
            .collection(Self::COLLECTION)
    }
}

#[async_trait::async_trait]
impl SentRepository for MongoSentRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_sent(&self, notification_id: u32) -> Result<SentEntity, DomainError> {
        self.collection()
            .await
            .find_one(doc! { "notification_id": i64::from(notification_id) }, None)
            .await
            .map_err(repo_error)?
            .map(SentEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "Sent".into(),
                id: notification_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_sent(&self, sent: &SentEntity) -> Result<SentEntity, DomainError> {
        let document = SentDocument::from(sent);
        self.collection()
            .await
            .insert_one(&document, None)
            .await
            .map_err(repo_error)?;
        Ok(sent.clone())
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_sent_now(&self, notification_id: u32) -> Result<SentEntity, DomainError> {
        self.create_sent(&SentEntity {
            notification_id,
            last_message_time: Utc::now().naive_utc(),
        })
        .await
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_sent(&self) -> Result<Vec<SentEntity>, DomainError> {
        let cursor = self
            .collection()
            .await
            .find(None, None)
            .await
            .map_err(repo_error)?;
        Ok(collect_cursor(cursor)
            .await?
            .into_iter()
            .map(SentEntity::try_from)
            .collect::<Result<Vec<_>, _>>()?)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn update_sent(&self, sent: &SentEntity) -> Result<SentEntity, DomainError> {
        let options = FindOneAndReplaceOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();
        self.collection()
            .await
            .find_one_and_replace(
                doc! { "notification_id": i64::from(sent.notification_id) },
                SentDocument::from(sent),
                options,
            )
            .await
            .map_err(repo_error)?
            .map(SentEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "Sent".into(),
                id: sent.notification_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn delete_sent(&self, notification_id: u32) -> Result<SentEntity, DomainError> {
        self.collection()
            .await
            .find_one_and_delete(doc! { "notification_id": i64::from(notification_id) }, None)
            .await
            .map_err(repo_error)?
            .map(SentEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "Sent".into(),
                id: notification_id.to_string(),
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SentDocument {
    pub notification_id: i64,
    pub last_message_time: mongodb::bson::DateTime,
}

impl TryFrom<SentDocument> for SentEntity {
    type Error = DomainError;

    fn try_from(value: SentDocument) -> Result<Self, Self::Error> {
        Ok(SentEntity {
            notification_id: id_to_u32(value.notification_id, "notification_id")?,
            last_message_time: bson_to_naive(value.last_message_time),
        })
    }
}

impl From<&SentEntity> for SentDocument {
    fn from(value: &SentEntity) -> Self {
        Self {
            notification_id: i64::from(value.notification_id),
            last_message_time: naive_to_bson(value.last_message_time),
        }
    }
}
