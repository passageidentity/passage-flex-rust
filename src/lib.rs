use std::fmt;

extern crate reqwest;
extern crate serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    InvalidRequest,
    InvalidAccessToken,
    InvalidNonce,
    OperationNotAllowed,
    DeviceNotFound,
    UserNotFound,
    UserHasNoPasskeys,
    InternalServerError,
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::InvalidRequest => ("response", "invalid request".to_string()),
            Error::InvalidAccessToken => ("response", "invalid access token".to_string()),
            Error::InvalidNonce => ("response", "invalid nonce".to_string()),
            Error::OperationNotAllowed => ("response", "operation not allowed".to_string()),
            Error::DeviceNotFound => ("response", "device not found".to_string()),
            Error::UserNotFound => ("response", "user not found".to_string()),
            Error::UserHasNoPasskeys => ("response", "user has no passkeys".to_string()),
            Error::InternalServerError => ("response", "internal server error".to_string()),
            Error::Other(e) => ("response", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

mod error;

pub mod passage_flex;
pub use openapi::apis::transactions_api::CreateRegisterTransactionError;
pub use passage_flex::PassageFlex;
