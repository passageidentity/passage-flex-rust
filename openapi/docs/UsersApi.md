# \UsersApi

All URIs are relative to *https://api.passage.id/v1/apps/TODO*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user**](UsersApi.md#get_user) | **GET** /users/{user_id} | Get User
[**list_paginated_users**](UsersApi.md#list_paginated_users) | **GET** /users | List Users



## get_user

> models::UserResponse get_user(user_id)
Get User

Get information about a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_paginated_users

> models::ListPaginatedUsersResponse list_paginated_users(page, limit, created_before, order_by, identifier, id, login_count, status, created_at, updated_at, last_login_at)
List Users

List users for an app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | page to fetch (min=1) |  |
**limit** | Option<**i32**> | number of users to fetch per page (max=500) |  |
**created_before** | Option<**i32**> | Unix timestamp to anchor pagination results (fetches events that were created before the timestamp) |  |
**order_by** | Option<**String**> | Comma separated list of <field>:<ASC/DESC> (example: order_by=id:DESC,created_at:ASC) **cannot order_by `identifier` |  |
**identifier** | Option<**String**> | search users email OR phone (pagination prepended operators identifier=<val>, identifier=<ne:val>, identifier=<gt:val>, identifier=<lt:val>, identifier=<like:val>, identifier=<not_like:val>) |  |
**id** | Option<**String**> | search users id (pagination prepended operators id=<val>, id=<ne:val>, id=<gt:val>, id=<lt:val>, id=<like:val>, id=<not_like:val>) |  |
**login_count** | Option<**i32**> | search users login_count (pagination prepended operators login_count=<val>, login_count=<ne:val>, login_count=<gt:val>, login_count=<lt:val>) |  |
**status** | Option<**String**> | search users by status (pagination prepended operators status=<val>, status=<ne:val>, status=<gt:val>, status=<lt:val>, status=<like:val>, status=<not_like:val>) -- valid values: (active, inactive, pending) |  |
**created_at** | Option<**String**> | search users created_at (pagination prepended operators created_at=<val>, created_at=<ne:val>, created_at=<gt:val>, created_at=<lt:val> -- valid timestamp in the format: 2006-01-02T15:04:05.000000Z required |  |
**updated_at** | Option<**String**> | search users updated_at (pagination prepended operators updated_at=<val>, updated_at=<ne:val>, updated_at=<gt:val>, updated_at=<lt:val> -- valid timestamp in the format: 2006-01-02T15:04:05.000000Z required |  |
**last_login_at** | Option<**String**> | search users last_login_at (pagination prepended operators last_login_at=<val>, lat_login_at=<ne:val>, last_login_at=<gt:val>, last_login_at=<lt:val> -- valid timestamp in the format: 2006-01-02T15:04:05.000000Z required |  |

### Return type

[**models::ListPaginatedUsersResponse**](ListPaginatedUsersResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

