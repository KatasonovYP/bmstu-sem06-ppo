use std::sync::Arc;

use adapters::{
    di_domain_module::BuildAppModule,
    settings::Settings,
};
use domain::ports::storage::{
    ActiveRepository,
    NotificationRepository,
    SentRepository,
    UserRepository,
};
use migration::{
    Migrator,
    MigratorTrait,
};
use sea_orm::{
    ConnectOptions,
    Database,
};
use secrecy::ExposeSecret;

pub struct TestManager {
    pub user_repo: Arc<dyn UserRepository>,
    pub active_repo: Arc<dyn ActiveRepository>,
    pub notification_repo: Arc<dyn NotificationRepository>,
    pub sent_repo: Arc<dyn SentRepository>,
    pub settings: Settings,
}

impl TestManager {
    pub async fn default() -> Self {
        let settings = Settings::new("../../config/app.default.yaml").unwrap();
        
        tracing::debug!("{settings:?}");
        let connect_options =
            ConnectOptions::new(settings.build_postgres_connection_string().expose_secret())
                .to_owned();
        let db = Database::connect(connect_options).await.unwrap();
        Migrator::fresh(&db).await.unwrap();
        let module = BuildAppModule::new(&settings).build().await;
        Self {
            user_repo: module.resolve(),
            active_repo: module.resolve(),
            notification_repo: module.resolve(),
            sent_repo: module.resolve(),
            settings,
        }
    }
}
