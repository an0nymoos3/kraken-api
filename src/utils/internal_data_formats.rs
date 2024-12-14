use serde::{Deserialize, Serialize};
use serde_json;

/// Internal data-structure for handling the response produced from the OHLC request to the kraken.com API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OhlcResponse {
    pub pair: String,
    pub prices: Vec<CoinPriceResponse>,
}

/// Internal data-structure for handling the nested data within a response produced from the OHLC
/// request to the kraken.com API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinPriceResponse {
    pub time: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub vwap: String,
    pub volume: String,
    pub count: u32,
}
