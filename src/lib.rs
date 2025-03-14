//! This crate provides a Rust SDK for Passage Passkey Flex, a service that enables server-side
//! authentication for applications using WebAuthn passkeys.
//!
//! # Examples
//!
//! ```ignore
//! use passage_flex::PassageFlex;
//!
//! let passage_flex = PassageFlex::new(
//!     std::env::var("PASSAGE_APP_ID").unwrap(),
//!     std::env::var("PASSAGE_API_KEY").unwrap(),
//! );
//! ```

use std::fmt;

extern crate reqwest;
extern crate serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    InvalidRequest(String),
    InvalidAccessToken,
    InvalidNonce,
    OperationNotAllowed,
    DeviceNotFound,
    UserNotFound,
    UserHasNoPasskeys,
    InternalServerError,
    Other(String),
    InvalidArgument(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::InvalidRequest(error) => ("response", format!("invalid request: {}", error)),
            Error::InvalidAccessToken => ("response", "invalid access token".to_string()),
            Error::InvalidNonce => ("response", "invalid nonce".to_string()),
            Error::OperationNotAllowed => ("response", "operation not allowed".to_string()),
            Error::DeviceNotFound => ("response", "device not found".to_string()),
            Error::UserNotFound => ("response", "user not found".to_string()),
            Error::UserHasNoPasskeys => ("response", "user has no passkeys".to_string()),
            Error::InternalServerError => ("response", "internal server error".to_string()),
            Error::Other(e) => ("response", e.to_string()),
            Error::InvalidArgument(e) => ("argument", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

mod error;
pub mod models;

#[rustfmt::skip]
pub mod openapi;

pub mod auth;
pub mod passage_flex;
pub mod user;
pub use passage_flex::PassageFlex;
