# UserInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  | 
**email** | **String** |  | 
**email_verified** | **bool** |  | 
**external_id** | **String** | The external ID of the user. Only set if the user was created in a Flex app. | 
**id** | **String** |  | 
**last_login_at** | **String** |  | 
**login_count** | **i32** |  | 
**phone** | **String** |  | 
**phone_verified** | **bool** |  | 
**recent_events** | [**Vec<models::UserRecentEvent>**](UserRecentEvent.md) |  | 
**social_connections** | [**models::UserSocialConnections**](UserSocialConnections.md) |  | 
**status** | [**models::UserStatus**](UserStatus.md) |  | 
**updated_at** | **String** |  | 
**user_metadata** | Option<[**serde_json::Value**](.md)> |  | 
**webauthn** | **bool** |  | 
**webauthn_devices** | [**Vec<models::WebAuthnDevices>**](WebAuthnDevices.md) |  | 
**webauthn_types** | [**Vec<models::WebAuthnType>**](WebAuthnType.md) | List of credential types that have been used for authentication | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


