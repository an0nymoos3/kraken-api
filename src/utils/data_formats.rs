use serde::{Deserialize, Serialize};
use serde_json;

/// Represents the format the crypto currencies are returned in from kraken.com
#[derive(Debug, Serialize, Deserialize)]
pub struct OhlcResponse {
    pub pair: String,
    pub prices: Vec<CoinPriceResponse>,
}

/// Helper struct for storing price data from a point in time.
#[derive(Debug, Serialize, Deserialize)]
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
