/*
 * Passage Management API
 *
 * Passage's management API to manage your Passage apps and users.
 *
 * The version of the OpenAPI document: 1
 * Contact: support@passage.id
 * Generated by: https://openapi-generator.tech
 */

use crate::openapi::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppleUserSocialConnection {
    /// The external ID of the Social Connection.
    #[serde(rename = "provider_id")]
    pub provider_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "last_login_at")]
    pub last_login_at: String,
    /// The email of connected social user.
    #[serde(rename = "provider_identifier")]
    pub provider_identifier: String,
}