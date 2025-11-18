use std::sync::Arc;

use domain::{
    errors::DomainError,
    models::UserEntity,
    ports::storage::UserRepository,
    value_objects::Username,
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
        collect_cursor,
        id_to_u32,
        next_sequence,
        repo_error,
    },
};

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct MongoUserRepository {
    #[shaku(inject)]
    connection: Arc<dyn MongoConnection>,
}

impl MongoUserRepository {
    const COLLECTION: &'static str = "users";

    async fn collection(&self) -> mongodb::Collection<UserDocument> {
        self.connection
            .database()
            .await
            .collection(Self::COLLECTION)
    }
}

#[async_trait::async_trait]
impl UserRepository for MongoUserRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_user(&self, user: &UserEntity) -> Result<UserEntity, DomainError> {
        let db = self.connection.database().await;
        let collection = db.collection::<UserDocument>(Self::COLLECTION);
        let mut entity = user.clone();
        if entity.user_id == 0 {
            entity.user_id = next_sequence(db.as_ref(), "users").await?;
        }
        let document = UserDocument::from(&entity);
        collection
            .insert_one(&document, None)
            .await
            .map_err(repo_error)?;
        Ok(entity)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_user(&self, user_id: u32) -> Result<UserEntity, DomainError> {
        self.collection()
            .await
            .find_one(doc! { "user_id": i64::from(user_id) }, None)
            .await
            .map_err(repo_error)?
            .map(UserEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "User".into(),
                id: user_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_user_by_tg_id(&self, tg_id: i64) -> Result<UserEntity, DomainError> {
        self.collection()
            .await
            .find_one(doc! { "tg_id": tg_id }, None)
            .await
            .map_err(repo_error)?
            .map(UserEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "User".into(),
                id: tg_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_users(&self) -> Result<Vec<UserEntity>, DomainError> {
        let cursor = self
            .collection()
            .await
            .find(None, None)
            .await
            .map_err(repo_error)?;
        Ok(collect_cursor(cursor)
            .await?
            .into_iter()
            .map(UserEntity::try_from)
            .collect::<Result<Vec<_>, _>>()?)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn update_user(&self, user: &UserEntity) -> Result<UserEntity, DomainError> {
        let options = FindOneAndReplaceOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();
        self.collection()
            .await
            .find_one_and_replace(
                doc! { "user_id": i64::from(user.user_id) },
                UserDocument::from(user),
                options,
            )
            .await
            .map_err(repo_error)?
            .map(UserEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "User".into(),
                id: user.user_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn delete_user(&self, user_id: u32) -> Result<UserEntity, DomainError> {
        self.collection()
            .await
            .find_one_and_delete(doc! { "user_id": i64::from(user_id) }, None)
            .await
            .map_err(repo_error)?
            .map(UserEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: "User".into(),
                id: user_id.to_string(),
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserDocument {
    pub user_id: i64,
    pub tg_id: i64,
    pub chat_id: i64,
    pub username: String,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
}

impl UserDocument {
    fn to_entity(self) -> Result<UserEntity, DomainError> {
        let username = Username::new(&self.username)?;
        Ok(UserEntity {
            user_id: id_to_u32(self.user_id, "user_id")?,
            tg_id: self.tg_id,
            chat_id: self.chat_id,
            username,
            first_name: self.first_name,
            second_name: self.second_name,
        })
    }
}

impl TryFrom<UserDocument> for UserEntity {
    type Error = DomainError;

    fn try_from(value: UserDocument) -> Result<Self, Self::Error> {
        value.to_entity()
    }
}

impl From<&UserEntity> for UserDocument {
    fn from(value: &UserEntity) -> Self {
        Self {
            user_id: i64::from(value.user_id),
            tg_id: value.tg_id,
            chat_id: value.chat_id,
            username: value.username.value.clone(),
            first_name: value.first_name.clone(),
            second_name: value.second_name.clone(),
        }
    }
}
