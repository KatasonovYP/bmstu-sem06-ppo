use crate::app::adapters::db::orm_models;
use crate::app::adapters::db::schema::actives;
use crate::domain::errors::DomainError;
use crate::domain::models::ActiveEntity;
use crate::ports::outbound::db::ActiveRepository;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct PostgresActiveRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresActiveRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ActiveRepository for PostgresActiveRepository {
    async fn get_active(&self, active_id: u32) -> Result<ActiveEntity, DomainError> {
        let connection = &mut self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        actives::table
            .filter(actives::active_id.eq(active_id as i32))
            .select(orm_models::OrmSelectActive::as_select())
            .first(connection)
            .optional()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .map(ActiveEntity::from)
            .ok_or(DomainError::RepositoryError("Active not found".to_string()))
            
    }

    async fn create_active(&self, active: ActiveEntity) -> Result<ActiveEntity, DomainError> {
        let connection = &mut self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let active_copy = active.clone();
        let active_orm = orm_models::OrmInsertActive::from(active);
        diesel::insert_into(actives::table)
            .values(active_orm)
            .execute(connection)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
            .map(|_| active_copy)
    }
}
