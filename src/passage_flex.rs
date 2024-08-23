use crate::openapi::apis::configuration::Configuration;
use crate::openapi::apis::{
    apps_api, authenticate_api, transactions_api, user_devices_api, users_api,
};
use crate::openapi::models;
use crate::Error;
use models::{AppInfo, UserInfo};

pub struct PassageFlex {
    app_id: String,
    configuration: Configuration,
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
        let mut configuration = Configuration::new();
        // Use the api_key as the bearer access token
        configuration.bearer_access_token = Some(api_key);
        // Set the Passage-Version header to the version of the crate
        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert(
            "Passage-Version",
            reqwest::header::HeaderValue::from_static(concat!(
                "passage-flex-rust ",
                env!("CARGO_PKG_VERSION")
            )),
        );
        configuration.client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create reqwest client for Passage");

        let mut client = Self {
            app_id,
            configuration,
        };
        // Set the default server URL
        client.set_server_url(SERVER_URL.to_string());

        client
    }

    fn set_server_url(&mut self, server_url: String) {
        // Use the app_id and server_url to set the base_path
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
            .map(|response| response.app)
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
            models::CreateTransactionRegisterRequest {
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
            models::CreateTransactionAuthenticateRequest { external_id },
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
        authenticate_api::authenticate_verify_nonce(&self.configuration, models::Nonce { nonce })
            .await
            .map(|response| response.external_id)
            .map_err(Into::into)
    }

    /// Get a user's ID in Passage by their external ID
    async fn get_id(&self, external_id: String) -> Result<String, Error> {
        let users = users_api::list_paginated_users(
            &self.configuration,
            Some(1),
            Some(1),
            None,
            None,
            Some(&external_id),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .map(|response| response.users)
        .map_err(Into::into);

        match users {
            Ok(mut users) => match users.len() {
                0 => Err(Error::UserNotFound),
                1 => {
                    let user = users.remove(0);
                    Ok(user.id)
                }
                _ => Err(Error::Other("Multiple users found".to_string())),
            },
            Err(e) => Err(e),
        }
    }

    /// Retrieves information about a user by their external ID.
    ///
    /// # Arguments
    ///
    /// * `external_id` - The unique, immutable ID that represents the user.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `UserInfo` struct or an `Error`.
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
    /// let external_id = "00000000-0000-0000-0000-000000000001";
    /// let user_info = passage_flex.get_user(external_id.to_string()).await.unwrap();
    /// println!("{:?}", user_info.id);
    /// ```
    pub async fn get_user(&self, external_id: String) -> Result<Box<UserInfo>, Error> {
        let user_id = self.get_id(external_id).await?;
        self.get_user_by_id(user_id).await
    }

    /// Retrieves information about a user's passkey devices.
    ///
    /// # Arguments
    ///
    /// * `external_id` - The unique, immutable ID that represents the user.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `WebAuthnDevices` or an `Error`.
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
    /// let external_id = "00000000-0000-0000-0000-000000000001";
    /// let passkey_devices = passage_flex.get_devices(external_id.to_string()).await.unwrap();
    /// for device in passkey_devices {
    ///     println!("{}", device.usage_count);
    /// }
    /// ```
    pub async fn get_devices(
        &self,
        external_id: String,
    ) -> Result<Vec<models::WebAuthnDevices>, Error> {
        let user_id = self.get_id(external_id).await?;
        user_devices_api::list_user_devices(&self.configuration, &user_id)
            .await
            .map(|response| response.devices)
            .map_err(Into::into)
    }

    /// Revokes a user's passkey device.
    ///
    /// # Arguments
    ///
    /// * `external_id` - The unique, immutable ID that represents the user.
    /// * `device_id` - The ID of the device to be revoked.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an `Error`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use passage_flex::PassageFlex;
    /// use chrono::{Duration, NaiveDate, Utc};
    ///
    /// let passage_flex = PassageFlex::new(
    ///     std::env::var("PASSAGE_APP_ID").unwrap(),
    ///     std::env::var("PASSAGE_API_KEY").unwrap(),
    /// );
    ///
    /// let external_id = "00000000-0000-0000-0000-000000000001";
    /// let last_year = Utc::now().naive_utc().date() - Duration::days(365);
    ///
    /// let passkey_devices = passage_flex.get_devices(external_id.to_string()).await.unwrap();
    ///
    /// for device in passkey_devices {
    ///     let last_login_at_parsed =
    ///         NaiveDate::parse_from_str(&device.last_login_at, "%Y-%m-%dT%H:%M:%S%z").unwrap();
    ///
    ///     if last_login_at_parsed < last_year {
    ///         if let Err(err) = passage_flex
    ///             .revoke_device(external_id.clone(), device.id)
    ///             .await
    ///         {
    ///             // device couldn't be revoked
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn revoke_device(&self, external_id: String, device_id: String) -> Result<(), Error> {
        let user_id = self.get_id(external_id).await?;
        user_devices_api::delete_user_devices(&self.configuration, &user_id, &device_id)
            .await
            .map_err(Into::into)
    }

    /// Retrieves information about a user by their user ID in Passage.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user in Passage.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `UserInfo` struct or an `Error`.
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
    /// let user_id = "user_id_string";
    /// let user_info = passage_flex.get_user_by_id(user_id.to_string()).await.unwrap();
    /// println!("{:?}", user_info.external_id);
    /// ```
    pub async fn get_user_by_id(&self, user_id: String) -> Result<Box<UserInfo>, Error> {
        users_api::get_user(&self.configuration, &user_id)
            .await
            .map(|response| response.user)
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod test;
