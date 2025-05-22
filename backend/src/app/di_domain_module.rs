use std::{
    env,
    sync::Arc,
};

use dotenvy::dotenv;
use shaku::module;
use teloxide::Bot;
use tokio::sync::Mutex;

use crate::{
    adapters::{
        moex::{
            MoexExchangeRepository,
            MoexExchangeRepositoryParameters,
        },
        postgres::{
            connection::{
                PostgresConnectionPool,
                PostgresConnectionPoolParameters,
            },
            PostgresActiveRepository,
            PostgresNotificationRepository,
            PostgresSentRepository,
            PostgresUserRepository,
        },
        redis::{
            RedisPriceCacheRepository,
            RedisPriceCacheRepositoryParameters,
        },
        telegram::{
            TelegramNotificationSender,
            TelegramNotificationSenderParameters,
        },
    },
    domain::{
        config::AppConfig,
        services::{
            ActiveService,
            LimitMonitorService,
            NotificationService,
            PriceCacheService,
            PriceOpsService,
            SentService,
            UserService,
        },
    },
};

module! {
    pub DomainModule {
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

pub async fn di_domain_module() -> DomainModule {
    dotenv().ok();

    let config = AppConfig {
        connection_string: env::var("DATABASE_URL").expect("DATABASE_URL should be set"),
        log_format: env::var("LOG_FORMAT").unwrap(),
    };
    tracing_subscriber::fmt()
        // .json()
        .pretty()
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_max_level(tracing::Level::INFO)
        .with_level(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    tracing::info!("logger inited successfully");

    let postgres_connection = PostgresConnectionPool::new_connection_pool(Arc::new(config))
        .await
        .unwrap();

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let redis_connection = client.get_multiplexed_async_connection().await.unwrap();

    tracing::info!("db connection inited successfully");

    DomainModule::builder()
        .with_component_parameters::<PostgresConnectionPool>(PostgresConnectionPoolParameters {
            connection: postgres_connection,
        })
        .with_component_parameters::<RedisPriceCacheRepository>(
            RedisPriceCacheRepositoryParameters {
                connection: Arc::new(Mutex::new(redis_connection)),
            },
        )
        .with_component_parameters::<TelegramNotificationSender>(
            TelegramNotificationSenderParameters {
                bot: Arc::new(Bot::from_env()),
            },
        )
        .with_component_parameters::<MoexExchangeRepository>(MoexExchangeRepositoryParameters {
            client: reqwest::Client::new(),
            base_url: "https://iss.moex.com".to_string(),
        })
        .build()
}
