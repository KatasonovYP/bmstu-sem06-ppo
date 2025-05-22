use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait,
};
use shaku::Component;

use crate::{
    adapters::postgres::{
        connection::AbstractConnectionPool,
        schema::{
            prelude::Users,
            users,
        },
    },
    domain::{
        errors::DomainError,
        models::UserEntity,
        ports::storage::UserRepository,
        value_objects::Username,
    },
};

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct PostgresUserRepository {
    #[shaku(inject)]
    db: Arc<dyn AbstractConnectionPool>,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_user(&self, user: UserEntity) -> Result<UserEntity, DomainError> {
        Users::insert(users::ActiveModel::from(user))
            .exec_with_returning(self.db.get_connection().await.as_ref())
            .await
            .map(UserEntity::try_from)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .map_err(|_| DomainError::RepositoryError("Failed to convert user entity".to_string()))
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_user(&self, user_id: u32) -> Result<UserEntity, DomainError> {
        Users::find_by_id(user_id as i32)
            .one(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .map(UserEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: std::any::type_name::<UserEntity>().into(),
                id: user_id.to_string(),
            })
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_users(&self) -> Result<Vec<UserEntity>, DomainError> {
        Users::find()
            .all(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .into_iter()
            .map(UserEntity::try_from)
            .collect()
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn update_user(&self, user: UserEntity) -> Result<UserEntity, DomainError> {
        Users::update(users::ActiveModel::from(user))
            .exec(self.db.get_connection().await.as_ref())
            .await
            .map(UserEntity::try_from)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn delete_user(&self, user_id: u32) -> Result<UserEntity, DomainError> {
        Users::delete_by_id(user_id as i32)
            .exec_with_returning(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .into_iter()
            .next()
            .map(UserEntity::try_from)
            .ok_or(DomainError::EntityNotFound {
                entity: std::any::type_name::<UserEntity>().into(),
                id: user_id.to_string(),
            })?
    }
}

impl TryFrom<users::Model> for UserEntity {
    type Error = DomainError;

    fn try_from(orm_user: users::Model) -> Result<Self, Self::Error> {
        let username = Username::new(orm_user.username)?;
        Ok(UserEntity {
            user_id: orm_user.user_id as u32,
            tg_id: orm_user.tg_id,
            chat_id: orm_user.chat_id,
            username,
            first_name: orm_user.first_name,
            second_name: orm_user.second_name,
        })
    }
}

impl From<UserEntity> for users::ActiveModel {
    fn from(entity: UserEntity) -> Self {
        let user_id = match entity.user_id {
            0 => NotSet,
            id => Set(id as i32)
        };
        Self {
            user_id,
            tg_id: Set(entity.tg_id),
            chat_id: Set(entity.chat_id),
            username: Set(entity.username.value),
            first_name: Set(entity.first_name),
            second_name: Set(entity.second_name),
        }
    }
}
