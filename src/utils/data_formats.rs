use serde::{Deserialize, Serialize};
use serde_json;

/// Represents the format the crypto currencies are returned in from kraken.com
#[derive(Debug, Serialize, Deserialize)]
pub struct OhlcResponse {
    pub pair: String,
    pub prices: Vec<CoinPriceData>,
}

/// Helper struct for storing price data from a point in time.
#[derive(Debug, Serialize, Deserialize)]
pub struct CoinPriceData {
    pub time: u64,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub vwap: f32,
    pub volume: f32,
    pub count: u32,
}
