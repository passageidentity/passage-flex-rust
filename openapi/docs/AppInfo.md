# AppInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_auth_origins** | **Vec<String>** |  | 
**allowed_callback_urls** | **Vec<String>** | The valid URLs where users can be redirected after authentication. | 
**allowed_identifier** | **String** |  | 
**allowed_logout_urls** | **Vec<String>** | The valid URLs where users can be redirected after logging out. | 
**application_login_uri** | **String** | A route within your application that redirects to the Authorization URL endpoint. | 
**auth_fallback_method** | **String** | Deprecated Property. Please refer to `auth_methods` to view settings for individual authentication methods. | 
**auth_fallback_method_ttl** | **i32** | Deprecated Property. Please refer to `auth_methods` to view settings for individual authentication methods. | 
**auth_methods** | [**models::AuthMethods**](AuthMethods.md) |  | 
**auth_origin** | **String** |  | 
**auto_theme_enabled** | **bool** |  | 
**created_at** | **String** |  | 
**default_language** | **String** |  | 
**id** | **String** |  | 
**layouts** | [**models::Layouts**](Layouts.md) |  | 
**login_url** | **String** |  | 
**light_logo_url** | Option<**String**> |  | [optional]
**dark_logo_url** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**hosted** | **bool** | whether or not the app's login page hosted by passage | 
**hosted_subdomain** | **String** | the subdomain of the app's hosted login page | 
**id_token_lifetime** | Option<**i32**> |  | [optional]
**passage_branding** | **bool** |  | 
**profile_management** | **bool** |  | 
**public_signup** | **bool** |  | 
**redirect_url** | **String** |  | 
**refresh_absolute_lifetime** | **i32** |  | 
**refresh_enabled** | **bool** |  | 
**refresh_inactivity_lifetime** | **i32** |  | 
**require_email_verification** | **bool** |  | 
**require_identifier_verification** | **bool** |  | 
**required_identifier** | **String** |  | 
**role** | **String** |  | 
**rsa_public_key** | **String** |  | 
**secret** | Option<**String**> | can only be retrieved by an app admin | [optional]
**session_timeout_length** | **i32** |  | 
**r#type** | **String** |  | 
**user_metadata_schema** | [**Vec<models::UserMetadataField>**](UserMetadataField.md) |  | 
**technologies** | [**Vec<models::Technologies>**](Technologies.md) |  | 
**element_customization** | [**models::ElementCustomization**](ElementCustomization.md) |  | 
**element_customization_dark** | [**models::ElementCustomization**](ElementCustomization.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


