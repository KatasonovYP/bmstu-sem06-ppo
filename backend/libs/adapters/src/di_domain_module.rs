use std::{
    fs::OpenOptions,
    str::FromStr,
    sync::Arc,
};

use domain::services::{
    ActiveService,
    LimitMonitorService,
    NotificationService,
    PriceCacheService,
    PriceOpsService,
    SentService,
    UserService,
};
use redis::aio::MultiplexedConnection;
use sea_orm::DatabaseConnection;
use secrecy::ExposeSecret;
use shaku::{
    HasComponent,
    Interface,
    module,
};
use teloxide::Bot;
use tokio::sync::Mutex;

use crate::{
    moex::{
        MoexExchangeRepository,
        MoexExchangeRepositoryParameters,
    },
    mongodb::{
        MongoActiveRepository,
        MongoConnectionPool,
        MongoConnectionPoolParameters,
        MongoNotificationRepository,
        MongoSentRepository,
        MongoUserRepository,
    },
    postgres::{
        PostgresActiveRepository,
        PostgresNotificationRepository,
        PostgresSentRepository,
        PostgresUserRepository,
        connection::{
            PostgresConnectionPool,
            PostgresConnectionPoolParameters,
        },
    },
    redis::{
        RedisPriceCacheRepository,
        RedisPriceCacheRepositoryParameters,
    },
    settings::Settings,
    telegram::{
        TelegramNotificationSender,
        TelegramNotificationSenderParameters,
    },
};

module! {
    pub PostgresDomainModule {
        components = [
            PostgresConnectionPool,
            PostgresUserRepository,
            PostgresActiveRepository,
            PostgresNotificationRepository,
            PostgresSentRepository,
            RedisPriceCacheRepository,
            TelegramNotificationSender,
            MoexExchangeRepository,
            PriceOpsService,
            UserService,
            ActiveService,
            NotificationService,
            SentService,
            LimitMonitorService,
            PriceCacheService,
        ],
        providers = []
    }
}

module! {
    pub MongoDomainModule {
        components = [
            MongoConnectionPool,
            MongoUserRepository,
            MongoActiveRepository,
            MongoNotificationRepository,
            MongoSentRepository,
            RedisPriceCacheRepository,
            TelegramNotificationSender,
            MoexExchangeRepository,
            PriceOpsService,
            UserService,
            ActiveService,
            NotificationService,
            SentService,
            LimitMonitorService,
            PriceCacheService,
        ],
        providers = []
    }
}

pub enum DomainModule {
    Postgres(PostgresDomainModule),
    Mongo(MongoDomainModule),
}

impl DomainModule {
    pub fn resolve<T>(&self) -> Arc<T>
    where
        T: Interface + ?Sized,
        PostgresDomainModule: HasComponent<T>,
        MongoDomainModule: HasComponent<T>,
    {
        match self {
            DomainModule::Postgres(module) => module.resolve(),
            DomainModule::Mongo(module) => module.resolve(),
        }
    }
}

pub struct BuildAppModule<'a> {
    settings: &'a Settings,
}

impl<'a> BuildAppModule<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }

    pub async fn build(&self) -> DomainModule {
        self.init_logger();
        tracing::debug!("app: start building");
        tracing::debug!("settings: {:?}", self.settings);
        if self.settings.use_mongo {
            tracing::info!("Initializing Mongo storage backend");
            let mongo_database = self.init_mongo_database().await;
            let redis_connection_pool = self.init_redis_connection_pool().await;
            let module = self
                .init_mongo_di_module(mongo_database, redis_connection_pool)
                .await;
            tracing::debug!("app: builded successfully");
            DomainModule::Mongo(module)
        } else {
            tracing::info!("Initializing Postgres storage backend");
            let postgres_connection_pool = self.init_postgres_connection_pool().await;
            let redis_connection_pool = self.init_redis_connection_pool().await;
            let module = self
                .init_postgres_di_module(postgres_connection_pool, redis_connection_pool)
                .await;
            tracing::debug!("app: builded successfully");
            DomainModule::Postgres(module)
        }
    }

    async fn init_postgres_connection_pool(&self) -> Arc<DatabaseConnection> {
        tracing::debug!("postgres: start init");
        let postgres_connection_pool = PostgresConnectionPool::new_connection_pool(
            self.settings
                .build_postgres_connection_string()
                .expose_secret(),
        )
        .await
        .unwrap();
        tracing::debug!("postgres: inited successfully");

        postgres_connection_pool
    }

    async fn init_mongo_database(&self) -> Arc<mongodb::Database> {
        tracing::debug!("mongo: start init");
        let database = MongoConnectionPool::connect(
            self.settings
                .build_mongo_connection_string()
                .expose_secret(),
            &self.settings.mongo_database,
        )
        .await
        .unwrap();
        tracing::debug!("mongo: inited successfully");

        database
    }

    async fn init_redis_connection_pool(&self) -> MultiplexedConnection {
        tracing::debug!("redis: start init");
        let redis_connection_pool =
            redis::Client::open(self.settings.redis_connection_string.expose_secret())
                .unwrap()
                .get_multiplexed_async_connection()
                .await
                .unwrap();
        tracing::debug!("redis: inited successfully");

        redis_connection_pool
    }

    async fn init_postgres_di_module(
        &self,
        postgres_connection_pool: Arc<DatabaseConnection>,
        redis_connection_pool: MultiplexedConnection,
    ) -> PostgresDomainModule {
        tracing::debug!("postgres di module: start init");
        let module = PostgresDomainModule::builder()
            .with_component_parameters::<PostgresConnectionPool>(PostgresConnectionPoolParameters {
                connection: postgres_connection_pool,
            })
            .with_component_parameters::<RedisPriceCacheRepository>(
                RedisPriceCacheRepositoryParameters {
                    connection: Arc::new(Mutex::new(redis_connection_pool)),
                },
            )
            .with_component_parameters::<TelegramNotificationSender>(
                TelegramNotificationSenderParameters {
                    bot: Arc::new(Bot::new(self.settings.telegram_bot_token.expose_secret())),
                },
            )
            .with_component_parameters::<MoexExchangeRepository>(MoexExchangeRepositoryParameters {
                client: reqwest::Client::new(),
                base_url: self.settings.moex_base_url.clone(),
            })
            .build();

        tracing::debug!("postgres di module: inited successfully");

        module
    }

    async fn init_mongo_di_module(
        &self,
        mongo_database: Arc<mongodb::Database>,
        redis_connection_pool: MultiplexedConnection,
    ) -> MongoDomainModule {
        tracing::debug!("mongo di module: start init");
        let module = MongoDomainModule::builder()
            .with_component_parameters::<MongoConnectionPool>(MongoConnectionPoolParameters {
                database: mongo_database,
            })
            .with_component_parameters::<RedisPriceCacheRepository>(
                RedisPriceCacheRepositoryParameters {
                    connection: Arc::new(Mutex::new(redis_connection_pool)),
                },
            )
            .with_component_parameters::<TelegramNotificationSender>(
                TelegramNotificationSenderParameters {
                    bot: Arc::new(Bot::new(self.settings.telegram_bot_token.expose_secret())),
                },
            )
            .with_component_parameters::<MoexExchangeRepository>(MoexExchangeRepositoryParameters {
                client: reqwest::Client::new(),
                base_url: self.settings.moex_base_url.clone(),
            })
            .build();

        tracing::debug!("mongo di module: inited successfully");

        module
    }

    fn init_logger(&self) {
        if self.settings.log_filepath != "stdout" {
            let log_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(self.settings.log_filepath.clone())
                .unwrap();

            tracing_subscriber::fmt()
                .json()
                .with_writer(log_file)
                .with_max_level(tracing::Level::from_str(&self.settings.log_level).unwrap())
                .with_level(true)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .init();
        } else {
            tracing_subscriber::fmt()
                .pretty()
                .with_max_level(tracing::Level::from_str(&self.settings.log_level).unwrap())
                .with_level(true)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .init();
        }

        tracing::debug!("logger: inited successfully");
    }
}
