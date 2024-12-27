use crate::utils::OhlcData;
use anyhow::Result;
use csv::Writer;
use std::fs::File;

/// Exports OHLC data to csv file.
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
/// if let Err(e) = export_to_csv("somefile.csv", &my_ohlc_data) {
///     // Handle error
/// }
/// ```
///
/// # Errors
///
/// This function will return an error in case of export failure.
pub fn export_to_csv(filename: &str, data: &OhlcData) -> Result<()> {
    let file: File = File::create(filename)?;
    let mut writer = Writer::from_writer(file);

    for ohlc_instance in &data.prices {
        writer.serialize(ohlc_instance)?;
    }

    writer.flush()?;
    Ok(())
}
