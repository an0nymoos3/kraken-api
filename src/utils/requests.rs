use super::internal_data_formats::{CoinPriceResponse, OhlcResponse};
use crate::utils::ohlc::{OhlcData, OhlcPriceInstance};
use anyhow::{bail, Result};
use reqwest::{self, Response};
use serde_json::Value;
use std::any::Any;

/// Sends a system status request to the kraken.com API.
///
/// # Returns
///
/// Returns an empty `Result` on success or an error if the request fails.
///
/// # Example
///
/// ```rust
/// if let Err(e) = get_kraken_status().await {
///     // Handle the error
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the request fails.
pub async fn get_kraken_status() -> Result<()> {
    let _: Response = reqwest::get("https://api.kraken.com/0/public/SystemStatus").await?;
    Ok(())
}

/// Fetches OHLC (Open, High, Low, Close) data for a specific trading pair.
///
/// # Parameters
///
/// - `pair`: The trading pair for which to fetch the OHLC data (e.g., `"XTCEUR"`).
/// - `interval`: An optional time interval for the OHLC data in minutes (e.g., 15 for 15-minute intervals). Defaults to `None`.
/// - `since`: An optional timestamp (in milliseconds) to fetch OHLC data starting from that point. Defaults to `None`.
///
/// # Returns
///
/// Returns a `Result` containing an `OhlcResponse` on success or an error if the request fails.
///
/// # Example
///
/// ```rust
/// let ohlc = match get_ohlc_data("XTCEUR", Some(15), Some(1631500800000)).await {
///     Ok(result) => result,
///     Err(e) => {
///         // Handle the error
///     }
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the request fails, or if the parameters are invalid.
pub async fn get_ohlc_data(
    pair: &str,
    interval: Option<u32>,
    since: Option<u64>,
) -> Result<OhlcData> {
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

    // Convert JSON Array into Vec<CoinPriceData>
    if let Some(result) = ohlc_data.get("result") {
        for (_k, v) in result.as_object().unwrap() {
            if let Some(json_array) = v.as_array() {
                // Create an array of prices with serde_json
                let prices: Vec<CoinPriceResponse> = json_array
                    .iter()
                    .map(|value| serde_json::from_value(value.clone()).unwrap())
                    .collect::<Vec<CoinPriceResponse>>();

                // Use custom try_from to convert to proper Rust instance
                let new_prices: Vec<OhlcPriceInstance> = prices
                    .iter()
                    .map(|price_instance| OhlcPriceInstance::from(price_instance.clone()))
                    .collect::<Vec<OhlcPriceInstance>>();

                // Return Rust representation of OHLC
                return Ok(OhlcData {
                    pair: pair.to_string(),
                    prices: new_prices,
                });
            }
        }
    }
    bail!("Failed to parse OHLC data from kraken.com!")
}

/// Sends a request to the Kraken.com API for all pairs available to trade.
///
/// # Returns
///
/// Returns a `Result` with a `Vec<String>` representing pairs on success or an error if the request fails.
///
/// # Example
///
/// ```rust
/// use kraken_api::utils::requests::get_tradable_pairs;
/// let pairs = match get_tradable_pairs().await {
///     Ok(result) => result,
///     Err(e) => {
///         // Handle error
///     }
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the request fails.
pub async fn get_tradable_pairs() -> Result<Vec<String>> {
    let request: &str = "https://api.kraken.com/0/public/AssetPairs";

    // Request all pairs from kraken.com
    let response: Response = reqwest::get(request).await?;
    let json_response: Value = serde_json::from_str(&response.text().await?)?;

    // Convert JSON Array into Vec<String>
    if let Some(result) = json_response.get("result") {
        let mut pairs: Vec<String> = Vec::new();
        for (k, _v) in result.as_object().unwrap() {
            pairs.push(k.to_string());
        }

        return Ok(pairs);
    }

    bail!("Failed to parse tradable pairs from kraken.com!")
}

/// Sends a request to the Kraken.com API for the latest value of a pair.
///
/// # Returns
///
/// Returns a `Result` with a `f32` which is the price of the pair.
///
/// # Example
///
/// ```rust
/// use kraken_api::utils::requests::latest_value;
/// let price = match latest_value().await {
///     Ok(result) => result,
///     Err(e) => {
///         // Handle error
///     }
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the request fails.
pub async fn latest_value(pair: &str) -> Result<f64> {
    let request: String = format!(
        "https://api.kraken.com/0/public/Trades?pair={}&count=1",
        pair
    );

    // Request latest trade for the pair.
    // Request returns response in the following format:
    // Array of trade entries [<price>, <volume>, <time>, <buy/sell>, <market/limit>, <miscellaneous>, <trade_id>]
    let response: Response = reqwest::get(request).await?;
    let json_response: Value = serde_json::from_str(&response.text().await?)?;

    // Return the first element from the JSON array result.
    if let Some(result) = json_response.get("result") {
        for (_k, v) in result.as_object().unwrap() {
            if let Some(json_array) = v.as_array() {
                if let Some(inner_array) = json_array.first().unwrap().as_array() {
                    return Ok(inner_array
                        .first()
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .parse::<f64>()?);
                }
            }
        }
    }

    bail!("Failed to parse tradable pairs from kraken.com!")
}
