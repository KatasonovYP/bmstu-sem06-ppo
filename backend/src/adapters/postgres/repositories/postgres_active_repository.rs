use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{
    ActiveValue::{
        NotSet,
        Set,
    },
    EntityTrait,
};
use shaku::Component;

use crate::{
    adapters::postgres::{
        connection::AbstractConnectionPool,
        schema::{
            actives,
            prelude::Actives,
        },
    },
    domain::{
        errors::DomainError,
        models::ActiveEntity,
        ports::storage::ActiveRepository,
        value_objects::Price,
    },
};

#[derive(Component)]
#[shaku(interface = ActiveRepository)]
pub struct PostgresActiveRepository {
    #[shaku(inject)]
    db: Arc<dyn AbstractConnectionPool>,
}

#[async_trait]
impl ActiveRepository for PostgresActiveRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_active(&self, active: ActiveEntity) -> Result<ActiveEntity, DomainError> {
        Actives::insert(actives::ActiveModel::from(active))
            .exec_with_returning(self.db.get_connection().await.as_ref())
            .await
            .map(ActiveEntity::try_from)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError> {
        Actives::find_by_id(active_id as i32)
            .one(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .map(ActiveEntity::try_from)
            .ok_or(DomainError::EntityNotFound {
                entity: "Active".into(),
                id: active_id.to_string(),
            })?
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_actives(&self) -> Result<Vec<ActiveEntity>, DomainError> {
        Actives::find()
            .all(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .into_iter()
            .map(ActiveEntity::try_from)
            .collect()
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn list_user_actives(&self, user_id: u32) -> Result<Vec<ActiveEntity>, DomainError> {
        Ok(self
            .list_actives()
            .await?
            .into_iter()
            .filter(|x| x.user_id == user_id)
            .collect())
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn update_active(&self, active: ActiveEntity) -> Result<ActiveEntity, DomainError> {
        Actives::update(actives::ActiveModel::from(active))
            .exec(self.db.get_connection().await.as_ref())
            .await
            .map(ActiveEntity::try_from)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn delete_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError> {
        Actives::delete_by_id(active_id as i32)
            .exec_with_returning(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .into_iter()
            .next()
            .map(ActiveEntity::try_from)
            .ok_or(DomainError::EntityNotFound {
                entity: "Active".into(),
                id: active_id.to_string(),
            })?
    }
}

impl TryFrom<actives::Model> for ActiveEntity {
    type Error = DomainError;

    fn try_from(orm_active: actives::Model) -> Result<Self, Self::Error> {
        Ok(ActiveEntity {
            active_id: orm_active.active_id as u32,
            user_id: orm_active.user_id as u32,
            security_id: orm_active.security_id,
            bought_price: Price::new(orm_active.bought_price, orm_active.currency),
            count: orm_active.count as u32,
        })
    }
}

impl From<ActiveEntity> for actives::ActiveModel {
    fn from(entity: ActiveEntity) -> Self {
        let active_id = match entity.active_id {
            0 => NotSet,
            id => Set(id as i32),
        };
        Self {
            active_id,
            user_id: Set(entity.user_id as i32),
            security_id: Set(entity.security_id),
            bought_price: Set(entity.bought_price.amount),
            currency: Set(entity.bought_price.currency.value),
            count: Set(entity.count as i32),
        }
    }
}
