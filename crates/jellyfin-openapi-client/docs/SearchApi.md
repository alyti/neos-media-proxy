# \SearchApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get**](SearchApi.md#get) | **GET** /Search/Hints | Gets the search hint result.



## get

> crate::models::SearchHintResult get(search_term, start_index, limit, user_id, include_item_types, exclude_item_types, media_types, parent_id, is_movie, is_series, is_news, is_kids, is_sports, include_people, include_media, include_genres, include_studios, include_artists)
Gets the search hint result.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_term** | **String** | The search term to filter on. | [required] |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**user_id** | Option<**String**> | Optional. Supply a user id to search within a user's library or omit to search all. |  |
**include_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | If specified, only results with the specified item types are returned. This allows multiple, comma delimeted. |  |
**exclude_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | If specified, results with these item types are filtered out. This allows multiple, comma delimeted. |  |
**media_types** | Option<[**Vec<String>**](String.md)> | If specified, only results with the specified media types are returned. This allows multiple, comma delimeted. |  |
**parent_id** | Option<**String**> | If specified, only children of the parent are returned. |  |
**is_movie** | Option<**bool**> | Optional filter for movies. |  |
**is_series** | Option<**bool**> | Optional filter for series. |  |
**is_news** | Option<**bool**> | Optional filter for news. |  |
**is_kids** | Option<**bool**> | Optional filter for kids. |  |
**is_sports** | Option<**bool**> | Optional filter for sports. |  |
**include_people** | Option<**bool**> | Optional filter whether to include people. |  |[default to true]
**include_media** | Option<**bool**> | Optional filter whether to include media. |  |[default to true]
**include_genres** | Option<**bool**> | Optional filter whether to include genres. |  |[default to true]
**include_studios** | Option<**bool**> | Optional filter whether to include studios. |  |[default to true]
**include_artists** | Option<**bool**> | Optional filter whether to include artists. |  |[default to true]

### Return type

[**crate::models::SearchHintResult**](SearchHintResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

