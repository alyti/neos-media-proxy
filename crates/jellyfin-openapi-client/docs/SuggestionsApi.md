# \SuggestionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_suggestions**](SuggestionsApi.md#get_suggestions) | **GET** /Users/{userId}/Suggestions | Gets suggestions.



## get_suggestions

> crate::models::BaseItemDtoQueryResult get_suggestions(user_id, media_type, _type, start_index, limit, enable_total_record_count)
Gets suggestions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The user id. | [required] |
**media_type** | Option<[**Vec<String>**](String.md)> | The media types. |  |
**_type** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | The type. |  |
**start_index** | Option<**i32**> | Optional. The start index. |  |
**limit** | Option<**i32**> | Optional. The limit. |  |
**enable_total_record_count** | Option<**bool**> | Whether to enable the total record count. |  |[default to false]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

