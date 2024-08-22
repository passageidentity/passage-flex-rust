/*
 * Passage Management API
 *
 * Passage's management API to manage your Passage apps and users.
 *
 * The version of the OpenAPI document: 1
 * Contact: support@passage.id
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPaginatedUsersItem {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
    /// The external ID of the user. Only set if the user was created in a Flex app.
    #[serde(rename = "external_id")]
    pub external_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "last_login_at")]
    pub last_login_at: String,
    #[serde(rename = "login_count")]
    pub login_count: i32,
    #[serde(rename = "phone")]
    pub phone: String,
    #[serde(rename = "phone_verified")]
    pub phone_verified: bool,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "user_metadata", deserialize_with = "Option::deserialize")]
    pub user_metadata: Option<serde_json::Value>,
}
