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
    pub admin_static_s3_bucket: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_host: String,
    pub postgres_port: String,
    pub postgres_database: String,
    pub redis_connection_string: String,
    pub telegram_bot_token: String,
    pub admin_user_login: String,
    pub admin_user_password: String,
    pub admin_user_token: String,
    pub admin_user_pid: String,
}

impl Settings {
    pub fn new(settings_path: &str) -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();
        let settings = Config::builder()
            .add_source(File::with_name(settings_path))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        settings.try_deserialize()
    }

    pub fn build_postgres_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database
        )
    }
}
