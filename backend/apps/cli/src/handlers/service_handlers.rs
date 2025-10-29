use crate::Services;

pub async fn handle_send_exceeding_messages(
    services: &Services,
) -> Result<(), Box<dyn std::error::Error>> {
    services
        .limit_monitor
        .send_exeeding_messages()
        .await
        .map_err(|e| e.into())
}

pub async fn handle_refresh_prices(services: &Services) -> Result<(), Box<dyn std::error::Error>> {
    services
        .price_cache
        .refresh_all_prices()
        .await
        .map_err(|e| e.into())
}
