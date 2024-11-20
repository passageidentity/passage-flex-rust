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
pub struct AppInfo {
    #[serde(rename = "additional_auth_origins")]
    pub additional_auth_origins: Vec<String>,
    /// The valid URLs where users can be redirected after authentication.
    #[serde(rename = "allowed_callback_urls")]
    pub allowed_callback_urls: Vec<String>,
    #[serde(rename = "allowed_identifier")]
    pub allowed_identifier: String,
    /// The valid URLs where users can be redirected after logging out.
    #[serde(rename = "allowed_logout_urls")]
    pub allowed_logout_urls: Vec<String>,
    /// A route within your application that redirects to the Authorization URL endpoint.
    #[serde(rename = "application_login_uri")]
    pub application_login_uri: String,
    /// Deprecated Property. Please refer to `auth_methods` to view settings for individual authentication methods.
    #[serde(rename = "auth_fallback_method")]
    pub auth_fallback_method: String,
    /// Deprecated Property. Please refer to `auth_methods` to view settings for individual authentication methods.
    #[serde(rename = "auth_fallback_method_ttl")]
    pub auth_fallback_method_ttl: i32,
    #[serde(rename = "auth_methods")]
    pub auth_methods: Box<models::AuthMethods>,
    #[serde(rename = "auth_origin")]
    pub auth_origin: String,
    /// Deprecated Property. Please use `hosted_theme` to set hosted page theming instead.
    #[serde(rename = "auto_theme_enabled")]
    pub auto_theme_enabled: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "default_language")]
    pub default_language: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "layouts")]
    pub layouts: Box<models::Layouts>,
    #[serde(rename = "login_url")]
    pub login_url: String,
    #[serde(rename = "light_logo_url", skip_serializing_if = "Option::is_none")]
    pub light_logo_url: Option<String>,
    #[serde(rename = "dark_logo_url", skip_serializing_if = "Option::is_none")]
    pub dark_logo_url: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    /// whether or not the app's login page is hosted by Passage
    #[serde(rename = "hosted")]
    pub hosted: bool,
    /// the subdomain of the app's hosted login page
    #[serde(rename = "hosted_subdomain")]
    pub hosted_subdomain: String,
    #[serde(rename = "hosted_theme")]
    pub hosted_theme: models::ThemeType,
    #[serde(rename = "id_token_lifetime", skip_serializing_if = "Option::is_none")]
    pub id_token_lifetime: Option<i32>,
    #[serde(rename = "passage_branding")]
    pub passage_branding: bool,
    #[serde(rename = "profile_management")]
    pub profile_management: bool,
    #[serde(rename = "public_signup")]
    pub public_signup: bool,
    #[serde(rename = "redirect_url")]
    pub redirect_url: String,
    #[serde(rename = "refresh_absolute_lifetime")]
    pub refresh_absolute_lifetime: i32,
    #[serde(rename = "refresh_enabled")]
    pub refresh_enabled: bool,
    #[serde(rename = "refresh_inactivity_lifetime")]
    pub refresh_inactivity_lifetime: i32,
    #[serde(rename = "require_email_verification")]
    pub require_email_verification: bool,
    #[serde(rename = "require_identifier_verification")]
    pub require_identifier_verification: bool,
    #[serde(rename = "required_identifier")]
    pub required_identifier: String,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "rsa_public_key")]
    pub rsa_public_key: String,
    /// can only be retrieved by an app admin
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "session_timeout_length")]
    pub session_timeout_length: i32,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "user_metadata_schema")]
    pub user_metadata_schema: Vec<models::UserMetadataField>,
    #[serde(rename = "technologies")]
    pub technologies: Vec<models::Technologies>,
    #[serde(rename = "element_customization")]
    pub element_customization: Box<models::ElementCustomization>,
    #[serde(rename = "element_customization_dark")]
    pub element_customization_dark: Box<models::ElementCustomization>,
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "flex")]
    Flex,
}


