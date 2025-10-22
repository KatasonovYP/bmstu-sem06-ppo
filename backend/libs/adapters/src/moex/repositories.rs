use domain::{
    errors::DomainError,
    models::{
        SecurityEntity,
        TradeEntity,
    },
    ports::exchange::ExchangeRepository,
};
use shaku::Component;

use crate::moex::dto::{
    SecurityResponse,
    TradesResponse,
};

#[derive(Component)]
#[shaku(interface = ExchangeRepository)]
pub struct MoexExchangeRepository {
    client: reqwest::Client,
    base_url: String,
}

impl MoexExchangeRepository {
    pub fn new(base_url: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.unwrap_or_else(|| "https://iss.moex.com".to_string()),
        }
    }
}

impl Default for MoexExchangeRepository {
    fn default() -> Self {
        Self::new(None)
    }
}

#[async_trait::async_trait]
impl ExchangeRepository for MoexExchangeRepository {
    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_security(&self, security_id: &str) -> Result<SecurityEntity, DomainError> {
        let response = self
            .client
            .get(format!(
                "{}/iss/securities/{}.xml",
                self.base_url, security_id,
            ))
            .send()
            .await
            .inspect(|x| tracing::debug!("{x:?}"))
            .unwrap()
            .text()
            .await
            .unwrap();

        let security_response = quick_xml::de::from_str::<SecurityResponse>(&response)
            .inspect(|x| tracing::debug!("{x:?}"))
            .map(SecurityEntity::from)
            .unwrap();

        Ok(security_response)
    }

    #[tracing::instrument(level = "trace", skip(self), err(Debug), ret)]
    async fn get_trades(&self, security_id: &str) -> Result<Vec<TradeEntity>, DomainError> {
        let url = format!(
            "{}/iss/engines/stock/markets/shares/securities/{}/trades.xml?limit=10&reversed=1",
            self.base_url, security_id
        );

        tracing::debug!("Fetching trades data from: {}", url);

        let response = self.client.get(&url).send().await.map_err(|e| {
            tracing::error!("Failed to fetch trades for {}: {}", security_id, e);
            DomainError::ExternalServiceError(format!("HTTP request failed: {e}"))
        })?;

        tracing::debug!("{response:?}");

        if !response.status().is_success() {
            let status = response.status();
            tracing::error!(
                "API returned error status {}: {}",
                status.as_u16(),
                status.as_str()
            );
            return Err(DomainError::ExternalServiceError(format!(
                "API returned status code: {status}"
            )));
        }

        let xml_text = response.text().await.map_err(|e| {
            tracing::error!("Failed to get response body: {}", e);
            DomainError::ExternalServiceError(format!("Failed to read response: {e}"))
        })?;

        let xml_response: TradesResponse = quick_xml::de::from_str(&xml_text).map_err(|e| {
            tracing::error!("Failed to parse XML for trades {}: {}", security_id, e);
            DomainError::ValidationError(format!("XML parsing error: {e}"))
        })?;

        let trades = xml_response.to_trades();
        tracing::debug!("Parsed {} trades", trades.len());

        Ok(trades)
    }
}
