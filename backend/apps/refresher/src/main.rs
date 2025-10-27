use std::sync::Arc;

use adapters::{
    di_domain_module::BuildAppModule,
    settings::Settings,
};
use domain::ports::domain::AbstractPriceCacheService;
use shaku::HasComponent;
use tokio::time::{
    self,
    Duration,
};

#[tokio::main]
async fn main() {
    let settings = Settings::new("config/app.default.yaml").unwrap();
    let module = BuildAppModule::new(&settings).build().await;
    let price_cache_service: Arc<dyn AbstractPriceCacheService> = module.resolve();
    let mut interval = time::interval(Duration::from_secs(settings.refresh_interval_sec));
    loop {
        tracing::info!("new check");
        interval.tick().await;
        price_cache_service.refresh_all_prices().await.unwrap();
    }
}
