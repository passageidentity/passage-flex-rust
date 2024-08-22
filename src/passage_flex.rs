use crate::Error;
use models::{AppInfo, UserInfo};
use openapi::apis::configuration::Configuration;
use openapi::apis::{apps_api, authenticate_api, transactions_api, user_devices_api, users_api};
use openapi::models;

pub struct PassageFlex {
    app_id: String,
    configuration: Configuration,
}

const SERVER_URL: &str = "https://api.passage.id";

impl PassageFlex {
    /// Initialize the PassageFlex client
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

    pub fn set_server_url(&mut self, server_url: String) {
        // Use the app_id and server_url to set the base_path
        self.configuration.base_path = format!("{}/v1/apps/{}", server_url, self.app_id);
    }

    /// Get the app information
    pub async fn get_app(&self) -> Result<Box<AppInfo>, Error> {
        apps_api::get_app(&self.configuration)
            .await
            .map(|response| response.app)
            .map_err(Into::into)
    }

    /// Create a transaction to start a user's registration process
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

    /// Create a transaction to start a user's authentication process
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

    /// Verify the nonce received from a WebAuthn registration or authentication ceremony
    pub async fn verify_nonce(&self, nonce: String) -> Result<String, Error> {
        authenticate_api::authenticate_verify_nonce(&self.configuration, models::Nonce { nonce })
            .await
            .map(|response| response.external_id)
            .map_err(Into::into)
    }

    /// Get a user by their external ID
    pub async fn get_user(&self, external_id: String) -> Result<Box<UserInfo>, Error> {
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
        .map(|response| response.users);

        match users {
            Ok(mut users) => match users.len() {
                0 => Err(Error::UserNotFound),
                1 => {
                    let user = users.remove(0);
                    self.get_user_by_id(user.id).await
                }
                _ => Err(Error::Other("Multiple users found".to_string())),
            },
            Err(e) => Err(e.into()),
        }
    }

    /// Get a user's devices by their external ID
    pub async fn get_devices(
        &self,
        external_id: String,
    ) -> Result<Vec<models::WebAuthnDevices>, Error> {
        let user = match self.get_user(external_id).await {
            Ok(user) => user,
            Err(e) => return Err(e),
        };

        user_devices_api::list_user_devices(&self.configuration, &user.id)
            .await
            .map(|response| response.devices)
            .map_err(Into::into)
    }

    /// Revoke a user's device by their external ID and the device ID
    pub async fn revoke_device(&self, external_id: String, device_id: String) -> Result<(), Error> {
        let user = match self.get_user(external_id).await {
            Ok(user) => user,
            Err(e) => return Err(e),
        };

        user_devices_api::delete_user_devices(&self.configuration, &user.id, &device_id)
            .await
            .map_err(Into::into)
    }

    /// Get a user by their user ID
    pub async fn get_user_by_id(&self, user_id: String) -> Result<Box<UserInfo>, Error> {
        users_api::get_user(&self.configuration, &user_id)
            .await
            .map(|response| response.user)
            .map_err(Into::into)
    }
}
