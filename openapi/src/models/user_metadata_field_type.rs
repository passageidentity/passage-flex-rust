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

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserMetadataFieldType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "email")]
    Email,
}

impl std::fmt::Display for UserMetadataFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Boolean => write!(f, "boolean"),
            Self::Integer => write!(f, "integer"),
            Self::Date => write!(f, "date"),
            Self::Phone => write!(f, "phone"),
            Self::Email => write!(f, "email"),
        }
    }
}

impl Default for UserMetadataFieldType {
    fn default() -> UserMetadataFieldType {
        Self::String
    }
}
