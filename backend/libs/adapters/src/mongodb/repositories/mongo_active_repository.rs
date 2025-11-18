use std::sync::Arc;

use domain::{
    errors::DomainError,
    models::ActiveEntity,
    ports::storage::ActiveRepository,
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
#[shaku(interface = ActiveRepository)]
pub struct MongoActiveRepository {
    #[shaku(inject)]
    connection: Arc<dyn MongoConnection>,
}

impl MongoActiveRepository {
    const COLLECTION: &'static str = "actives";

    async fn collection(&self) -> mongodb::Collection<ActiveDocument> {
        self.connection
            .database()
            .await
            .collection(Self::COLLECTION)
    }
}

#[async_trait::async_trait]
impl ActiveRepository for MongoActiveRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_active(&self, active: &ActiveEntity) -> Result<ActiveEntity, DomainError> {
        let db = self.connection.database().await;
        let collection = db.collection::<ActiveDocument>(Self::COLLECTION);
        let mut entity = active.clone();
        if entity.active_id == 0 {
            entity.active_id = next_sequence(db.as_ref(), "actives").await?;
        }
        let document = ActiveDocument::from(&entity);
        collection
            .insert_one(&document, None)
            .await
            .map_err(repo_error)?;
        Ok(entity)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError> {
        self.collection()
            .await
            .find_one(doc! { "active_id": i64::from(active_id) }, None)
            .await
            .map_err(repo_error)?
            .map(ActiveEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "Active".into(),
                id: active_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_actives(&self) -> Result<Vec<ActiveEntity>, DomainError> {
        let cursor = self
            .collection()
            .await
            .find(None, None)
            .await
            .map_err(repo_error)?;
        Ok(collect_cursor(cursor)
            .await?
            .into_iter()
            .map(ActiveEntity::try_from)
            .collect::<Result<Vec<_>, _>>()?)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_user_actives(&self, user_id: u32) -> Result<Vec<ActiveEntity>, DomainError> {
        let cursor = self
            .collection()
            .await
            .find(doc! { "user_id": i64::from(user_id) }, None)
            .await
            .map_err(repo_error)?;
        Ok(collect_cursor(cursor)
            .await?
            .into_iter()
            .map(ActiveEntity::try_from)
            .collect::<Result<Vec<_>, _>>()?)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn update_active(&self, active: &ActiveEntity) -> Result<ActiveEntity, DomainError> {
        let options = FindOneAndReplaceOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();
        self.collection()
            .await
            .find_one_and_replace(
                doc! { "active_id": i64::from(active.active_id) },
                ActiveDocument::from(active),
                options,
            )
            .await
            .map_err(repo_error)?
            .map(ActiveEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "Active".into(),
                id: active.active_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn delete_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError> {
        self.collection()
            .await
            .find_one_and_delete(doc! { "active_id": i64::from(active_id) }, None)
            .await
            .map_err(repo_error)?
            .map(ActiveEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "Active".into(),
                id: active_id.to_string(),
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActiveDocument {
    pub active_id: i64,
    pub user_id: i64,
    pub security_id: String,
    pub bought_price: PriceDocument,
    pub count: i64,
}

impl TryFrom<ActiveDocument> for ActiveEntity {
    type Error = DomainError;

    fn try_from(value: ActiveDocument) -> Result<Self, Self::Error> {
        Ok(ActiveEntity {
            active_id: id_to_u32(value.active_id, "active_id")?,
            user_id: id_to_u32(value.user_id, "user_id")?,
            security_id: value.security_id,
            bought_price: Price::from(value.bought_price),
            count: id_to_u32(value.count, "count")?,
        })
    }
}

impl From<&ActiveEntity> for ActiveDocument {
    fn from(value: &ActiveEntity) -> Self {
        Self {
            active_id: i64::from(value.active_id),
            user_id: i64::from(value.user_id),
            security_id: value.security_id.clone(),
            bought_price: PriceDocument::from(&value.bought_price),
            count: i64::from(value.count),
        }
    }
}
