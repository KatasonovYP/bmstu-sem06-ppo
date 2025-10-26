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
    async fn create_sent(&self, sent: &SentEntity) -> Result<SentEntity, DomainError> {
        Sent::insert(sent::ActiveModel::from(sent.clone()))
            .exec_with_returning(self.db.get_connection().await.as_ref())
            .await
            .map(SentEntity::from)
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn create_sent_now(&self, notification_id: u32) -> Result<SentEntity, DomainError> {
        self.create_sent(&SentEntity {
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
    async fn update_sent(&self, sent: &SentEntity) -> Result<SentEntity, DomainError> {
        Sent::update(sent::ActiveModel::from(sent.clone()))
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

#[cfg(not(feature = "production"))]
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDateTime;
    use domain::{
        errors::DomainError,
        models::SentEntity,
        ports::storage::SentRepository,
    };
    use sea_orm::{
        DatabaseBackend,
        MockDatabase,
    };

    use crate::postgres::{
        PostgresSentRepository,
        connection::PostgresConnectionPool,
        schema::sent,
    };

    fn sample_datetime() -> NaiveDateTime {
        NaiveDateTime::parse_from_str("2024-01-02 11:22:33", "%Y-%m-%d %H:%M:%S").unwrap()
    }

    fn model(notification_id: i32) -> sent::Model {
        sent::Model {
            notification_id,
            last_message_time: chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(
                sample_datetime(),
                chrono::FixedOffset::east_opt(0).unwrap(),
            ),
        }
    }

    fn entity(notification_id: u32) -> SentEntity {
        SentEntity {
            notification_id,
            last_message_time: sample_datetime(),
        }
    }

    #[tokio::test]
    async fn test_create_sent() {
        let sent_entity = entity(42);
        let sent_model = model(42);

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[sent_model.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let result = repo.create_sent(&sent_entity).await.unwrap();

        assert_eq!(result, entity(42));
    }

    #[tokio::test]
    async fn test_create_sent_error() {
        let sent_entity = entity(1);

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "duplicate".into(),
            ))])
            .into_connection();

        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let result = repo.create_sent(&sent_entity).await;
        assert!(result.is_err());

        match result {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("duplicate")),
            _ => panic!("Expected RepositoryError!"),
        }
    }

    #[tokio::test]
    async fn test_create_sent_now() {
        let sent_model = model(55);

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[sent_model.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let result = repo.create_sent_now(55).await.unwrap();
        assert_eq!(result.notification_id, 55);
    }

    #[tokio::test]
    async fn test_get_sent() {
        let sent_model = model(100);
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[sent_model.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let got = repo.get_sent(100).await.unwrap();

        assert_eq!(got, entity(100));
    }

    #[tokio::test]
    async fn test_get_sent_not_found() {
        let empty: Vec<sent::Model> = vec![];
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([empty])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let res = repo.get_sent(777).await;
        match res {
            Err(DomainError::EntityNotFound { entity, id }) => {
                assert_eq!(entity, "Sent");
                assert_eq!(id, "777");
            },
            _ => panic!("Expected EntityNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_list_sent() {
        let sm1 = model(1);
        let sm2 = model(2);

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[sm1.clone(), sm2.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let got = repo.list_sent().await.unwrap();
        assert_eq!(got, vec![SentEntity::from(sm1), SentEntity::from(sm2)]);
    }

    #[tokio::test]
    async fn test_list_sent_error() {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "err".into(),
            ))])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let res = repo.list_sent().await;
        match res {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("err")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_update_sent() {
        let sent_entity = entity(16);
        let sent_model = model(16);

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[sent_model.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let result = repo.update_sent(&sent_entity).await.unwrap();

        assert_eq!(result, SentEntity::from(sent_model));
    }

    // #[tokio::test]
    // async fn test_update_sent_error() {
    //     let sent_entity = entity(16);

    //     let connection = MockDatabase::new(DatabaseBackend::Postgres)
    //         .append_exec_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
    //             "update err".into(),
    //         ))])
    //         .into_connection();
    //     let pool = PostgresConnectionPool::new(Arc::new(connection));
    //     let repo = PostgresSentRepository { db: Arc::new(pool) };

    //     let result = repo.update_sent(sent_entity).await;
    //     match result {
    //         Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("update err")),
    //         _ => panic!("Expected RepositoryError"),
    //     }
    // }

    #[tokio::test]
    async fn test_delete_sent() {
        let sent_model = model(21);

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[sent_model.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let result = repo.delete_sent(21).await.unwrap();

        assert_eq!(result, SentEntity::from(sent_model));
    }

    #[tokio::test]
    async fn test_delete_sent_error() {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "del err".into(),
            ))])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let result = repo.delete_sent(55).await;
        match result {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("del err")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_delete_sent_not_found() {
        let empty: Vec<sent::Model> = vec![];
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([empty])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresSentRepository { db: Arc::new(pool) };

        let result = repo.delete_sent(987).await;
        match result {
            Err(DomainError::EntityNotFound { entity, id }) => {
                assert!(entity.contains("SentEntity")); // подробное имя типа из std::any
                assert_eq!(id, "987");
            },
            _ => panic!("Expected EntityNotFound error"),
        }
    }
}
