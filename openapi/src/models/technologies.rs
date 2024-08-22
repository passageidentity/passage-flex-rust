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
pub enum Technologies {
    #[serde(rename = "react")]
    React,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "vue")]
    Vue,
    #[serde(rename = "angular")]
    Angular,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "javascript")]
    Javascript,
    #[serde(rename = "ios")]
    Ios,
    #[serde(rename = "android")]
    Android,
}

impl std::fmt::Display for Technologies {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::React => write!(f, "react"),
            Self::Go => write!(f, "go"),
            Self::Vue => write!(f, "vue"),
            Self::Angular => write!(f, "angular"),
            Self::Python => write!(f, "python"),
            Self::Javascript => write!(f, "javascript"),
            Self::Ios => write!(f, "ios"),
            Self::Android => write!(f, "android"),
        }
    }
}

impl Default for Technologies {
    fn default() -> Technologies {
        Self::React
    }
}
