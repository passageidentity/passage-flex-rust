use crate::models::AppInfo;
use crate::openapi::apis::configuration::Configuration;
use crate::openapi::apis::{apps_api, authenticate_api, transactions_api};
use crate::user::User;
use crate::Error;

pub struct PassageFlex {
    app_id: String,
    configuration: Configuration,
    pub user: User,
}

const SERVER_URL: &str = "https://api.passage.id";

impl PassageFlex {
    /// Creates a new instance of the `PassageFlex` client.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The Passage application ID.
    /// * `api_key` - The Passage API key.
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
    /// ```
    pub fn new(app_id: String, api_key: String) -> Self {
        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert(
            "Passage-Version",
            reqwest::header::HeaderValue::from_static(concat!(
                "passage-flex-rust ",
                env!("CARGO_PKG_VERSION")
            )),
        );

        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(api_key);
        configuration.client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create reqwest client for Passage");

        let user = User::new(configuration.clone());

        let mut client = Self {
            app_id,
            configuration,
            user,
        };
        // Set the default server URL
        client.set_server_url(SERVER_URL.to_string());

        client
    }

    fn set_server_url(&mut self, server_url: String) {
        self.user.configuration.base_path = format!("{}/v1/apps/{}", server_url, self.app_id);
        self.configuration.base_path = format!("{}/v1/apps/{}", server_url, self.app_id);
    }

    /// Retrieves information about the application.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `AppInfo` struct or an `Error`.
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
    /// let app_info = passage_flex.get_app().await.unwrap();
    /// println!("{}", app_info.auth_origin);
    /// ```
    pub async fn get_app(&self) -> Result<Box<AppInfo>, Error> {
        apps_api::get_app(&self.configuration)
            .await
            .map(|response| {
                Box::new(AppInfo {
                    auth_origin: response.app.auth_origin,
                    id: response.app.id,
                    name: response.app.name,
                })
            })
            .map_err(Into::into)
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

#[cfg(test)]
mod test;
