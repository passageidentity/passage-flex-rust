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

    fn set_server_url(&mut self, server_url: String) {
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

#[cfg(test)]
mod test {
    use super::*;
    use mockito::{Matcher, Mock, Server, ServerGuard};

    #[tokio::test]
    async fn test_passage_flex_constructor() {
        let app_id = "test_app_id".to_string();
        let api_key = "test_api_key".to_string();

        // Test successful creation
        assert!(
            std::panic::catch_unwind(|| {
                PassageFlex::new(app_id.clone(), api_key.clone());
            })
            .is_ok(),
            "Should not panic when app_id and api_key are provided"
        );
    }

    async fn setup_passage_flex() -> (String, PassageFlex, ServerGuard) {
        // Setup PassageFlex instance
        let app_id = "test_app_id".to_string();
        let api_key = "test_api_key".to_string();
        let mut passage = PassageFlex::new(app_id.clone(), api_key.clone());
        // Setup mock server
        let server = Server::new_async().await;
        // Set server URL to mock server
        passage.set_server_url(server.url());

        (app_id, passage, server)
    }

    async fn setup_empty_list_paginated_users_mock(app_id: &str, server: &mut ServerGuard) -> Mock {
        server
        .mock(
            "GET",
            format!("/v1/apps/{}/users?page=1&limit=1&identifier=invalid", app_id).as_str(),
        )
        .with_status(200)
        .with_body(
            r#"{
            "users": [],
            "_links": {
                "first": {
                    "href": "api.passage.id/v1/apps/test/users?created_before=1724113247\u0026limit=15\u0026order_by=created_at%3ADESC\u0026page=1"
                },
                "last": {
                    "href": "api.passage.id/v1/apps/test/users?created_before=1724113247\u0026limit=15\u0026order_by=created_at%3ADESC\u0026page=1"
                },
                "next": {
                    "href": ""
                },
                "previous": {
                    "href": ""
                },
                "self": {
                    "href": "api.passage.id/v1/apps/test/users?created_before=1724113247\u0026limit=15\u0026order_by=created_at%3ADESC\u0026page=1"
                }
            },
            "created_before": 1724113247,
            "limit": 1,
            "page": 1,
            "total_users": 0
        }"#,
        )
        .create_async()
        .await
    }

    async fn setup_valid_list_paginated_users_mock(app_id: &str, server: &mut ServerGuard) -> Mock {
        server
        .mock(
            "GET",
            format!("/v1/apps/{}/users?page=1&limit=1&identifier=valid", app_id).as_str(),
        )
        .with_status(200)
        .with_body(
            r#"{
            "users": [
                {
                    "created_at": "2021-01-01T00:00:00Z",
                    "email": "",
                    "email_verified": false,
                    "external_id": "valid",
                    "id": "test_passage_id",
                    "last_login_at": "2021-01-02T00:00:00Z",
                    "login_count": 1,
                    "phone": "",
                    "phone_verified": false,
                    "status": "active",
                    "updated_at": "2021-01-03T00:00:00Z",
                    "user_metadata": null
                }
            ],
            "_links": {
                "first": {
                    "href": "api.passage.id/v1/apps/test/users?created_before=1724113247\u0026limit=15\u0026order_by=created_at%3ADESC\u0026page=1"
                },
                "last": {
                    "href": "api.passage.id/v1/apps/test/users?created_before=1724113247\u0026limit=15\u0026order_by=created_at%3ADESC\u0026page=1"
                },
                "next": {
                    "href": ""
                },
                "previous": {
                    "href": ""
                },
                "self": {
                    "href": "api.passage.id/v1/apps/test/users?created_before=1724113247\u0026limit=15\u0026order_by=created_at%3ADESC\u0026page=1"
                }
            },
            "created_before": 1724113247,
            "limit": 1,
            "page": 1,
            "total_users": 1
        }"#,
        )
        .create_async()
        .await
    }

    async fn setup_valid_get_user_mock(app_id: &str, server: &mut ServerGuard) -> Mock {
        server
            .mock(
                "GET",
                format!("/v1/apps/{}/users/test_passage_id", app_id).as_str(),
            )
            .with_status(200)
            .with_body(
                r#"{
            "user": {
                    "created_at": "2021-01-01T00:00:00Z",
                    "email": "",
                    "email_verified": false,
                    "external_id": "valid",
                    "id": "test_passage_id",
                    "last_login_at": "2021-01-02T00:00:00Z",
                    "login_count": 1,
                    "phone": "",
                    "phone_verified": false,
                    "recent_events": [],
                    "social_connections": {},
                    "status": "active",
                    "updated_at": "2021-01-03T00:00:00Z",
                    "user_metadata": null,
                    "webauthn": false,
                    "webauthn_devices": [{
                        "created_at": "2021-01-01T00:00:00Z",
                        "cred_id": "test_cred_id",
                        "friendly_name": "Device 1",
                        "id": "test_device_id",
                        "last_login_at": "2021-01-03T00:00:00Z",
                        "type": "platform",
                        "updated_at": "2021-01-02T00:00:00Z",
                        "usage_count": 1,
                        "icons": {"light": null, "dark": null}
                    }],
                    "webauthn_types": ["platform"]
            }
        }"#,
            )
            .create_async()
            .await
    }

    async fn setup_valid_get_devices_mock(app_id: &str, server: &mut ServerGuard) -> Mock {
        server
            .mock(
                "GET",
                format!("/v1/apps/{}/users/test_passage_id/devices", app_id).as_str(),
            )
            .with_status(200)
            .with_body(
                r#"{
            "devices": [
                {
                    "created_at": "2021-01-01T00:00:00Z",
                    "cred_id": "test_cred_id",
                    "friendly_name": "Device 1",
                    "id": "test_device_id",
                    "last_login_at": "2021-01-03T00:00:00Z",
                    "type": "platform",
                    "updated_at": "2021-01-02T00:00:00Z",
                    "usage_count": 1,
                    "icons": {"light": null, "dark": null}
                }
            ]
        }"#,
            )
            .create_async()
            .await
    }

    async fn setup_valid_revoke_device_mock(app_id: &str, server: &mut ServerGuard) -> Mock {
        server
            .mock(
                "DELETE",
                format!(
                    "/v1/apps/{}/users/test_passage_id/devices/test_device_id",
                    app_id
                )
                .as_str(),
            )
            .with_status(200)
            .create_async()
            .await
    }

    #[tokio::test]
    async fn test_create_register_transaction() {
        let (app_id, passage_flex, mut server) = setup_passage_flex().await;
        let m1 = server
            .mock(
                "POST",
                format!("/v1/apps/{}/transactions/register", app_id).as_str(),
            )
            .match_body(Matcher::Regex(
                r#"\{"external_id":"test","passkey_display_name":"test"\}"#.into(),
            ))
            .with_status(200)
            .with_body(r#"{"transaction_id": "test_transaction_id"}"#)
            .create_async()
            .await;

        let transaction_id = passage_flex
            .create_register_transaction("test".to_string(), "test".to_string())
            .await
            .unwrap();
        m1.assert_async().await;
        assert_eq!(transaction_id, "test_transaction_id");
    }

    #[tokio::test]
    async fn test_create_authenticate_transaction() {
        let (app_id, passage_flex, mut server) = setup_passage_flex().await;
        let m1 = server
            .mock(
                "POST",
                format!("/v1/apps/{}/transactions/authenticate", app_id).as_str(),
            )
            .match_body(Matcher::Regex(r#"\{"external_id":"test"\}"#.into()))
            .with_status(200)
            .with_body(r#"{"transaction_id": "test_transaction_id"}"#)
            .create_async()
            .await;

        let transaction_id = passage_flex
            .create_authenticate_transaction("test".to_string())
            .await
            .unwrap();
        m1.assert_async().await;
        assert_eq!(transaction_id, "test_transaction_id");
    }

    #[tokio::test]
    async fn test_verify_nonce() {
        let (app_id, passage_flex, mut server) = setup_passage_flex().await;
        let m1 = server.mock("POST", format!("/v1/apps/{}/authenticate/verify", app_id).as_str())
        .match_body(Matcher::Regex(r#"\{"nonce":"invalid"\}"#.into()))
        .with_status(400)
        .with_body(r#"{"error": "Could not verify nonce: nonce is invalid, expired, or cannot be found","code": "invalid_nonce"}"#)
        .create_async().await;

        let invalid_result = passage_flex.verify_nonce("invalid".to_string()).await;
        assert!(invalid_result.is_err());
        m1.assert_async().await;

        let m2 = server
            .mock(
                "POST",
                format!("/v1/apps/{}/authenticate/verify", app_id).as_str(),
            )
            .match_body(Matcher::Regex(r#"\{"nonce":"valid"\}"#.into()))
            .with_status(200)
            .with_body(r#"{"external_id": "test_external_id"}"#)
            .create_async()
            .await;

        let external_id = passage_flex
            .verify_nonce("valid".to_string())
            .await
            .unwrap();
        m2.assert_async().await;
        assert_eq!(external_id, "test_external_id");
    }

    #[tokio::test]
    async fn test_get_user() {
        let (app_id, passage_flex, mut server) = setup_passage_flex().await;

        let m1 = setup_empty_list_paginated_users_mock(&app_id, &mut server).await;
        let invalid_result = passage_flex.get_user("invalid".to_string()).await;
        m1.assert_async().await;
        assert!(invalid_result.is_err());

        let m2 = setup_valid_list_paginated_users_mock(&app_id, &mut server).await;
        let m3 = setup_valid_get_user_mock(&app_id, &mut server).await;

        let user_info = passage_flex.get_user("valid".to_string()).await.unwrap();
        m2.assert_async().await;
        m3.assert_async().await;
        assert_eq!(user_info.external_id, "valid");
    }

    #[tokio::test]
    async fn test_get_devices() {
        let (app_id, passage_flex, mut server) = setup_passage_flex().await;

        let m1 = setup_empty_list_paginated_users_mock(&app_id, &mut server).await;
        let invalid_result = passage_flex.get_devices("invalid".to_string()).await;
        m1.assert_async().await;
        assert!(invalid_result.is_err());

        let m2 = setup_valid_list_paginated_users_mock(&app_id, &mut server).await;
        let m3 = setup_valid_get_user_mock(&app_id, &mut server).await;
        let m4 = setup_valid_get_devices_mock(&app_id, &mut server).await;
        let devices = passage_flex.get_devices("valid".to_string()).await.unwrap();
        m2.assert_async().await;
        m3.assert_async().await;
        m4.assert_async().await;
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].id, "test_device_id");
    }

    #[tokio::test]
    async fn test_revoke_device() {
        let (app_id, passage_flex, mut server) = setup_passage_flex().await;

        let m1 = setup_empty_list_paginated_users_mock(&app_id, &mut server).await;
        let invalid_result = passage_flex
            .revoke_device("invalid".to_string(), "invalid".to_string())
            .await;
        m1.assert_async().await;
        assert!(invalid_result.is_err());

        let m2 = setup_valid_list_paginated_users_mock(&app_id, &mut server).await;
        let m3 = setup_valid_get_user_mock(&app_id, &mut server).await;
        let m4 = setup_valid_revoke_device_mock(&app_id, &mut server).await;
        let result = passage_flex
            .revoke_device("valid".to_string(), "test_device_id".to_string())
            .await;
        m2.assert_async().await;
        m3.assert_async().await;
        m4.assert_async().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_app() {
        let mut server = Server::new_async().await;
        let api_key = "test_api_key";

        let app_id = "invalid";
        let mut invalid_passage = PassageFlex::new(app_id.to_string(), api_key.to_string());
        invalid_passage.set_server_url(server.url());
        let m1 = server
            .mock("GET", "/v1/apps/invalid/")
            .with_status(404)
            .with_body(r#"{"error": "App not found","code": "app_not_found"}"#)
            .create_async()
            .await;
        let invalid_result = invalid_passage.get_app().await;
        m1.assert_async().await;
        assert!(invalid_result.is_err());

        let app_id = "test_app_id";
        let mut passage = PassageFlex::new(app_id.to_string(), api_key.to_string());
        passage.set_server_url(server.url());
        let m2 = server
            .mock("GET", "/v1/apps/test_app_id/")
            .with_status(200)
            .with_body(
                r#"{
            "app": {
                "additional_auth_origins": [],
                "allowed_callback_urls": [],
                "allowed_identifier": "external",
                "allowed_logout_urls": [],
                "application_login_uri": "",
                "auth_fallback_method": "none",
                "auth_fallback_method_ttl": 0,
                "auth_methods": {
                    "passkeys": { "enabled": false },
                    "otp": {
                        "enabled": false,
                        "ttl": 0,
                        "ttl_display_unit": "s"
                    },
                    "magic_link": {
                        "enabled": false,
                        "ttl": 0,
                        "ttl_display_unit": "s"
                    }
                },
                "auth_origin": "https://auth.test.com",
                "auto_theme_enabled": false,
                "created_at": "2021-01-01T00:00:00Z",
                "default_language": "en",
                "id": "test_app_id",
                "layouts": {
                    "profile": [],
                    "registration": []
                },
                "login_url": "",
                "name": "Test App",
                "hosted": false,
                "hosted_subdomain": "",
                "id_token_lifetime": null,
                "passage_branding": false,
                "profile_management": false,
                "public_signup": false,
                "redirect_url": "",
                "refresh_absolute_lifetime": 0,
                "refresh_enabled": false,
                "refresh_inactivity_lifetime": 0,
                "require_email_verification": false,
                "require_identifier_verification": false,
                "required_identifier": "",
                "role": "",
                "rsa_public_key": "",
                "secret": null,
                "session_timeout_length": 0,
                "type": "flex",
                "user_metadata_schema": [],
                "technologies": [],
                "element_customization": {},
                "element_customization_dark": {}
            }
        }"#,
            )
            .create_async()
            .await;

        let app_info = passage.get_app().await.unwrap();
        m2.assert_async().await;
        assert_eq!(app_info.id, "test_app_id");
        assert_eq!(app_info.name, "Test App");
        assert_eq!(app_info.auth_origin, "https://auth.test.com");
    }
}
