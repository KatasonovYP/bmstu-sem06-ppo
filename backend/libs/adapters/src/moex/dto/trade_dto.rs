use domain::models::TradeEntity;
use serde::Deserialize;

// XML Response structure
#[derive(Debug, Deserialize)]
#[serde(rename = "document")]
pub struct TradesResponse {
    #[serde(rename = "data")]
    data: Vec<DataBlock>,
}

#[derive(Debug, Deserialize)]
struct DataBlock {
    #[serde(rename = "@id")]
    id: String,
    rows: Rows,
}

#[derive(Debug, Deserialize)]
struct Rows {
    #[serde(rename = "row", default)]
    row: Vec<TradeRow>,
}

#[derive(Debug, Deserialize)]
pub struct TradeRow {
    #[serde(rename = "@TRADENO")]
    pub trade_no: Option<String>,

    #[serde(rename = "@TRADETIME")]
    pub trade_time: Option<String>,

    #[serde(rename = "@BOARDID")]
    pub board_id: Option<String>,

    #[serde(rename = "@SECID")]
    pub security_id: Option<String>,

    #[serde(rename = "@PRICE")]
    pub price: Option<String>,

    #[serde(rename = "@QUANTITY")]
    pub quantity: Option<String>,

    #[serde(rename = "@VALUE")]
    pub value: Option<String>,

    #[serde(rename = "@BUYSELL")]
    pub buy_sell: Option<String>,

    #[serde(rename = "@TRADEDATE")]
    pub trade_date: Option<String>,

    #[serde(rename = "@SYSTIME")]
    pub system_time: Option<String>,

    #[serde(rename = "@DECIMALS")]
    pub decimals: Option<String>,
}

// Implementation to convert XML response to a list of trades
impl TradesResponse {
    pub fn to_trades(&self) -> Vec<TradeEntity> {
        // Convert each row to a TradeEntity
        let trades_data = match self.data.iter().find(|block| block.id == "trades") {
            Some(data) => data,
            None => return Vec::new(),
        };

        // Convert each row to a TradeEntity
        trades_data
            .rows
            .row
            .iter()
            .map(|row| TradeEntity {
                trade_no: row
                    .trade_no
                    .as_ref()
                    .and_then(|v| v.parse::<i64>().ok())
                    .unwrap_or_default(),
                trade_time: row.trade_time.clone().unwrap_or_default(),
                board_id: row.board_id.clone().unwrap_or_default(),
                security_id: row.security_id.clone().unwrap_or_default(),
                price: row
                    .price
                    .as_ref()
                    .and_then(|v| v.parse::<f64>().ok())
                    .unwrap_or_default(),
                quantity: row
                    .quantity
                    .as_ref()
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or_default(),
                value: row
                    .value
                    .as_ref()
                    .and_then(|v| v.parse::<f64>().ok())
                    .unwrap_or_default(),
                buy_sell: row.buy_sell.clone().unwrap_or_default(),
                trade_date: row.trade_date.clone().unwrap_or_default(),
                system_time: row.system_time.clone().unwrap_or_default(),
                decimals: row
                    .decimals
                    .as_ref()
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or_default(),
            })
            .collect()
    }
}
