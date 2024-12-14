/// A `SandboxClient` represents a fake instance of the account instance used to trade with the kraken.com API.
/// The `SandboxClient` does not take any credentials to ensure it cannot even by mistake access the funds of the
/// user. Therefore things like balance is manually handed over to the `SandboxClient`.
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
    /// let my_fake_client = SandboxClient::new(1000, "EUR");
    /// ```
    pub fn new(balance: i32, currency: &str) -> Self {
        Self {
            balance,
            balance_currency: currency.to_string(),
        }
    }
}
