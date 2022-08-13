# \MusicGenresApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_music_genre**](MusicGenresApi.md#get_music_genre) | **GET** /MusicGenres/{genreName} | Gets a music genre, by name.
[**get_music_genres**](MusicGenresApi.md#get_music_genres) | **GET** /MusicGenres | Gets all music genres from a given item, folder, or the entire library.



## get_music_genre

> crate::models::BaseItemDto get_music_genre(genre_name, user_id)
Gets a music genre, by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**genre_name** | **String** | The genre name. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |

### Return type

[**crate::models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_genres

> crate::models::BaseItemDtoQueryResult get_music_genres(start_index, limit, search_term, parent_id, fields, exclude_item_types, include_item_types, is_favorite, image_type_limit, enable_image_types, user_id, name_starts_with_or_greater, name_starts_with, name_less_than, sort_by, sort_order, enable_images, enable_total_record_count)
Gets all music genres from a given item, folder, or the entire library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**search_term** | Option<**String**> | The search term. |  |
**parent_id** | Option<**String**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**exclude_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited. |  |
**include_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered in based on item type. This allows multiple, comma delimited. |  |
**is_favorite** | Option<**bool**> | Optional filter by items that are marked as favorite, or not. |  |
**image_type_limit** | Option<**i32**> | Optional, the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**user_id** | Option<**String**> | User id. |  |
**name_starts_with_or_greater** | Option<**String**> | Optional filter by items whose name is sorted equally or greater than a given input string. |  |
**name_starts_with** | Option<**String**> | Optional filter by items whose name is sorted equally than a given input string. |  |
**name_less_than** | Option<**String**> | Optional filter by items whose name is equally or lesser than a given input string. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Optional. Specify one or more sort orders, comma delimited. |  |
**sort_order** | Option<[**Vec<crate::models::SortOrder>**](crate::models::SortOrder.md)> | Sort Order - Ascending,Descending. |  |
**enable_images** | Option<**bool**> | Optional, include image information in output. |  |[default to true]
**enable_total_record_count** | Option<**bool**> | Optional. Include total record count. |  |[default to true]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

