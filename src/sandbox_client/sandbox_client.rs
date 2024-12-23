use anyhow::Result;

/// A `SandboxClient` represents a fake instance of the account instance used to trade with the kraken.com API.
/// The `SandboxClient` does not take any credentials to ensure it cannot even by mistake access the funds of the
/// user. Therefore, things like balance is manually handed over to the `SandboxClient`.
pub struct SandboxClient {
    /// A `SandboxClient` must store its own balance as it cannot access the users balance via the API.
    balance: i32,

    /// A `SandboxClient` must also have the currency that the user wants to use.
    balance_currency: String,
}

impl SandboxClient {
    /// Creates a new SandboxClient instance.
    ///
    /// # Parameters
    ///
    /// - `balance`: The amount that the user wants to start out with in their balance (e.g., 1000).
    /// - `currency`: The currency that the balance is meassured in. (e.g., "EUR" for euros).
    ///
    /// # Returns
    /// Returns a `SandboxClient`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sandbox_client::SandboxClient;
    /// let my_test_client = SandboxClient::new(1000, "EUR");
    /// ```
    pub fn new(balance: i32, currency: &str) -> Self {
        Self {
            balance,
            balance_currency: currency.to_string(),
        }
    }

    /// Performs a fake trade with the balance in the SandboxClient.
    ///
    /// # Parameters
    ///
    /// - `pair`: The pair to trade with (e.g. "XXBTZEUR").
    /// - `amount`: The amount the user wants to trade (e.g. 0.01).
    ///
    /// # Returns
    /// Returns a `Result` indicating if trade would have been successful or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use sandbox_client::SandboxClient;
    /// let my_test_client = SandboxClient::new(1000, "EUR");
    /// match my_test_client.trade("XXBTZEUR", 0.01).await {
    ///     Ok(_) => // Handle success
    ///     Err(e) => // Handle failure
    /// }
    /// ```
    pub async fn trade(&mut self, pair: &str, amount: f32) -> Result<()> {
        Ok(())
    }
}
