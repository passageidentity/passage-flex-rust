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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRecentEvent {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ip_addr")]
    pub ip_addr: String,
    #[serde(rename = "status")]
    pub status: models::UserEventStatus,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "user_agent")]
    pub user_agent: String,
}

impl UserRecentEvent {
    pub fn new(
        created_at: String,
        completed_at: Option<String>,
        id: String,
        ip_addr: String,
        status: models::UserEventStatus,
        r#type: String,
        user_agent: String,
    ) -> UserRecentEvent {
        UserRecentEvent {
            created_at,
            completed_at,
            id,
            ip_addr,
            status,
            r#type,
            user_agent,
        }
    }
}
