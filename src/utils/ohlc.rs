use crate::utils::internal_data_formats::CoinPriceResponse;
use serde::{Deserialize, Serialize};

/// A `OhlcData` represents the response from kraken.com API when requesting the OHLC of a pair.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OhlcData {
    /// A `OhlcData` always has a pair, (e.g., "XXBTZEUR").
    pub pair: String,

    /// A `OhlcData` contains a vector or individual "timestamps" when the OHLC data was collected
    /// from.
    pub prices: Vec<OhlcPriceInstance>,
}

/// A `OhlcPriceInstance` is OHLC data from a single point in time.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct OhlcPriceInstance {
    pub time: u64,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub vwap: f32,
    pub volume: f32,
    pub count: u32,
}

impl From<CoinPriceResponse> for OhlcPriceInstance {
    fn from(item: CoinPriceResponse) -> Self {
        let time = item.time;
        let open = item.open.parse::<f32>().unwrap();
        let high = item.high.parse::<f32>().unwrap();
        let low = item.low.parse::<f32>().unwrap();
        let close = item.close.parse::<f32>().unwrap();
        let vwap = item.vwap.parse::<f32>().unwrap();
        let volume = item.volume.parse::<f32>().unwrap();
        let count = item.count;

        Self {
            time,
            open,
            high,
            low,
            close,
            vwap,
            volume,
            count,
        }
    }
}
