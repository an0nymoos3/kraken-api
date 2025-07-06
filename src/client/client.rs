pub trait KrakenClient {
    pub fn perform_trade() -> Result<()>;
}

/// A Client represents the account instance used to trade with the kraken.com API.
pub struct Client {}

impl Client {
    /// Creates a new Client instance.
    ///
    /// # Returns
    /// Returns a `SandboxClient`.
    ///
    /// # Examples
    ///
    /// ```
    /// use client::Client;
    /// let my_client = Client::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}
