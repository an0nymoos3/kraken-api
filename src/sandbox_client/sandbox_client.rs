use crate::utils::latest_value;
use anyhow::{bail, Result};
use std::collections::HashMap;

// TODO: Support trading with pairs that don't contain same fiat as SandboxClient.

/// A `SandboxClient` represents a fake instance of the account instance used to trade with the kraken.com API.
/// The `SandboxClient` does not take any credentials to ensure it cannot even by mistake access the funds of the
/// user. Therefore, things like balance is manually handed over to the `SandboxClient`.
#[derive(Debug)]
pub struct SandboxClient {
    /// A `SandboxClient` must store its own balance as it cannot access the users balance via the API.
    fiat_balance: f32,

    /// A `SandboxClient` must also have the currency that the user wants to use.
    fiat_currency: String,

    /// Map of <pair, amount> for tracking what crypto the user holds.
    crypto_holdings: HashMap<String, f32>,
}

impl SandboxClient {
    /// Creates a new SandboxClient instance.
    ///
    /// # Parameters
    ///
    /// - `balance`: The amount that the user wants to start out with in their balance (e.g., 1000.0).
    /// - `currency`: The currency that the balance is meassured in. (e.g., "EUR" for euros).
    ///
    /// # Returns
    /// Returns a `SandboxClient`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sandbox_client::SandboxClient;
    /// let my_test_client = SandboxClient::new(1000.0, "EUR");
    /// ```
    pub fn new(balance: f32, currency: &str) -> Self {
        Self {
            fiat_balance: balance,
            fiat_currency: currency.to_string(),
            crypto_holdings: HashMap::new(),
        }
    }

    /// Buy crypto in `SandboxClient`.
    ///
    /// # Parameters
    ///
    /// - `pair`: The pair to trade with (e.g. "XXBTZEUR").
    /// - `amount`: The amount the user wants to trade (e.g. 0.01).
    ///
    /// # Returns
    /// Returns a `Result` indicating if trade is successful or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use sandbox_client::SandboxClient;
    /// let mut my_test_client = SandboxClient::new(1000.0, "EUR");
    /// if let Err(e) = my_test_client.buy("XXBTZEUR", 0.01).await {
    ///     // Handle failure
    /// }
    /// ```
    pub async fn buy(&mut self, pair: &str, amount: f32) -> Result<()> {
        if pair[pair.len() - 3..] != self.fiat_currency {
            bail!("Currently does not support trading with pairs with different fiat from account.")
        }

        let price: f32 = latest_value(pair).await?;

        // Subract the cost of buying the crypto
        self.fiat_balance -= price * amount;

        if !self.crypto_holdings.contains_key(pair) {
            // Insert new investment into crypto holdings
            self.crypto_holdings.insert(pair.to_string(), amount);
        } else {
            // Replace existing investment with the existing investment + new investment
            let account_amount: f32 = *self.crypto_holdings.get(pair).unwrap();
            self.crypto_holdings.remove(pair);
            self.crypto_holdings
                .insert(pair.to_string(), account_amount + amount);
        }

        Ok(())
    }

    /// Sells crypto in `SandboxClient`.
    ///
    /// # Parameters
    ///
    /// - `pair`: The pair to trade with (e.g. "XXBTZEUR").
    /// - `amount`: The amount the user wants to trade (e.g. 0.01).
    ///
    /// # Returns
    /// Returns a `Result` indicating if trade is successful or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use sandbox_client::SandboxClient;
    /// let mut my_test_client = SandboxClient::new(1000.0, "EUR");
    /// if let Err(e) = my_test_client.sell("XXBTZEUR", 0.01).await {
    ///     // Handle failure
    /// }
    /// ```
    pub async fn sell(&mut self, pair: &str, amount: f32) -> Result<()> {
        if pair[pair.len() - 3..] != self.fiat_currency {
            bail!("Currently does not support trading with pairs with different fiat from account.")
        }

        let price: f32 = latest_value(pair).await?;

        if !self.crypto_holdings.contains_key(pair) {
            // Insert new investment into crypto holdings
            bail!("Account does not contain pair!")
        }

        // Subtract the amount of crypto in holdings
        let mut account_amount: f32 = *self.crypto_holdings.get(pair).unwrap();

        // Cap maximum amount able to be sold to the amount holding.
        let sold_amount: f32 = if account_amount > amount {
            amount
        } else {
            account_amount
        };

        // Remove from holding
        account_amount -= sold_amount;
        self.crypto_holdings.remove(pair);

        // Add balance back in fiat
        self.fiat_balance += sold_amount * price;

        // If any amount remains add it back to the accounts crypto holdings
        if account_amount > 0.0 {
            self.crypto_holdings
                .insert(pair.to_string(), account_amount);
        }

        Ok(())
    }
}
