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
use shaku::module;
use teloxide::Bot;
use tokio::sync::Mutex;

use crate::{
    moex::{
        MoexExchangeRepository,
        MoexExchangeRepositoryParameters,
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

pub async fn di_domain_module(settings: Settings) -> DomainModule {
    if settings.log_filepath != "stdout" {
        let log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(settings.log_filepath.clone())
            .unwrap();

        tracing_subscriber::fmt()
            .json()
            .with_writer(log_file)
            .with_max_level(tracing::Level::from_str(&settings.log_level).unwrap())
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
            .with_max_level(tracing::Level::from_str(&settings.log_level).unwrap())
            .with_level(true)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
    }

    tracing::debug!("logger inited successfully");

    tracing::debug!("{settings:?}");

    let postgres_connection_pool =
        PostgresConnectionPool::new_connection_pool(settings.build_postgres_connection_string())
            .await
            .unwrap();

    let client = redis::Client::open(settings.redis_connection_string).unwrap();
    let redis_connection = client.get_multiplexed_async_connection().await.unwrap();

    tracing::debug!("db connection inited successfully");

    DomainModule::builder()
        .with_component_parameters::<PostgresConnectionPool>(PostgresConnectionPoolParameters {
            connection: postgres_connection_pool,
        })
        .with_component_parameters::<RedisPriceCacheRepository>(
            RedisPriceCacheRepositoryParameters {
                connection: Arc::new(Mutex::new(redis_connection)),
            },
        )
        .with_component_parameters::<TelegramNotificationSender>(
            TelegramNotificationSenderParameters {
                bot: Arc::new(Bot::new(settings.telegram_bot_token)),
            },
        )
        .with_component_parameters::<MoexExchangeRepository>(MoexExchangeRepositoryParameters {
            client: reqwest::Client::new(),
            base_url: settings.moex_base_url,
        })
        .build()
}
