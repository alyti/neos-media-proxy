# \ArtistsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_album_artists**](ArtistsApi.md#get_album_artists) | **GET** /Artists/AlbumArtists | Gets all album artists from a given item, folder, or the entire library.
[**get_artist_by_name**](ArtistsApi.md#get_artist_by_name) | **GET** /Artists/{name} | Gets an artist by name.
[**get_artists**](ArtistsApi.md#get_artists) | **GET** /Artists | Gets all artists from a given item, folder, or the entire library.



## get_album_artists

> crate::models::BaseItemDtoQueryResult get_album_artists(min_community_rating, start_index, limit, search_term, parent_id, fields, exclude_item_types, include_item_types, filters, is_favorite, media_types, genres, genre_ids, official_ratings, tags, years, enable_user_data, image_type_limit, enable_image_types, person, person_ids, person_types, studios, studio_ids, user_id, name_starts_with_or_greater, name_starts_with, name_less_than, sort_by, sort_order, enable_images, enable_total_record_count)
Gets all album artists from a given item, folder, or the entire library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_community_rating** | Option<**f64**> | Optional filter by minimum community rating. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**search_term** | Option<**String**> | Optional. Search term. |  |
**parent_id** | Option<**String**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**exclude_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited. |  |
**include_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**filters** | Option<[**Vec<crate::models::ItemFilter>**](crate::models::ItemFilter.md)> | Optional. Specify additional filters to apply. |  |
**is_favorite** | Option<**bool**> | Optional filter by items that are marked as favorite, or not. |  |
**media_types** | Option<[**Vec<String>**](String.md)> | Optional filter by MediaType. Allows multiple, comma delimited. |  |
**genres** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited. |  |
**genre_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited. |  |
**official_ratings** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited. |  |
**years** | Option<[**Vec<i32>**](i32.md)> | Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited. |  |
**enable_user_data** | Option<**bool**> | Optional, include user data. |  |
**image_type_limit** | Option<**i32**> | Optional, the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**person** | Option<**String**> | Optional. If specified, results will be filtered to include only those containing the specified person. |  |
**person_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered to include only those containing the specified person ids. |  |
**person_types** | Option<[**Vec<String>**](String.md)> | Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited. |  |
**studios** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited. |  |
**studio_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited. |  |
**user_id** | Option<**String**> | User id. |  |
**name_starts_with_or_greater** | Option<**String**> | Optional filter by items whose name is sorted equally or greater than a given input string. |  |
**name_starts_with** | Option<**String**> | Optional filter by items whose name is sorted equally than a given input string. |  |
**name_less_than** | Option<**String**> | Optional filter by items whose name is equally or lesser than a given input string. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Optional. Specify one or more sort orders, comma delimited. |  |
**sort_order** | Option<[**Vec<crate::models::SortOrder>**](crate::models::SortOrder.md)> | Sort Order - Ascending,Descending. |  |
**enable_images** | Option<**bool**> | Optional, include image information in output. |  |[default to true]
**enable_total_record_count** | Option<**bool**> | Total record count. |  |[default to true]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_by_name

> crate::models::BaseItemDto get_artist_by_name(name, user_id)
Gets an artist by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Studio name. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |

### Return type

[**crate::models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artists

> crate::models::BaseItemDtoQueryResult get_artists(min_community_rating, start_index, limit, search_term, parent_id, fields, exclude_item_types, include_item_types, filters, is_favorite, media_types, genres, genre_ids, official_ratings, tags, years, enable_user_data, image_type_limit, enable_image_types, person, person_ids, person_types, studios, studio_ids, user_id, name_starts_with_or_greater, name_starts_with, name_less_than, sort_by, sort_order, enable_images, enable_total_record_count)
Gets all artists from a given item, folder, or the entire library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_community_rating** | Option<**f64**> | Optional filter by minimum community rating. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**search_term** | Option<**String**> | Optional. Search term. |  |
**parent_id** | Option<**String**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**exclude_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered out based on item type. This allows multiple, comma delimited. |  |
**include_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**filters** | Option<[**Vec<crate::models::ItemFilter>**](crate::models::ItemFilter.md)> | Optional. Specify additional filters to apply. |  |
**is_favorite** | Option<**bool**> | Optional filter by items that are marked as favorite, or not. |  |
**media_types** | Option<[**Vec<String>**](String.md)> | Optional filter by MediaType. Allows multiple, comma delimited. |  |
**genres** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited. |  |
**genre_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited. |  |
**official_ratings** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited. |  |
**years** | Option<[**Vec<i32>**](i32.md)> | Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited. |  |
**enable_user_data** | Option<**bool**> | Optional, include user data. |  |
**image_type_limit** | Option<**i32**> | Optional, the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**person** | Option<**String**> | Optional. If specified, results will be filtered to include only those containing the specified person. |  |
**person_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered to include only those containing the specified person ids. |  |
**person_types** | Option<[**Vec<String>**](String.md)> | Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited. |  |
**studios** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited. |  |
**studio_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited. |  |
**user_id** | Option<**String**> | User id. |  |
**name_starts_with_or_greater** | Option<**String**> | Optional filter by items whose name is sorted equally or greater than a given input string. |  |
**name_starts_with** | Option<**String**> | Optional filter by items whose name is sorted equally than a given input string. |  |
**name_less_than** | Option<**String**> | Optional filter by items whose name is equally or lesser than a given input string. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Optional. Specify one or more sort orders, comma delimited. |  |
**sort_order** | Option<[**Vec<crate::models::SortOrder>**](crate::models::SortOrder.md)> | Sort Order - Ascending,Descending. |  |
**enable_images** | Option<**bool**> | Optional, include image information in output. |  |[default to true]
**enable_total_record_count** | Option<**bool**> | Total record count. |  |[default to true]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

