# ListPaginatedUsersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_links** | [**models::PaginatedLinks**](PaginatedLinks.md) |  | 
**created_before** | **i64** | time anchor (Unix timestamp) --> all users returned created before this timestamp | 
**limit** | **i32** |  | 
**page** | **i32** |  | 
**total_users** | **i64** | total number of users for a particular query | 
**users** | [**Vec<models::ListPaginatedUsersItem>**](ListPaginatedUsersItem.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


