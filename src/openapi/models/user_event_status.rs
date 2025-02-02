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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserEventStatus {
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "incomplete")]
    Incomplete,

}

impl std::fmt::Display for UserEventStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Complete => write!(f, "complete"),
            Self::Incomplete => write!(f, "incomplete"),
        }
    }
}


