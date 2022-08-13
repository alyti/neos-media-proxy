# \FilterApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_query_filters**](FilterApi.md#get_query_filters) | **GET** /Items/Filters2 | Gets query filters.
[**get_query_filters_legacy**](FilterApi.md#get_query_filters_legacy) | **GET** /Items/Filters | Gets legacy query filters.



## get_query_filters

> crate::models::QueryFilters get_query_filters(user_id, parent_id, include_item_types, is_airing, is_movie, is_sports, is_kids, is_news, is_series, recursive)
Gets query filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | Optional. User id. |  |
**parent_id** | Option<**String**> | Optional. Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**include_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**is_airing** | Option<**bool**> | Optional. Is item airing. |  |
**is_movie** | Option<**bool**> | Optional. Is item movie. |  |
**is_sports** | Option<**bool**> | Optional. Is item sports. |  |
**is_kids** | Option<**bool**> | Optional. Is item kids. |  |
**is_news** | Option<**bool**> | Optional. Is item news. |  |
**is_series** | Option<**bool**> | Optional. Is item series. |  |
**recursive** | Option<**bool**> | Optional. Search recursive. |  |

### Return type

[**crate::models::QueryFilters**](QueryFilters.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_query_filters_legacy

> crate::models::QueryFiltersLegacy get_query_filters_legacy(user_id, parent_id, include_item_types, media_types)
Gets legacy query filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | Optional. User id. |  |
**parent_id** | Option<**String**> | Optional. Parent id. |  |
**include_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**media_types** | Option<[**Vec<String>**](String.md)> | Optional. Filter by MediaType. Allows multiple, comma delimited. |  |

### Return type

[**crate::models::QueryFiltersLegacy**](QueryFiltersLegacy.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

