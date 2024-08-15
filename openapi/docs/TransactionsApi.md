# \TransactionsApi

All URIs are relative to *https://api.passage.id/v1/apps/TODO*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_authenticate_transaction**](TransactionsApi.md#create_authenticate_transaction) | **POST** /transactions/authenticate | Create a transaction to start a user's authentication process
[**create_register_transaction**](TransactionsApi.md#create_register_transaction) | **POST** /transactions/register | Create a transaction to start a user's registration process



## create_authenticate_transaction

> models::CreateTransactionResponse create_authenticate_transaction(create_transaction_authenticate_request)
Create a transaction to start a user's authentication process

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_transaction_authenticate_request** | [**CreateTransactionAuthenticateRequest**](CreateTransactionAuthenticateRequest.md) |  | [required] |

### Return type

[**models::CreateTransactionResponse**](CreateTransactionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_register_transaction

> models::CreateTransactionResponse create_register_transaction(create_transaction_register_request)
Create a transaction to start a user's registration process

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_transaction_register_request** | [**CreateTransactionRegisterRequest**](CreateTransactionRegisterRequest.md) |  | [required] |

### Return type

[**models::CreateTransactionResponse**](CreateTransactionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

