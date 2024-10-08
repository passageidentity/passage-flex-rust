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
pub struct UserMetadataField {
    #[serde(rename = "field_name")]
    pub field_name: String,
    #[serde(rename = "friendly_name")]
    pub friendly_name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "profile")]
    pub profile: bool,
    #[serde(rename = "registration")]
    pub registration: bool,
    #[serde(rename = "type")]
    pub r#type: models::UserMetadataFieldType,
}
