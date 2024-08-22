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
pub struct OtpAuthMethod {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Maximum time (IN SECONDS) for the auth to expire.
    #[serde(rename = "ttl")]
    pub ttl: i32,
    #[serde(rename = "ttl_display_unit")]
    pub ttl_display_unit: models::TtlDisplayUnit,
}
