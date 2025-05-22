use std::sync::Arc;

use shaku::HasComponent;
use stocks_tracker::{
    app::di_domain_module::di_domain_module,
    domain::ports::domain::AbstractPriceCacheService,
};

#[tokio::main]
async fn main() {
    let module = di_domain_module().await;
    let price_cache_service: Arc<dyn AbstractPriceCacheService> = module.resolve();
    price_cache_service.start().await.unwrap();
}
