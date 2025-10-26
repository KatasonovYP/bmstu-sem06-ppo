use std::sync::Arc;

use adapters::{
    di_domain_module::di_domain_module,
    postgres::{
        connection::{
            AbstractConnectionPool,
            PostgresConnectionPool,
        },
        schema::prelude::{
            Actives,
            Notifications,
            Sent,
            Users,
        },
    },
    settings::Settings,
};
use async_dropper::{
    AsyncDrop,
    AsyncDropper,
};
use async_trait::async_trait;
use domain::ports::storage::{
    ActiveRepository,
    NotificationRepository,
    SentRepository,
    UserRepository,
};
use sea_orm::EntityTrait;
use shaku::HasComponent;

#[derive(Default)]
struct DropTables {
    db: Arc<PostgresConnectionPool>,
}

impl DropTables {
    async fn default() -> Self {
        let settings = Settings::new("../../config/app.default.yaml").unwrap();
        let postgres_connection_pool = PostgresConnectionPool::new_connection_pool(
            settings.build_postgres_connection_string(),
        )
        .await
        .unwrap();
        let db = Arc::new(PostgresConnectionPool::new(postgres_connection_pool));
        Self { db }
    }
}

#[async_trait]
impl AsyncDrop for DropTables {
    async fn async_drop(&mut self) {
        Sent::delete_many()
            .exec(self.db.get_connection().await.as_ref())
            .await
            .unwrap();

        Notifications::delete_many()
            .exec(self.db.get_connection().await.as_ref())
            .await
            .unwrap();

        Actives::delete_many()
            .exec(self.db.get_connection().await.as_ref())
            .await
            .unwrap();

        Users::delete_many()
            .exec(self.db.get_connection().await.as_ref())
            .await
            .unwrap();
    }
}
pub struct TestManager {
    pub user_repo: Arc<dyn UserRepository>,
    pub active_repo: Arc<dyn ActiveRepository>,
    pub notification_repo: Arc<dyn NotificationRepository>,
    pub sent_repo: Arc<dyn SentRepository>,
    _dropper: AsyncDropper<DropTables>,
}

impl TestManager {
    pub async fn default() -> Self {
        let settings = Settings::new("../../config/app.default.yaml").unwrap();
        tracing::debug!("{settings:?}");
        let module = di_domain_module(settings).await;
        Self {
            user_repo: module.resolve(),
            active_repo: module.resolve(),
            notification_repo: module.resolve(),
            sent_repo: module.resolve(),
            _dropper: AsyncDropper::new(DropTables::default().await),
        }
    }
}
