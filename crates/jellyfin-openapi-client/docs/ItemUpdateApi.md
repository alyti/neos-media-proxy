# \ItemUpdateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_metadata_editor_info**](ItemUpdateApi.md#get_metadata_editor_info) | **GET** /Items/{itemId}/MetadataEditor | Gets metadata editor info for an item.
[**update_item**](ItemUpdateApi.md#update_item) | **POST** /Items/{itemId} | Updates an item.
[**update_item_content_type**](ItemUpdateApi.md#update_item_content_type) | **POST** /Items/{itemId}/ContentType | Updates an item's content type.



## get_metadata_editor_info

> crate::models::MetadataEditorInfo get_metadata_editor_info(item_id)
Gets metadata editor info for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |

### Return type

[**crate::models::MetadataEditorInfo**](MetadataEditorInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_item

> update_item(item_id, update_item_request)
Updates an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**update_item_request** | [**UpdateItemRequest**](UpdateItemRequest.md) | The new item properties. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_item_content_type

> update_item_content_type(item_id, content_type)
Updates an item's content type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**content_type** | Option<**String**> | The content type of the item. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

