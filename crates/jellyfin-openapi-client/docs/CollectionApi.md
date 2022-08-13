# \CollectionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_to_collection**](CollectionApi.md#add_to_collection) | **POST** /Collections/{collectionId}/Items | Adds items to a collection.
[**create_collection**](CollectionApi.md#create_collection) | **POST** /Collections | Creates a new collection.
[**remove_from_collection**](CollectionApi.md#remove_from_collection) | **DELETE** /Collections/{collectionId}/Items | Removes items from a collection.



## add_to_collection

> add_to_collection(collection_id, ids)
Adds items to a collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | The collection id. | [required] |
**ids** | [**Vec<String>**](String.md) | Item ids, comma delimited. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_collection

> crate::models::CollectionCreationResult create_collection(name, ids, parent_id, is_locked)
Creates a new collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the collection. |  |
**ids** | Option<[**Vec<String>**](String.md)> | Item Ids to add to the collection. |  |
**parent_id** | Option<**String**> | Optional. Create the collection within a specific folder. |  |
**is_locked** | Option<**bool**> | Whether or not to lock the new collection. |  |[default to false]

### Return type

[**crate::models::CollectionCreationResult**](CollectionCreationResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_from_collection

> remove_from_collection(collection_id, ids)
Removes items from a collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | The collection id. | [required] |
**ids** | [**Vec<String>**](String.md) | Item ids, comma delimited. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

