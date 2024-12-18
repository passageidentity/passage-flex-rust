use crate::models::PassageUser;
use crate::openapi::apis::configuration::Configuration;
use crate::openapi::apis::{user_devices_api, users_api};
use crate::Error;

pub struct User {
    pub(crate) configuration: Configuration,
}

impl User {
    /// Creates a new instance of the `User` struct.
    pub fn new(configuration: Configuration) -> Self {
        Self { configuration }
    }

    /// Get a user's ID in Passage by their external ID
    async fn get_id(&self, external_id: String) -> Result<String, Error> {
        let users = users_api::list_paginated_users(
            &self.configuration,
            Some(1),
            Some(1),
            None,
            None,
            Some(&external_id),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .map(|response| response.users)
        .map_err(Into::into);

        match users {
            Ok(mut users) => match users.len() {
                0 => Err(Error::UserNotFound),
                1 => {
                    let user = users.remove(0);
                    Ok(user.id)
                }
                _ => Err(Error::Other("Multiple users found".to_string())),
            },
            Err(e) => Err(e),
        }
    }

    /// Retrieves information about a user by their external ID.
    ///
    /// # Arguments
    ///
    /// * `external_id` - The unique, immutable ID that represents the user.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `UserInfo` struct or an `Error`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use passage_flex::PassageFlex;
    ///
    /// let passage_flex = PassageFlex::new(
    ///     std::env::var("PASSAGE_APP_ID").unwrap(),
    ///     std::env::var("PASSAGE_API_KEY").unwrap(),
    /// );
    ///
    /// let external_id = "00000000-0000-0000-0000-000000000001";
    /// let passage_user = passage_flex.user.get(external_id.to_string()).await.unwrap();
    /// println!("{:?}", passage_user.id);
    /// ```
    pub async fn get(&self, external_id: String) -> Result<Box<PassageUser>, Error> {
        let user_id = self.get_id(external_id).await?;
        users_api::get_user(&self.configuration, &user_id)
            .await
            .map(|response| {
                Box::new(PassageUser {
                    created_at: response.user.created_at,
                    external_id: response.user.external_id,
                    id: response.user.id,
                    last_login_at: response.user.last_login_at,
                    login_count: response.user.login_count,
                    status: response.user.status,
                    updated_at: response.user.updated_at,
                    user_metadata: response.user.user_metadata,
                    webauthn: response.user.webauthn,
                    webauthn_devices: response.user.webauthn_devices,
                    webauthn_types: response.user.webauthn_types,
                })
            })
            .map_err(Into::into)
    }

    /// Retrieves information about a user's passkey devices.
    ///
    /// # Arguments
    ///
    /// * `external_id` - The unique, immutable ID that represents the user.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `WebAuthnDevices` or an `Error`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use passage_flex::PassageFlex;
    ///
    /// let passage_flex = PassageFlex::new(
    ///     std::env::var("PASSAGE_APP_ID").unwrap(),
    ///     std::env::var("PASSAGE_API_KEY").unwrap(),
    /// );
    ///
    /// let external_id = "00000000-0000-0000-0000-000000000001";
    /// let passkey_devices = passage_flex.user.list_devices(external_id.to_string()).await.unwrap();
    /// for device in passkey_devices {
    ///     println!("{}", device.usage_count);
    /// }
    /// ```
    pub async fn list_devices(
        &self,
        external_id: String,
    ) -> Result<Vec<crate::openapi::models::WebAuthnDevices>, Error> {
        let user_id = self.get_id(external_id).await?;
        user_devices_api::list_user_devices(&self.configuration, &user_id)
            .await
            .map(|response| response.devices)
            .map_err(Into::into)
    }

    /// Revokes a user's passkey device.
    ///
    /// # Arguments
    ///
    /// * `external_id` - The unique, immutable ID that represents the user.
    /// * `device_id` - The ID of the device to be revoked.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` or an `Error`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use passage_flex::PassageFlex;
    /// use chrono::{Duration, NaiveDate, Utc};
    ///
    /// let passage_flex = PassageFlex::new(
    ///     std::env::var("PASSAGE_APP_ID").unwrap(),
    ///     std::env::var("PASSAGE_API_KEY").unwrap(),
    /// );
    ///
    /// let external_id = "00000000-0000-0000-0000-000000000001";
    /// let last_year = Utc::now().naive_utc().date() - Duration::days(365);
    ///
    /// let passkey_devices = passage_flex.user.list_devices(external_id.to_string()).await.unwrap();
    ///
    /// for device in passkey_devices {
    ///     let last_login_at_parsed =
    ///         NaiveDate::parse_from_str(&device.last_login_at, "%Y-%m-%dT%H:%M:%S%z").unwrap();
    ///
    ///     if last_login_at_parsed < last_year {
    ///         if let Err(err) = passage_flex
    ///             .user
    ///             .revoke_device(external_id.clone(), device.id)
    ///             .await
    ///         {
    ///             // device couldn't be revoked
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn revoke_device(&self, external_id: String, device_id: String) -> Result<(), Error> {
        let user_id = self.get_id(external_id).await?;
        user_devices_api::delete_user_devices(&self.configuration, &user_id, &device_id)
            .await
            .map_err(Into::into)
    }
}
