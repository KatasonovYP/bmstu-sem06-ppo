use std::sync::Arc;

use shaku::HasComponent;
use stocks_tracker::{
    app::di_domain_module::di_domain_module,
    domain::ports::domain::AbstractLimitMonitorService,
};

#[tokio::main]
async fn main() {
    let module = di_domain_module().await;
    let limit_monitor_service: Arc<dyn AbstractLimitMonitorService> = module.resolve();
    limit_monitor_service.start().await.unwrap();
}
