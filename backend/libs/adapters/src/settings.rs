use config::{
    Config,
    ConfigError,
    Environment,
    File,
};
use secrecy::{
    ExposeSecret,
    SecretString,
};
use serde_derive::Deserialize;

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
    pub postgres_password: SecretString,
    pub postgres_host: String,
    pub postgres_port: String,
    pub postgres_database: String,
    pub redis_connection_string: SecretString,
    pub telegram_bot_token: SecretString,
    pub admin_user_login: String,
    pub admin_user_password: SecretString,
    pub admin_user_token: SecretString,
    pub admin_user_pid: String,
    pub e2e_token: SecretString,
    pub e2e_backend_taget_url: String,
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

    pub fn build_postgres_connection_string(&self) -> SecretString {
        SecretString::new(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.postgres_user,
                self.postgres_password.expose_secret(),
                self.postgres_host,
                self.postgres_port,
                self.postgres_database
            )
            .into(),
        )
    }
}
