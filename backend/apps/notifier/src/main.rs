use std::sync::Arc;

use adapters::{
    di_domain_module::BuildAppModule,
    settings::Settings,
};
use domain::ports::domain::AbstractLimitMonitorService;
use shaku::HasComponent;
use tokio::time::{
    self,
    Duration,
};

#[tokio::main]
async fn main() {
    let settings = Settings::new("config/app.default.yaml").unwrap();
    let module = BuildAppModule::new(&settings).build().await;
    let limit_monitor_service: Arc<dyn AbstractLimitMonitorService> = module.resolve();
    let mut interval = time::interval(Duration::from_secs(settings.notify_interval_sec));
    loop {
        tracing::info!("new check");
        interval.tick().await;
        limit_monitor_service
            .send_exeeding_messages()
            .await
            .unwrap();
    }
}
