use std::sync::Arc;

use domain::{
    errors::DomainError,
    models::UserEntity,
    ports::storage::UserRepository,
    value_objects::Username,
};
use sea_orm::{
    ActiveValue::{
        NotSet,
        Set,
    },
    ColumnTrait,
    EntityTrait,
    QueryFilter,
};
use shaku::Component;

use crate::postgres::{
    connection::AbstractConnectionPool,
    schema::{
        prelude::Users,
        users,
    },
};

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct PostgresUserRepository {
    #[shaku(inject)]
    db: Arc<dyn AbstractConnectionPool>,
}

#[async_trait::async_trait]
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
    async fn get_user_by_tg_id(&self, tg_id: i64) -> Result<UserEntity, DomainError> {
        Users::find()
            .filter(users::Column::TgId.eq(tg_id))
            .one(self.db.get_connection().await.as_ref())
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?
            .map(UserEntity::try_from)
            .transpose()?
            .ok_or(DomainError::EntityNotFound {
                entity: std::any::type_name::<UserEntity>().into(),
                id: tg_id.to_string(),
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
            id => Set(id as i32),
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

#[cfg(not(feature = "production"))]
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::{
        errors::DomainError,
        models::UserEntity,
        ports::storage::UserRepository,
        value_objects::Username,
    };
    use sea_orm::{DatabaseBackend, MockDatabase};

    use crate::postgres::{
        PostgresUserRepository,
        connection::PostgresConnectionPool,
        schema::users,
    };

    fn user_model(id: i32, tg_id: i64, name: &str) -> users::Model {
        users::Model {
            user_id: id,
            tg_id,
            chat_id: tg_id + 10000,
            username: name.to_owned(),
            first_name: Some("A".to_owned()),
            second_name: Some("B".to_owned()),
        }
    }
    fn user_entity(id: u32, tg_id: i64, name: &str) -> UserEntity {
        UserEntity {
            user_id: id,
            tg_id,
            chat_id: tg_id + 10000,
            username: Username::new(name.to_owned()).unwrap(),
            first_name: Some("A".to_owned()),
            second_name: Some("B".to_owned()),
        }
    }

    // #[tokio::test]
    // async fn test_create_user() {
    //     let entity = user_entity(0, 42, "sam");
    //     let model = user_model(1, 42, "sam");

    //     let connection = MockDatabase::new(DatabaseBackend::Postgres)
    //         .append_query_results([[model.clone()]])
    //         .into_connection();
    //     let pool = PostgresConnectionPool::new(Arc::new(connection));
    //     let repo = PostgresUserRepository { db: Arc::new(pool) };

    //     let user = repo.create_user(entity.clone()).await.unwrap();
    //     assert_eq!(user, user_entity(1, 42, "sam"));
    // }

    // #[tokio::test]
    // async fn test_create_user_error() {
    //     let entity = user_entity(0, 0, "fail");

    //     let connection = MockDatabase::new(DatabaseBackend::Postgres)
    //         .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
    //             "duplicate".into(),
    //         ))])
    //         .into_connection();
    //     let pool = PostgresConnectionPool::new(Arc::new(connection));
    //     let repo = PostgresUserRepository { db: Arc::new(pool) };

    //     let res = repo.create_user(entity).await;
    //     assert!(res.is_err());
    //     match res {
    //         Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("duplicate")),
    //         _ => panic!("Expected RepositoryError"),
    //     }
    // }

    // #[tokio::test]
    // async fn test_create_user_tryfrom_fail() {
    //     // username пустой, Username::new(err) => Err
    //     let model = users::Model {
    //         user_id: 1,
    //         tg_id: 111,
    //         chat_id: 222,
    //         username: "".to_string(),
    //         first_name: Some("a".to_owned()),
    //         second_name: Some("b".to_owned()),
    //     };
    //     let connection = MockDatabase::new(DatabaseBackend::Postgres)
    //         .append_query_results([[model]])
    //         .into_connection();
    //     let pool = PostgresConnectionPool::new(Arc::new(connection));
    //     let repo = PostgresUserRepository { db: Arc::new(pool) };
    //     // Преобразование упадёт во второй map_err
    //     let result = repo.create_user(user_entity(0, 111, "")).await;
    //     match result {
    //         Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("convert")),
    //         _ => panic!("Expected RepositoryError"),
    //     }
    // }

    // #[tokio::test]
    // async fn test_get_user() {
    //     let model = user_model(5, 8001, "bob");
    //     let entity = user_entity(5, 8001, "bob");

    //     let connection = MockDatabase::new(DatabaseBackend::Postgres)
    //         .append_query_results([[model.clone()]])
    //         .into_connection();
    //     let pool = PostgresConnectionPool::new(Arc::new(connection));
    //     let repo = PostgresUserRepository { db: Arc::new(pool) };

    //     let got = repo.get_user(5).await.unwrap();
    //     assert_eq!(got, entity);
    // }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let empty: Vec<users::Model> = vec![];
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([empty])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresUserRepository { db: Arc::new(pool) };

        let res = repo.get_user(1234).await;
        match res {
            Err(DomainError::EntityNotFound { entity, id }) => {
                assert!(entity.contains("UserEntity"));
                assert_eq!(id, "1234");
            },
            _ => panic!("Expected EntityNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_user_error() {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "fail".into(),
            ))])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresUserRepository { db: Arc::new(pool) };

        let res = repo.get_user(1).await;
        match res {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("fail")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_get_user_by_tg_id() {
        let model = user_model(17, 9955, "alice");
        let entity = user_entity(17, 9955, "alice");

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[model.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresUserRepository { db: Arc::new(pool) };

        let got = repo.get_user_by_tg_id(9955).await.unwrap();
        assert_eq!(got, entity);
    }

    #[tokio::test]
    async fn test_get_user_by_tg_id_not_found() {
        let empty: Vec<users::Model> = vec![];
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([empty])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresUserRepository { db: Arc::new(pool) };

        let res = repo.get_user_by_tg_id(1111).await;
        match res {
            Err(DomainError::EntityNotFound { entity, id }) => {
                assert!(entity.contains("UserEntity"));
                assert_eq!(id, "1111");
            },
            _ => panic!("Expected EntityNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_user_by_tg_id_error() {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "db err".into(),
            ))])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresUserRepository { db: Arc::new(pool) };

        let res = repo.get_user_by_tg_id(7).await;
        match res {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("db err")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    // #[tokio::test]
    // async fn test_list_users() {
    //     let m1 = user_model(1, 10, "u1");
    //     let m2 = user_model(2, 20, "u2");

    //     let connection = MockDatabase::new(DatabaseBackend::Postgres)
    //         .append_query_results([[m1.clone(), m2.clone()]])
    //         .into_connection();
    //     let pool = PostgresConnectionPool::new(Arc::new(connection));
    //     let repo = PostgresUserRepository { db: Arc::new(pool) };

    //     let got = repo.list_users().await.unwrap();
    //     assert_eq!(
    //         got,
    //         vec![
    //             UserEntity::try_from(m1).unwrap(),
    //             UserEntity::try_from(m2).unwrap()
    //         ]
    //     );
    // }

    #[tokio::test]
    async fn test_list_users_error() {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "problem".into(),
            ))])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresUserRepository { db: Arc::new(pool) };

        let res = repo.list_users().await;
        match res {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("problem")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_update_user() {
        let entity = user_entity(5, 900, "testo");
        let model = user_model(5, 900, "testo");

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[model.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresUserRepository { db: Arc::new(pool) };

        let user = repo.update_user(entity.clone()).await.unwrap();
        assert_eq!(user, UserEntity::try_from(model).unwrap());
    }

    // #[tokio::test]
    // async fn test_update_user_error() {
    //     let entity = user_entity(5, 900, "testo");

    //     let connection = MockDatabase::new(DatabaseBackend::Postgres)
    //         .append_exec_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
    //             "update failed".into(),
    //         ))])
    //         .into_connection();
    //     let pool = PostgresConnectionPool::new(Arc::new(connection));
    //     let repo = PostgresUserRepository { db: Arc::new(pool) };

    //     let result = repo.update_user(entity).await;
    //     match result {
    //         Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("update failed")),
    //         _ => panic!("Expected RepositoryError"),
    //     }
    // }

    #[tokio::test]
    async fn test_delete_user() {
        let model = user_model(42, 997, "denis");

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[model.clone()]])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresUserRepository { db: Arc::new(pool) };

        let user = repo.delete_user(42).await.unwrap();
        assert_eq!(user, UserEntity::try_from(model).unwrap());
    }

    #[tokio::test]
    async fn test_delete_user_error() {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors([sea_orm::DbErr::Exec(sea_orm::RuntimeErr::Internal(
                "delete fail".into(),
            ))])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresUserRepository { db: Arc::new(pool) };

        let result = repo.delete_user(96).await;

        match result {
            Err(DomainError::RepositoryError(msg)) => assert!(msg.contains("delete fail")),
            _ => panic!("Expected RepositoryError"),
        }
    }

    #[tokio::test]
    async fn test_delete_user_not_found() {
        let empty: Vec<users::Model> = vec![];
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([empty])
            .into_connection();
        let pool = PostgresConnectionPool::new(Arc::new(connection));
        let repo = PostgresUserRepository { db: Arc::new(pool) };

        let result = repo.delete_user(444).await;
        match result {
            Err(DomainError::EntityNotFound { entity, id }) => {
                assert!(entity.contains("UserEntity"));
                assert_eq!(id, "444");
            },
            _ => panic!("Expected EntityNotFound error"),
        }
    }
}
