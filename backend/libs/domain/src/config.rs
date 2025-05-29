use shaku::{
    Component,
    Interface,
};

#[async_trait::async_trait]
pub trait AbstractAppConfig: Interface + Send + Sync + 'static {
    fn get_connection_string(&self) -> &str;
}

#[derive(Clone, Component)]
#[shaku(interface = AbstractAppConfig)]
pub struct AppConfig {
    #[shaku(default)]
    pub connection_string: String,
    pub log_format: String,
}

#[async_trait::async_trait]
impl AbstractAppConfig for AppConfig {
    fn get_connection_string(&self) -> &str {
        &self.connection_string
    }
}
