use super::data_formats::{CoinPriceData, OhlcResponse};
use anyhow::Result;
use reqwest::{self, Response};
use serde_json::Value;

/// Performs GET request to kraken.com API and return Ok(()) if response was received.
pub async fn get_kraken_status() -> Result<()> {
    let response: Response = reqwest::get("https://api.kraken.com/0/public/SystemStatus").await?;
    println!("Received response: {:?}", response);
    Ok(())
}

/// As the name suggests. Returns the OHLC data from kraken.com API.
pub async fn get_ohlc_data(
    pair: &str,
    interval: Option<u32>,
    since: Option<u64>,
) -> Result<OhlcResponse> {
    let mut request: String = format!("https://api.kraken.com/0/public/OHLC?pair={}", pair);

    // Add optional parameters
    if let Some(inter) = interval {
        request.push_str(&format!("&interval={}", inter));
    }
    if let Some(time) = since {
        request.push_str(&format!("&since={}", time));
    }

    // Request OHLC data from kraken API
    let response: Response = reqwest::get(&request).await?;
    let ohlc_data: Value = serde_json::from_str(&response.text().await?)?;

    let mut prices = Vec::new();

    // Convert JSON Array into Vec<CoinPriceData>
    if let Some(result) = ohlc_data.get("result") {
        for (k, v) in result.as_object().unwrap() {
            if let Some(json_array) = v.as_array() {
                prices = json_array
                    .iter()
                    .map(|value| serde_json::from_value(value.clone()).unwrap())
                    .collect::<Vec<CoinPriceData>>();
            }
        }
    }

    // Return Rust representation of OHLC
    Ok(OhlcResponse {
        pair: pair.to_string(),
        prices,
    })
}
