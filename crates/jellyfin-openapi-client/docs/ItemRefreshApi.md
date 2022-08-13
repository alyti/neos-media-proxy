# \ItemRefreshApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**refresh_item**](ItemRefreshApi.md#refresh_item) | **POST** /Items/{itemId}/Refresh | Refreshes metadata for an item.



## refresh_item

> refresh_item(item_id, metadata_refresh_mode, image_refresh_mode, replace_all_metadata, replace_all_images)
Refreshes metadata for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**metadata_refresh_mode** | Option<[**crate::models::MetadataRefreshMode**](.md)> | (Optional) Specifies the metadata refresh mode. |  |
**image_refresh_mode** | Option<[**crate::models::MetadataRefreshMode**](.md)> | (Optional) Specifies the image refresh mode. |  |
**replace_all_metadata** | Option<**bool**> | (Optional) Determines if metadata should be replaced. Only applicable if mode is FullRefresh. |  |[default to false]
**replace_all_images** | Option<**bool**> | (Optional) Determines if images should be replaced. Only applicable if mode is FullRefresh. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

