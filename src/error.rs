use crate::openapi::apis::{
    apps_api, authenticate_api, transactions_api, user_devices_api, users_api,
};
use crate::Error;

// This function converts an openapi error into a crate error
fn convert_error<Src>(error: crate::openapi::apis::Error<Src>, map_fn: fn(Src) -> Error) -> Error {
    match error {
        crate::openapi::apis::Error::Reqwest(e) => Error::Reqwest(e), // Forward the reqwest error directly
        crate::openapi::apis::Error::Serde(e) => Error::Serde(e), // Forward the serde error directly
        crate::openapi::apis::Error::Io(e) => Error::Io(e),       // Forward the I/O error directly
        crate::openapi::apis::Error::ResponseError(response) => match response.entity {
            Some(entity) => map_fn(entity),
            None => Error::Other(response.content),
        },
    }
}

impl From<crate::openapi::apis::Error<apps_api::GetAppError>> for Error {
    fn from(e: crate::openapi::apis::Error<apps_api::GetAppError>) -> Self {
        convert_error(e, |e| match e {
            apps_api::GetAppError::Status401(model) => model.into(),
            apps_api::GetAppError::Status404(model) => model.into(),
            apps_api::GetAppError::Status500(model) => model.into(),
            apps_api::GetAppError::UnknownValue(v) => Error::Other(v.to_string()),
        })
    }
}

impl From<crate::openapi::apis::Error<transactions_api::CreateRegisterTransactionError>> for Error {
    fn from(
        e: crate::openapi::apis::Error<transactions_api::CreateRegisterTransactionError>,
    ) -> Self {
        convert_error(e, |e| match e {
            transactions_api::CreateRegisterTransactionError::Status400(model) => model.into(),
            transactions_api::CreateRegisterTransactionError::Status401(model) => model.into(),
            transactions_api::CreateRegisterTransactionError::Status403(model) => model.into(),
            transactions_api::CreateRegisterTransactionError::Status404(model) => model.into(),
            transactions_api::CreateRegisterTransactionError::Status500(model) => model.into(),
            transactions_api::CreateRegisterTransactionError::UnknownValue(v) => {
                Error::Other(v.to_string())
            }
        })
    }
}

impl From<crate::openapi::apis::Error<transactions_api::CreateAuthenticateTransactionError>>
    for Error
{
    fn from(
        e: crate::openapi::apis::Error<transactions_api::CreateAuthenticateTransactionError>,
    ) -> Self {
        convert_error(e, |e| match e {
            transactions_api::CreateAuthenticateTransactionError::Status400(model) => model.into(),
            transactions_api::CreateAuthenticateTransactionError::Status401(model) => model.into(),
            transactions_api::CreateAuthenticateTransactionError::Status403(model) => model.into(),
            transactions_api::CreateAuthenticateTransactionError::Status404(model) => model.into(),
            transactions_api::CreateAuthenticateTransactionError::Status409(model) => model.into(),
            transactions_api::CreateAuthenticateTransactionError::Status500(model) => model.into(),
            transactions_api::CreateAuthenticateTransactionError::UnknownValue(v) => {
                Error::Other(v.to_string())
            }
        })
    }
}

impl From<crate::openapi::apis::Error<authenticate_api::AuthenticateVerifyNonceError>> for Error {
    fn from(
        e: crate::openapi::apis::Error<authenticate_api::AuthenticateVerifyNonceError>,
    ) -> Self {
        convert_error(e, |e| match e {
            authenticate_api::AuthenticateVerifyNonceError::Status400(model) => model.into(),
            authenticate_api::AuthenticateVerifyNonceError::Status401(model) => model.into(),
            authenticate_api::AuthenticateVerifyNonceError::Status403(model) => model.into(),
            authenticate_api::AuthenticateVerifyNonceError::Status404(model) => model.into(),
            authenticate_api::AuthenticateVerifyNonceError::Status500(model) => model.into(),
            authenticate_api::AuthenticateVerifyNonceError::UnknownValue(v) => {
                Error::Other(v.to_string())
            }
        })
    }
}

impl From<crate::openapi::apis::Error<users_api::ListPaginatedUsersError>> for Error {
    fn from(e: crate::openapi::apis::Error<users_api::ListPaginatedUsersError>) -> Self {
        convert_error(e, |e| match e {
            users_api::ListPaginatedUsersError::Status400(model) => model.into(),
            users_api::ListPaginatedUsersError::Status401(model) => model.into(),
            users_api::ListPaginatedUsersError::Status404(model) => model.into(),
            users_api::ListPaginatedUsersError::Status500(model) => model.into(),
            users_api::ListPaginatedUsersError::UnknownValue(v) => Error::Other(v.to_string()),
        })
    }
}

impl From<crate::openapi::apis::Error<users_api::GetUserError>> for Error {
    fn from(e: crate::openapi::apis::Error<users_api::GetUserError>) -> Self {
        convert_error(e, |e| match e {
            users_api::GetUserError::Status401(model) => model.into(),
            users_api::GetUserError::Status404(model) => model.into(),
            users_api::GetUserError::Status500(model) => model.into(),
            users_api::GetUserError::UnknownValue(v) => Error::Other(v.to_string()),
        })
    }
}

impl From<crate::openapi::apis::Error<user_devices_api::ListUserDevicesError>> for Error {
    fn from(e: crate::openapi::apis::Error<user_devices_api::ListUserDevicesError>) -> Self {
        convert_error(e, |e| match e {
            user_devices_api::ListUserDevicesError::Status401(model) => model.into(),
            user_devices_api::ListUserDevicesError::Status404(model) => model.into(),
            user_devices_api::ListUserDevicesError::Status500(model) => model.into(),
            user_devices_api::ListUserDevicesError::UnknownValue(v) => Error::Other(v.to_string()),
        })
    }
}

impl From<crate::openapi::apis::Error<user_devices_api::DeleteUserDevicesError>> for Error {
    fn from(e: crate::openapi::apis::Error<user_devices_api::DeleteUserDevicesError>) -> Self {
        convert_error(e, |e| match e {
            user_devices_api::DeleteUserDevicesError::Status401(model) => model.into(),
            user_devices_api::DeleteUserDevicesError::Status404(model) => model.into(),
            user_devices_api::DeleteUserDevicesError::Status500(model) => model.into(),
            user_devices_api::DeleteUserDevicesError::UnknownValue(v) => {
                Error::Other(v.to_string())
            }
        })
    }
}

impl From<crate::openapi::models::model_401_error::Model401Error> for Error {
    fn from(e: crate::openapi::models::model_401_error::Model401Error) -> Self {
        match e.code {
            crate::openapi::models::model_401_error::Code::AccessToken => Error::InvalidAccessToken,
            crate::openapi::models::model_401_error::Code::Nonce => Error::InvalidNonce,
        }
    }
}

impl From<crate::openapi::models::model_400_error::Model400Error> for Error {
    fn from(e: crate::openapi::models::model_400_error::Model400Error) -> Self {
        match e.code {
            crate::openapi::models::model_400_error::Code::InvalidRequest => Error::InvalidRequest,
            _ => Error::Other(e.error),
        }
    }
}

impl From<crate::openapi::models::model_403_error::Model403Error> for Error {
    fn from(e: crate::openapi::models::model_403_error::Model403Error) -> Self {
        match e.code {
            crate::openapi::models::model_403_error::Code::OperationNotAllowed => {
                Error::OperationNotAllowed
            }
            _ => Error::Other(e.error),
        }
    }
}

impl From<crate::openapi::models::model_404_error::Model404Error> for Error {
    fn from(e: crate::openapi::models::model_404_error::Model404Error) -> Self {
        match e.code {
            crate::openapi::models::model_404_error::Code::DeviceNotFound => Error::DeviceNotFound,
            crate::openapi::models::model_404_error::Code::UserNotFound => Error::UserNotFound,
            _ => Error::Other(e.error),
        }
    }
}

impl From<crate::openapi::models::model_409_error::Model409Error> for Error {
    fn from(e: crate::openapi::models::model_409_error::Model409Error) -> Self {
        match e.code {
            crate::openapi::models::model_409_error::Code::HasNoPasskeys => {
                Error::UserHasNoPasskeys
            }
            _ => Error::Other(e.error),
        }
    }
}

impl From<crate::openapi::models::model_500_error::Model500Error> for Error {
    fn from(_: crate::openapi::models::model_500_error::Model500Error) -> Self {
        Error::InternalServerError
    }
}
