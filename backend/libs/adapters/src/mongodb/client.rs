use std::sync::Arc;

use mongodb::{
    Client,
    Database,
    options::ClientOptions,
};
use shaku::{
    Component,
    Interface,
};

#[async_trait::async_trait]
pub trait MongoConnection: Interface + Send + Sync + 'static {
    async fn database(&self) -> Arc<Database>;
}

#[derive(Component)]
#[shaku(interface = MongoConnection)]
pub struct MongoConnectionPool {
    database: Arc<Database>,
}

impl MongoConnectionPool {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub async fn connect(
        connection_string: &str,
        database_name: &str,
    ) -> Result<Arc<Database>, mongodb::error::Error> {
        let mut client_options = ClientOptions::parse(connection_string).await?;
        client_options.app_name = Some("ppo-backend".into());
        let client = Client::with_options(client_options)?;
        let database = client.database(database_name);
        Ok(Arc::new(database))
    }
}

#[async_trait::async_trait]
impl MongoConnection for MongoConnectionPool {
    async fn database(&self) -> Arc<Database> {
        Arc::clone(&self.database)
    }
}
