# \UserViewsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_grouping_options**](UserViewsApi.md#get_grouping_options) | **GET** /Users/{userId}/GroupingOptions | Get user view grouping options.
[**get_user_views**](UserViewsApi.md#get_user_views) | **GET** /Users/{userId}/Views | Get user views.



## get_grouping_options

> Vec<crate::models::SpecialViewOptionDto> get_grouping_options(user_id)
Get user view grouping options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |

### Return type

[**Vec<crate::models::SpecialViewOptionDto>**](SpecialViewOptionDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_views

> crate::models::BaseItemDtoQueryResult get_user_views(user_id, include_external_content, preset_views, include_hidden)
Get user views.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**include_external_content** | Option<**bool**> | Whether or not to include external views such as channels or live tv. |  |
**preset_views** | Option<[**Vec<String>**](String.md)> | Preset views. |  |
**include_hidden** | Option<**bool**> | Whether or not to include hidden content. |  |[default to false]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

