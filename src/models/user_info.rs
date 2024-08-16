use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The external ID of the user. Only set if the user was created in a Flex app.
    #[serde(rename = "external_id")]
    pub external_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "last_login_at")]
    pub last_login_at: String,
    #[serde(rename = "login_count")]
    pub login_count: i32,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "user_metadata", deserialize_with = "Option::deserialize")]
    pub user_metadata: Option<serde_json::Value>,
    #[serde(rename = "webauthn")]
    pub webauthn: bool,
    #[serde(rename = "webauthn_devices")]
    pub webauthn_devices: Vec<models::WebAuthnDevices>,
    /// List of credential types that have been used for authentication
    #[serde(rename = "webauthn_types")]
    pub webauthn_types: Vec<models::WebAuthnType>,
}
