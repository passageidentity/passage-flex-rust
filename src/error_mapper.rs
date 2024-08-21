use openapi::apis::{user_devices_api, users_api};
use openapi::apis::{Error, ResponseContent};
use openapi::models;

// Define a generic function to map response entities between different error types.
fn map_response_entity<Src, Dst>(entity: Option<Src>, map_fn: fn(Src) -> Dst) -> Option<Dst> {
    entity.map(map_fn)
}

// Define a helper function to convert common Error variants.
fn convert_error<Src, Dst>(error: Error<Src>, map_fn: fn(Src) -> Dst) -> Error<Dst> {
    match error {
        Error::Reqwest(e) => Error::Reqwest(e), // Forward the reqwest error directly
        Error::Serde(e) => Error::Serde(e),     // Forward the serde error directly
        Error::Io(e) => Error::Io(e),           // Forward the I/O error directly
        Error::ResponseError(response) => {
            let mapped_entity = map_response_entity(response.entity, map_fn);

            Error::ResponseError(ResponseContent {
                status: response.status,
                content: response.content,
                entity: mapped_entity,
            })
        }
    }
}

pub(crate) fn user_not_found_get_user_error() -> Error<users_api::GetUserError> {
    let model = models::Model404Error::new(
        models::model_404_error::Code::UserNotFound,
        "User not found".to_string(),
    );

    Error::ResponseError(ResponseContent {
        status: reqwest::StatusCode::NOT_FOUND,
        content: "".to_string(),
        entity: Some(users_api::GetUserError::Status404(model)),
    })
}

pub(crate) fn internal_server_error_get_user_error() -> Error<users_api::GetUserError> {
    let model = models::Model500Error::new(
        models::model_500_error::Code::InternalServerError,
        "unexpected error".to_string());

    Error::ResponseError(ResponseContent {
        status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        content: "".to_string(),
        entity: Some(users_api::GetUserError::Status500(model)),
    })
}

// Function to map ListPaginatedUsersError to GetUserError
fn map_list_to_get_user_error(
    error: users_api::ListPaginatedUsersError,
) -> users_api::GetUserError {
    match error {
        users_api::ListPaginatedUsersError::Status400(model) => {
            users_api::GetUserError::Status404(models::Model404Error::new(
                models::model_404_error::Code::UserNotFound,
                model.error.clone(),
            ))
        }
        users_api::ListPaginatedUsersError::Status401(model) => {
            users_api::GetUserError::Status401(model)
        }
        users_api::ListPaginatedUsersError::Status404(model) => {
            users_api::GetUserError::Status404(model)
        }
        users_api::ListPaginatedUsersError::Status500(model) => {
            users_api::GetUserError::Status500(model)
        }
        users_api::ListPaginatedUsersError::UnknownValue(value) => {
            users_api::GetUserError::UnknownValue(value)
        }
    }
}

// Public function to convert ListPaginatedUsersError to GetUserError
pub(crate) fn convert_list_error_to_get_user_error(
    error: Error<users_api::ListPaginatedUsersError>,
) -> Error<users_api::GetUserError> {
    convert_error(error, map_list_to_get_user_error)
}

// Function to map GetUserError to ListUserDevicesError
fn map_get_user_to_list_user_devices_error(
    error: users_api::GetUserError,
) -> user_devices_api::ListUserDevicesError {
    match error {
        users_api::GetUserError::Status401(model) => {
            user_devices_api::ListUserDevicesError::Status401(model)
        }
        users_api::GetUserError::Status404(model) => {
            user_devices_api::ListUserDevicesError::Status404(model)
        }
        users_api::GetUserError::Status500(model) => {
            user_devices_api::ListUserDevicesError::Status500(model)
        }
        users_api::GetUserError::UnknownValue(value) => {
            user_devices_api::ListUserDevicesError::UnknownValue(value)
        }
    }
}

// Public function to convert GetUserError to ListUserDevicesError
pub(crate) fn convert_get_user_error_to_list_user_devices_error(
    error: Error<users_api::GetUserError>,
) -> Error<user_devices_api::ListUserDevicesError> {
    convert_error(error, map_get_user_to_list_user_devices_error)
}

// Function to map GetUserError to DeleteUserDevicesError
fn map_get_user_to_delete_user_devices_error(
    error: users_api::GetUserError,
) -> user_devices_api::DeleteUserDevicesError {
    match error {
        users_api::GetUserError::Status401(model) => {
            user_devices_api::DeleteUserDevicesError::Status401(model)
        }
        users_api::GetUserError::Status404(model) => {
            user_devices_api::DeleteUserDevicesError::Status404(model)
        }
        users_api::GetUserError::Status500(model) => {
            user_devices_api::DeleteUserDevicesError::Status500(model)
        }
        users_api::GetUserError::UnknownValue(value) => {
            user_devices_api::DeleteUserDevicesError::UnknownValue(value)
        }
    }
}

// Public function to convert GetUserError to DeleteUserDevicesError
pub(crate) fn convert_get_user_error_to_delete_user_devices_error(
    error: Error<users_api::GetUserError>,
) -> Error<user_devices_api::DeleteUserDevicesError> {
    convert_error(error, map_get_user_to_delete_user_devices_error)
}
