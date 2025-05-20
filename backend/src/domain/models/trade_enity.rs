#[cfg_attr(not(feature = "production"), derive(fake::Dummy))]
#[derive(Debug, Clone)]
pub struct TradeEntity {
    pub trade_no: i64,
    pub trade_time: String,
    pub board_id: String,
    pub security_id: String,
    pub price: f64,
    pub quantity: i32,
    pub value: f64,
    pub buy_sell: String,
    pub trade_date: String,
    pub system_time: String,
    pub decimals: i32,
}
