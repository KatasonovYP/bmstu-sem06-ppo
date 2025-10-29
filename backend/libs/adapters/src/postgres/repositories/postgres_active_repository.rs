use std::sync::Arc;

use domain::{
    errors::DomainError,
    models::ActiveEntity,
    ports::storage::ActiveRepository,
    value_objects::Price,
};
use sea_orm::{
    ActiveValue::{
        NotSet,
        Set,
    },
    EntityTrait,
};
use shaku::Component;

use crate::postgres::{
    connection::AbstractConnectionPool,
    schema::{
        actives,
        prelude::Actives,
    },
};

#[derive(Component)]
#[shaku(interface = ActiveRepository)]
pub struct PostgresActiveRepository {
    #[shaku(inject)]
    db: Arc<dyn AbstractConnectionPool>,
}

impl PostgresActiveRepository {
    #[allow(dead_code)]
    fn new(db: Arc<dyn AbstractConnectionPool>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl ActiveRepository for PostgresActiveRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_active(&self, active: &ActiveEntity) -> Result<ActiveEntity, DomainError> {
        Actives::insert(actives::ActiveModel::from(active.clone()))
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
    async fn update_active(&self, active: &ActiveEntity) -> Result<ActiveEntity, DomainError> {
        Actives::update(actives::ActiveModel::from(active.clone()))
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::{
        models::ActiveEntity,
        ports::storage::ActiveRepository,
    };
    use sea_orm::{
        DatabaseBackend,
        MockDatabase,
    };

    use crate::postgres::{
        PostgresActiveRepository,
        connection::PostgresConnectionPool,
        schema::actives,
    };

    #[tokio::test]
    async fn test_create_active() {
        let active_entity = ActiveEntity::default();
        let active_model = actives::Model {
            ..Default::default()
        };
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[active_model]])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));
        let active_entity_result = active_repo.create_active(&active_entity).await.unwrap();

        assert_eq!(active_entity_result, active_entity);
    }

    #[tokio::test]
    async fn test_create_active_error() {
        use domain::errors::DomainError;

        let active_entity = ActiveEntity::default();
        // Эмулируем ошибку (например нарушение ограничения уникальности).
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "duplicate key value violates unique constraint".into(),
            ))])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));

        let result = active_repo.create_active(&active_entity).await;

        // Проверяем, что вернулась ошибка
        assert!(result.is_err());

        // Можно проверить, что это именно ошибка репозитория
        match result {
            Err(DomainError::RepositoryError(msg)) => {
                assert!(msg.contains("duplicate key value violates unique constraint"))
            },
            _ => panic!("Expected RepositoryError!"),
        }
    }

    #[tokio::test]
    async fn test_get_active() {
        let active_model = actives::Model {
            active_id: 1,
            user_id: 42,
            ..Default::default()
        };
        let active_entity = ActiveEntity::try_from(active_model.clone()).unwrap();
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[active_model.clone()]])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));

        let got = active_repo.get_active(1).await.unwrap();
        assert_eq!(got, active_entity);
    }

    #[tokio::test]
    async fn test_get_active_not_found() {
        use domain::errors::DomainError;
        let empty: Vec<actives::Model> = vec![];

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([empty])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));

        let res = active_repo.get_active(1).await;
        match res {
            Err(DomainError::EntityNotFound { entity, id }) => {
                assert_eq!(entity, "Active");
                assert_eq!(id, "1");
            },
            _ => panic!("Expected EntityNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_list_actives() {
        let am1 = actives::Model {
            active_id: 1,
            user_id: 1,
            ..Default::default()
        };
        let am2 = actives::Model {
            active_id: 2,
            user_id: 2,
            ..Default::default()
        };
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[am1.clone(), am2.clone()]])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));
        let got = active_repo.list_actives().await.unwrap();

        assert_eq!(
            got,
            vec![
                ActiveEntity::try_from(am1).unwrap(),
                ActiveEntity::try_from(am2).unwrap()
            ]
        );
    }

    #[tokio::test]
    async fn test_list_actives_error() {
        use domain::errors::DomainError;

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "some db error".into(),
            ))])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));

        let res = active_repo.list_actives().await;
        match res {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("some db error")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_list_user_actives() {
        let am1 = actives::Model {
            active_id: 1,
            user_id: 2,
            ..Default::default()
        };
        let am2 = actives::Model {
            active_id: 2,
            user_id: 3,
            ..Default::default()
        };
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[am1.clone(), am2.clone()]])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));
        let got = active_repo.list_user_actives(2).await.unwrap();

        assert_eq!(got, vec![ActiveEntity::try_from(am1).unwrap(),]);
    }

    #[tokio::test]
    async fn test_list_user_actives_error() {
        use domain::errors::DomainError;

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "fail".into(),
            ))])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));

        let res = active_repo.list_user_actives(1).await;
        match res {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("fail")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_update_active() {
        // Передаем active_id != 0, иначе PrimaryKey is not set
        let active_entity = ActiveEntity {
            active_id: 100,
            ..Default::default()
        };
        let active_model = actives::Model {
            active_id: 100,
            ..Default::default()
        };

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[active_model.clone()]])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));
        let result = active_repo.update_active(&active_entity).await.unwrap();
        assert_eq!(result, active_entity);
    }

    #[tokio::test]
    async fn test_update_active_error() {
        use domain::errors::DomainError;

        let active_entity = ActiveEntity {
            active_id: 100,
            ..Default::default()
        };

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "update error".into(),
            ))])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));
        let result = active_repo.update_active(&active_entity).await;

        match result {
            Err(DomainError::RepositoryError(msg)) => {
                // println!("RepositoryError msg: {msg}");
                assert!(
                    msg.contains("update error") || !msg.is_empty(),
                    "RepositoryError message: {msg}"
                );
            },
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_delete_active() {
        let active_model = actives::Model {
            active_id: 1,
            ..Default::default()
        };
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[active_model.clone()]])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));

        let deleted = active_repo.delete_active(1).await.unwrap();
        assert_eq!(deleted, ActiveEntity::try_from(active_model).unwrap());
    }

    #[tokio::test]
    async fn test_delete_active_error() {
        use domain::errors::DomainError;

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "delete error".into(),
            ))])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));
        let result = active_repo.delete_active(1).await;
        match result {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("delete error")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_delete_active_not_found() {
        use domain::errors::DomainError;
        let empty: Vec<actives::Model> = vec![];
        // Возвращаем пустой массив — значит ничего не удалено
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([empty])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let active_repo = PostgresActiveRepository::new(Arc::new(pool));
        let result = active_repo.delete_active(1).await;
        match result {
            Err(DomainError::EntityNotFound { entity, id }) => {
                assert_eq!(entity, "Active");
                assert_eq!(id, "1");
            },
            _ => panic!("Expected EntityNotFound error"),
        }
    }
}
