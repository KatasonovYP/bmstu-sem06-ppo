use config::{
    Config,
    ConfigError,
    Environment,
    File,
};
use serde_derive::Deserialize;

#[cfg_attr(not(feature = "production"), derive(fake::Dummy))]
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub log_level: String,
    pub log_filepath: String,
    pub refresh_interval_sec: u64,
    pub notify_interval_sec: u64,
    pub moex_base_url: String,
    pub api_server_port: u16,
    pub api_cors_origins: Vec<String>,
    pub local_admin: bool,
    pub postgres_connection_string: String,
    pub redis_connection_string: String,
    pub telegram_bot_token: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();
        let settings_path: String = "config".into();
        // let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let settings = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(&format!("{settings_path}/app.default.yaml")))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            // .add_source(
            //     File::with_name(&format!("{settings_path}/{run_mode}"))
            //         .required(false),
            // )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            // .add_source(File::with_name(&format!("{settings_path}/local")).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            // .set_override("database.url", "postgres://")?
            .build()?;

        settings.try_deserialize()
    }
}
