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
pub struct ElementCustomization {
    /// Container background color in hex. Default is `#ffffff` in light mode & `#383838` in dark mode. 
    #[serde(rename = "passage_container_background_color", skip_serializing_if = "Option::is_none")]
    pub passage_container_background_color: Option<String>,
    /// Maximum width of container (px)
    #[serde(rename = "passage_container_max_width", skip_serializing_if = "Option::is_none")]
    pub passage_container_max_width: Option<i32>,
    /// Input box background color in hex. Default is `#ffffff` in light mode & `#4b4b4b` in dark mode. 
    #[serde(rename = "passage_input_box_background_color", skip_serializing_if = "Option::is_none")]
    pub passage_input_box_background_color: Option<String>,
    /// Input box border radius (px)
    #[serde(rename = "passage_input_box_border_radius", skip_serializing_if = "Option::is_none")]
    pub passage_input_box_border_radius: Option<i32>,
    #[serde(rename = "passage_header_font_family", skip_serializing_if = "Option::is_none")]
    pub passage_header_font_family: Option<models::FontFamily>,
    #[serde(rename = "passage_body_font_family", skip_serializing_if = "Option::is_none")]
    pub passage_body_font_family: Option<models::FontFamily>,
    /// Header text color in hex. Default is `#222222` in light mode & `#f3f3f3` in dark mode. 
    #[serde(rename = "passage_header_text_color", skip_serializing_if = "Option::is_none")]
    pub passage_header_text_color: Option<String>,
    /// Body text color in hex. Default is `#222222` in light mode & `#f3f3f3` in dark mode. 
    #[serde(rename = "passage_body_text_color", skip_serializing_if = "Option::is_none")]
    pub passage_body_text_color: Option<String>,
    /// Primary button background colour (hex)
    #[serde(rename = "passage_primary_button_background_color", skip_serializing_if = "Option::is_none")]
    pub passage_primary_button_background_color: Option<String>,
    /// Primary button font colour (hex)
    #[serde(rename = "passage_primary_button_text_color", skip_serializing_if = "Option::is_none")]
    pub passage_primary_button_text_color: Option<String>,
    /// Primary button background on hover (hex)
    #[serde(rename = "passage_primary_button_hover_color", skip_serializing_if = "Option::is_none")]
    pub passage_primary_button_hover_color: Option<String>,
    /// Primary button border radius (px)
    #[serde(rename = "passage_primary_button_border_radius", skip_serializing_if = "Option::is_none")]
    pub passage_primary_button_border_radius: Option<i32>,
    /// Primary button border color
    #[serde(rename = "passage_primary_button_border_color", skip_serializing_if = "Option::is_none")]
    pub passage_primary_button_border_color: Option<String>,
    /// Primary button border width (px)
    #[serde(rename = "passage_primary_button_border_width", skip_serializing_if = "Option::is_none")]
    pub passage_primary_button_border_width: Option<i32>,
    /// Secondary button background colour (hex)
    #[serde(rename = "passage_secondary_button_background_color", skip_serializing_if = "Option::is_none")]
    pub passage_secondary_button_background_color: Option<String>,
    /// Secondary button font colour (hex)
    #[serde(rename = "passage_secondary_button_text_color", skip_serializing_if = "Option::is_none")]
    pub passage_secondary_button_text_color: Option<String>,
    /// Secondary button background on hover (hex)
    #[serde(rename = "passage_secondary_button_hover_color", skip_serializing_if = "Option::is_none")]
    pub passage_secondary_button_hover_color: Option<String>,
    /// Secondary button border radius (px)
    #[serde(rename = "passage_secondary_button_border_radius", skip_serializing_if = "Option::is_none")]
    pub passage_secondary_button_border_radius: Option<i32>,
    /// Secondary button border color
    #[serde(rename = "passage_secondary_button_border_color", skip_serializing_if = "Option::is_none")]
    pub passage_secondary_button_border_color: Option<String>,
    /// Secondary button border width (px)
    #[serde(rename = "passage_secondary_button_border_width", skip_serializing_if = "Option::is_none")]
    pub passage_secondary_button_border_width: Option<i32>,
}


