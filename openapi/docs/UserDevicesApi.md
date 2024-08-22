# \UserDevicesApi

All URIs are relative to *https://api.passage.id/v1/apps/TODO*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user_devices**](UserDevicesApi.md#delete_user_devices) | **DELETE** /users/{user_id}/devices/{device_id} | Delete a device for a user
[**list_user_devices**](UserDevicesApi.md#list_user_devices) | **GET** /users/{user_id}/devices | List User Devices



## delete_user_devices

> delete_user_devices(user_id, device_id)
Delete a device for a user

Delete a device for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**device_id** | **String** | Device ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_devices

> models::ListDevicesResponse list_user_devices(user_id)
List User Devices

List user devices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**models::ListDevicesResponse**](ListDevicesResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

