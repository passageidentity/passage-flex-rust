use crate::openapi::apis::configuration::Configuration;
use crate::openapi::apis::{authenticate_api, transactions_api};
use crate::Error;

pub struct Auth {
    pub(crate) configuration: Configuration,
}

impl Auth {
    /// Creates a new instance of the `Auth` struct.
    pub fn new(configuration: Configuration) -> Self {
        Self { configuration }
    }

    /// Creates a transaction to start a user's registration process.
    ///
    /// # Arguments
    ///
    /// * `external_id` - A unique, immutable string that represents the user.
    /// * `passkey_display_name` - The label for the user's passkey that they will see when logging in.
    ///
    /// # Returns
    ///
    /// A `Result` containing the transaction ID as a string or an `Error`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use passage_flex::PassageFlex;
    ///
    /// let passage_flex = PassageFlex::new(
    ///     std::env::var("PASSAGE_APP_ID").unwrap(),
    ///     std::env::var("PASSAGE_API_KEY").unwrap(),
    /// );
    ///
    /// let transaction = passage_flex
    ///     .create_register_transaction(
    ///         "00000000-0000-0000-0000-000000000001".to_string(),
    ///         "user@example.com".to_string(),
    ///     )
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn create_register_transaction(
        &self,
        external_id: String,
        passkey_display_name: String,
    ) -> Result<String, Error> {
        transactions_api::create_register_transaction(
            &self.configuration,
            crate::openapi::models::CreateTransactionRegisterRequest {
                external_id,
                passkey_display_name,
            },
        )
        .await
        .map(|response| response.transaction_id)
        .map_err(Into::into)
    }

    /// Creates a transaction to start a user's authentication process.
    ///
    /// # Arguments
    ///
    /// * `external_id` - A unique, immutable string that represents the user.
    ///
    /// # Returns
    ///
    /// A `Result` containing the transaction ID as a string or an `Error`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use passage_flex::PassageFlex;
    ///
    /// let passage_flex = PassageFlex::new(
    ///     std::env::var("PASSAGE_APP_ID").unwrap(),
    ///     std::env::var("PASSAGE_API_KEY").unwrap(),
    /// );
    ///
    /// let transaction = passage_flex
    ///     .create_authenticate_transaction(
    ///         "00000000-0000-0000-0000-000000000001".to_string(),
    ///     )
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn create_authenticate_transaction(
        &self,
        external_id: String,
    ) -> Result<String, Error> {
        transactions_api::create_authenticate_transaction(
            &self.configuration,
            crate::openapi::models::CreateTransactionAuthenticateRequest { external_id },
        )
        .await
        .map(|response| response.transaction_id)
        .map_err(Into::into)
    }

    /// Verifies the nonce received from a WebAuthn registration or authentication ceremony.
    ///
    /// # Arguments
    ///
    /// * `nonce` - The nonce string to be verified.
    ///
    /// # Returns
    ///
    /// A `Result` containing the external ID as a string or an `Error`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use passage_flex::PassageFlex;
    ///
    /// let passage_flex = PassageFlex::new(
    ///     std::env::var("PASSAGE_APP_ID").unwrap(),
    ///     std::env::var("PASSAGE_API_KEY").unwrap(),
    /// );
    ///
    /// match passage_flex.verify_nonce("01234567890123456789".to_string()).await {
    ///     Ok(external_id) => {
    ///         // use external_id to do things like generate and send your own auth token
    ///     }
    ///     Err(err) => {
    ///         // nonce was invalid or unable to be verified
    ///     }
    /// }
    /// ```
    pub async fn verify_nonce(&self, nonce: String) -> Result<String, Error> {
        authenticate_api::authenticate_verify_nonce(
            &self.configuration,
            crate::openapi::models::Nonce { nonce },
        )
        .await
        .map(|response| response.external_id)
        .map_err(Into::into)
    }
}
