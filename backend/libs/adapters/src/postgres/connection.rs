use std::{
    sync::Arc,
    time::Duration,
};

use sea_orm::{
    ConnectOptions,
    Database,
    DatabaseConnection,
};
use shaku::{
    Component,
    Interface,
};

#[async_trait::async_trait]
pub trait AbstractConnectionPool: Interface + Send + Sync + 'static {
    async fn get_connection(&self) -> Arc<DatabaseConnection>;
}

#[derive(Component, Default)]
#[shaku(interface = AbstractConnectionPool)]
pub struct PostgresConnectionPool {
    #[shaku(default)]
    connection: Arc<DatabaseConnection>,
}

impl PostgresConnectionPool {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn new_connection_pool(
        connection_string: &str,
    ) -> Result<Arc<DatabaseConnection>, sea_orm::DbErr> {
        let mut opt = ConnectOptions::new(connection_string);

        opt.max_connections(100)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Trace)
            .set_schema_search_path("public");

        let connection = Database::connect(opt).await?;

        Ok(Arc::new(connection))
    }
}

#[async_trait::async_trait]
impl AbstractConnectionPool for PostgresConnectionPool {
    async fn get_connection(&self) -> Arc<DatabaseConnection> {
        Arc::clone(&self.connection)
    }
}
