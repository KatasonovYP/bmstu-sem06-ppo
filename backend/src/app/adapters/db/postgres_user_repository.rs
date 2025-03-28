use crate::app::adapters::db::orm_models;
use crate::app::adapters::db::schema::users;
use crate::domain::errors::DomainError;
use crate::domain::models::UserEntity;
use crate::ports::outbound::db::UserRepository;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct PostgresUserRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresUserRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn get_user(&self, tg_id: u32) -> Result<UserEntity, DomainError> {
        let connection = &mut self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        users::table
            .select(orm_models::OrmSelectUser::as_select())
            .filter(users::tg_id.eq(tg_id as i32))
            .first(connection)
            .optional()
            .inspect(|x| tracing::debug!("{x:?}"))
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .and_then(|orm_user| UserEntity::try_from(orm_user).ok())
            .ok_or(DomainError::RepositoryError("User not found".to_string()))
    }

    async fn create_user(&self, user: UserEntity) -> Result<UserEntity, DomainError> {
        let connection = &mut self
            .pool
            .get()
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let user_copy = user.clone();
        let user_orm = orm_models::OrmInsertUser::from(user);
        let user = diesel::insert_into(users::table)
            .values(user_orm)
            .execute(connection)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
            .map(|_| user_copy);
        
        return user;
    }
}
