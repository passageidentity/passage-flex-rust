use mockito::{Matcher, Server, ServerGuard};
use passage_flex::passage_flex::PassageFlex;

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

    let m1 = server
        .mock("GET", format!("/v1/apps/{}/users/invalid", app_id).as_str())
        .with_status(404)
        .with_body(r#"{"error": "User not found","code": "user_not_found"}"#)
        .create_async()
        .await;

    let invalid_result = passage_flex.get_user("invalid".to_string()).await;
    m1.assert_async().await;
    assert!(invalid_result.is_err());

    let m2 = server
        .mock("GET", format!("/v1/apps/{}/users/valid", app_id).as_str())
        .with_status(200)
        .with_body(
            r#"{
            "user": {
                "created_at": "2021-01-01T00:00:00Z",
                "email": "",
                "email_verified": false,
                "external_id": "test_external_id",
                "id": "valid",
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
                "webauthn_devices": [],
                "webauthn_types": []
            }
        }"#,
        )
        .create_async()
        .await;

    let user_info = passage_flex.get_user("valid".to_string()).await.unwrap();
    m2.assert_async().await;
    assert_eq!(user_info.id, "valid");
}

#[tokio::test]
async fn test_get_devices() {
    let (app_id, passage_flex, mut server) = setup_passage_flex().await;

    let m1 = server
        .mock(
            "GET",
            format!("/v1/apps/{}/users/invalid/devices", app_id).as_str(),
        )
        .with_status(404)
        .with_body(r#"{"error": "User not found","code": "user_not_found"}"#)
        .create_async()
        .await;

    let invalid_result = passage_flex.get_devices("invalid".to_string()).await;
    m1.assert_async().await;
    assert!(invalid_result.is_err());

    let m2 = server
        .mock(
            "GET",
            format!("/v1/apps/{}/users/valid/devices", app_id).as_str(),
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
        .await;

    let devices = passage_flex.get_devices("valid".to_string()).await.unwrap();
    m2.assert_async().await;
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].id, "test_device_id");
}

#[tokio::test]
async fn test_revoke_device() {
    let (app_id, passage_flex, mut server) = setup_passage_flex().await;
    let m1 = server
        .mock(
            "DELETE",
            format!("/v1/apps/{}/users/invalid/devices/invalid", app_id).as_str(),
        )
        .with_status(404)
        .with_body(r#"{"error": "User not found","code": "user_not_found"}"#)
        .create_async()
        .await;

    let invalid_result = passage_flex
        .revoke_device("invalid".to_string(), "invalid".to_string())
        .await;
    m1.assert_async().await;
    assert!(invalid_result.is_err());

    let m2 = server
        .mock(
            "DELETE",
            format!("/v1/apps/{}/users/valid/devices/valid", app_id).as_str(),
        )
        .with_status(200)
        .create_async()
        .await;

    let result = passage_flex
        .revoke_device("valid".to_string(), "valid".to_string())
        .await;
    m2.assert_async().await;
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
