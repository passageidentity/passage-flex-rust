# \AuthenticateApi

All URIs are relative to *https://api.passage.id/v1/apps/TODO*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authenticate_verify_nonce**](AuthenticateApi.md#authenticate_verify_nonce) | **POST** /authenticate/verify | Verify the nonce received from a WebAuthn ceremony



## authenticate_verify_nonce

> models::AuthenticateVerifyNonceResponse authenticate_verify_nonce(body)
Verify the nonce received from a WebAuthn ceremony

Verify the nonce received from a WebAuthn registration or authentication ceremony. This endpoint checks that the nonce for the given application is valid, then returns a success or error message to the caller.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **models::Nonce** | User Data | [required] |

### Return type

[**models::AuthenticateVerifyNonceResponse**](AuthenticateVerifyNonceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

