use openapi::apis::configuration::Configuration;
use openapi::apis::Error;
use openapi::apis::{apps_api, authenticate_api, transactions_api, users_api, user_devices_api};
use models::{AppInfo, UserInfo};
use openapi::models;

pub struct PassageFlex {
    configuration: Configuration,
}

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

impl PassageFlex {
    /// Initialize the PassageFlex client
    pub fn new(app_id: String, api_key: String) -> Self {
        let mut configuration = Configuration::new();
        // Use the app_id to set the base path
        configuration.base_path = format!("https://api.passage.id/v1/apps/{}", app_id);
        // Use the api_key as the bearer access token
        configuration.bearer_access_token = Some(api_key);
        // Set the Passage-Version header to the version of the crate
        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert(
            "Passage-Version",
            reqwest::header::HeaderValue::from_static(PKG_VERSION),
        );
        configuration.client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create reqwest client for Passage");

        Self { configuration }
    }

    /// Get the app information
    pub async fn get_app(&self) -> Result<Box<AppInfo>, Error<apps_api::GetAppError>> {
        apps_api::get_app(&self.configuration)
            .await
            .map(|response| response.app)
    }

    /// Create a transaction to start a user's registration process
    pub async fn create_register_transaction(
        &self,
        external_id: String,
        passkey_display_name: String,
    ) -> Result<String, Error<transactions_api::CreateRegisterTransactionError>> {
        transactions_api::create_register_transaction(
            &self.configuration,
            models::CreateTransactionRegisterRequest {
                external_id,
                passkey_display_name,
            },
        )
        .await
        .map(|response| response.transaction_id)
    }

    /// Create a transaction to start a user's authentication process
    pub async fn create_authenticate_transaction(
        &self,
        external_id: String,
    ) -> Result<String, Error<transactions_api::CreateAuthenticateTransactionError>> {
        transactions_api::create_authenticate_transaction(
            &self.configuration,
            models::CreateTransactionAuthenticateRequest { external_id },
        )
        .await
        .map(|response| response.transaction_id)
    }

    /// Verify the nonce received from a WebAuthn registration or authentication ceremony
    pub async fn verify_nonce(
        &self,
        nonce: String,
    ) -> Result<String, Error<authenticate_api::AuthenticateVerifyNonceError>> {
        authenticate_api::authenticate_verify_nonce(&self.configuration, models::Nonce { nonce })
            .await
            .map(|response| response.external_id)
    }

    /// Get a user by their external ID
    pub async fn get_user(
        &self,
        external_id: String,
    ) -> Result<Box<UserInfo>, Error<users_api::GetUserError>> {
        users_api::get_user(&self.configuration, &external_id)
            .await
            .map(|response| response.user)
    }

    /// Get a user's devices by their external ID
    pub async fn get_devices(
        &self,
        external_id: String,
    ) -> Result<Vec<models::WebAuthnDevices>, Error<user_devices_api::ListUserDevicesError>> {
        user_devices_api::list_user_devices(&self.configuration, &external_id)
            .await
            .map(|response| response.devices)
    }

    /// Revoke a user's device by their external ID and the device ID
    pub async fn revoke_device(
        &self,
        external_id: String,
        device_id: String,
    ) -> Result<(), Error<user_devices_api::DeleteUserDevicesError>> {
        user_devices_api::delete_user_devices(&self.configuration, &external_id, &device_id)
            .await
    }
    
    /// Get a user by their user ID
    pub async fn get_user_by_id(
        &self,
        user_id: String,
    ) -> Result<Box<UserInfo>, Error<users_api::GetUserError>> {
        users_api::get_user(&self.configuration, &user_id)
            .await
            .map(|response| response.user)
    }
}
