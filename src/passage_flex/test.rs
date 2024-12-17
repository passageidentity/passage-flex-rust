use crate::Error;

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
        .auth
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
        .auth
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

    let invalid_result = passage_flex.auth.verify_nonce("invalid".to_string()).await;
    assert!(invalid_result.is_err());
    assert!(matches!(invalid_result, Err(Error::InvalidNonce)));
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
        .auth
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
    let invalid_result = passage_flex.user.get("invalid".to_string()).await;
    m1.assert_async().await;
    assert!(invalid_result.is_err());
    assert!(matches!(invalid_result, Err(Error::UserNotFound)));

    let m2 = setup_valid_list_paginated_users_mock(&app_id, &mut server).await;
    let m3 = setup_valid_get_user_mock(&app_id, &mut server).await;

    let user_info = passage_flex.user.get("valid".to_string()).await.unwrap();
    m2.assert_async().await;
    m3.assert_async().await;
    assert_eq!(user_info.external_id, "valid");
}

#[tokio::test]
async fn test_get_devices() {
    let (app_id, passage_flex, mut server) = setup_passage_flex().await;

    let m1 = setup_empty_list_paginated_users_mock(&app_id, &mut server).await;
    let invalid_result = passage_flex.user.list_devices("invalid".to_string()).await;
    m1.assert_async().await;
    assert!(invalid_result.is_err());
    assert!(matches!(invalid_result, Err(Error::UserNotFound)));

    let m2 = setup_valid_list_paginated_users_mock(&app_id, &mut server).await;
    let m3 = setup_valid_get_devices_mock(&app_id, &mut server).await;
    let devices = passage_flex
        .user
        .list_devices("valid".to_string())
        .await
        .unwrap();
    m2.assert_async().await;
    m3.assert_async().await;
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].id, "test_device_id");
}

#[tokio::test]
async fn test_revoke_device() {
    let (app_id, passage_flex, mut server) = setup_passage_flex().await;

    let m1 = setup_empty_list_paginated_users_mock(&app_id, &mut server).await;
    let invalid_result = passage_flex
        .user
        .revoke_device("invalid".to_string(), "invalid".to_string())
        .await;
    m1.assert_async().await;
    assert!(invalid_result.is_err());
    assert!(matches!(invalid_result, Err(Error::UserNotFound)));

    let m2 = setup_valid_list_paginated_users_mock(&app_id, &mut server).await;
    let m3 = setup_valid_revoke_device_mock(&app_id, &mut server).await;
    let result = passage_flex
        .user
        .revoke_device("valid".to_string(), "test_device_id".to_string())
        .await;
    m2.assert_async().await;
    m3.assert_async().await;
    assert!(result.is_ok());
}
