use crate::utils::{OhlcData, OhlcPriceInstance};
use anyhow::Result;
use bincode;
use rocksdb::{IteratorMode, Options, DB};

/// Writes OHLC data to a rocksdb file.
/// For performance/compactness the OHLC data is first serialized
/// into a binary representation with bincode.
///
/// # Parameters
///
/// - `db_path`: Name of the database that data is exported to. (e.g., "xbt_db").
/// - `data`: A reference to the `OhlcData` that should be exported.
///
/// # Returns
///
/// Returns a `Result` indicating success of failure.
///
/// # Example
///
/// ```rust
/// use kraken_api::export::export_to_csv;
/// use kraken_api::utils::get_ohlc_data;
///
/// let my_ohlc_data = get_ohlc_data("XBTEUR", Some(15), Some(1631500800000)).await.unwrap();
///
/// if let Err(e) = write_to_db("xbt_db", &my_ohlc_data) {
///     // Handle error
/// }
/// ```
///
/// # Errors
///
/// This function will return an error in case of export failure.
pub fn write_to_db(db_path: &str, data: &OhlcData) -> Result<()> {
    let db: DB = DB::open_default(db_path)?;

    for ohlc_instance in &data.prices {
        let bin_encoded: Vec<u8> = bincode::serialize(&ohlc_instance)?;
        db.put(ohlc_instance.time.to_le_bytes(), bin_encoded)?;
    }

    Ok(())
}

/// Reads OHLC data from a rocksdb file.
///
/// # Parameters
///
/// - `db_path`: Name of the database data is stored in. (e.g., "xbt_db").
/// - `pair`: The name of the pair being read. This is used in the `pair` field of the `OhlcData` structure that is generated. (e.g. "XBTEUR")
///
/// # Returns
///
/// Returns a `Result` which contains the `OhlcData` that was read from the db file.
///
/// # Example
///
/// ```rust
/// use kraken_api::export::export_to_csv;
/// use kraken_api::utils::get_ohlc_data;
///
/// let my_ohlc_data = get_ohlc_data("XBTEUR", Some(15), Some(1631500800000)).await.unwrap();
///
/// if let Err(e) = write_to_db("xbt_db", &my_ohlc_data) {
///     // Handle error
/// }
/// ```
///
/// # Errors
///
/// This function will return an error in case of export failure.
pub fn read_from_db(db_path: &str, pair: &str) -> Result<OhlcData> {
    let db: DB = DB::open_default(db_path)?;
    let mut ohlc_prices: Vec<OhlcPriceInstance> = Vec::new();
    let mut iter = db.iterator(IteratorMode::Start);

    for item in iter {
        let (_k, v) = item?;
        ohlc_prices.push(bincode::deserialize(&v)?);
    }

    Ok(OhlcData {
        pair: pair.to_string(),
        prices: ohlc_prices,
    })
}
