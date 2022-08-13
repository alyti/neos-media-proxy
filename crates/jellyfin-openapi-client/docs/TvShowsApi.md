# \TvShowsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_episodes**](TvShowsApi.md#get_episodes) | **GET** /Shows/{seriesId}/Episodes | Gets episodes for a tv season.
[**get_next_up**](TvShowsApi.md#get_next_up) | **GET** /Shows/NextUp | Gets a list of next up episodes.
[**get_seasons**](TvShowsApi.md#get_seasons) | **GET** /Shows/{seriesId}/Seasons | Gets seasons for a tv series.
[**get_upcoming_episodes**](TvShowsApi.md#get_upcoming_episodes) | **GET** /Shows/Upcoming | Gets a list of upcoming episodes.



## get_episodes

> crate::models::BaseItemDtoQueryResult get_episodes(series_id, user_id, fields, season, season_id, is_missing, adjacent_to, start_item_id, start_index, limit, enable_images, image_type_limit, enable_image_types, enable_user_data, sort_by)
Gets episodes for a tv season.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** | The series id. | [required] |
**user_id** | Option<**String**> | The user id. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |
**season** | Option<**i32**> | Optional filter by season number. |  |
**season_id** | Option<**String**> | Optional. Filter by season id. |  |
**is_missing** | Option<**bool**> | Optional. Filter by items that are missing episodes or not. |  |
**adjacent_to** | Option<**String**> | Optional. Return items that are siblings of a supplied item. |  |
**start_item_id** | Option<**String**> | Optional. Skip through the list until a given item is found. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**enable_images** | Option<**bool**> | Optional, include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional, the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**sort_by** | Option<**String**> | Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_next_up

> crate::models::BaseItemDtoQueryResult get_next_up(user_id, start_index, limit, fields, series_id, parent_id, enable_images, image_type_limit, enable_image_types, enable_user_data, next_up_date_cutoff, enable_total_record_count, disable_first_episode, enable_rewatching)
Gets a list of next up episodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user id of the user to get the next up episodes for. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**series_id** | Option<**String**> | Optional. Filter by series id. |  |
**parent_id** | Option<**String**> | Optional. Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**next_up_date_cutoff** | Option<**String**> | Optional. Starting date of shows to show in Next Up section. |  |
**enable_total_record_count** | Option<**bool**> | Whether to enable the total records count. Defaults to true. |  |[default to true]
**disable_first_episode** | Option<**bool**> | Whether to disable sending the first episode in a series as next up. |  |[default to false]
**enable_rewatching** | Option<**bool**> | Whether to include watched episode in next up results. |  |[default to false]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_seasons

> crate::models::BaseItemDtoQueryResult get_seasons(series_id, user_id, fields, is_special_season, is_missing, adjacent_to, enable_images, image_type_limit, enable_image_types, enable_user_data)
Gets seasons for a tv series.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | **String** | The series id. | [required] |
**user_id** | Option<**String**> | The user id. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |
**is_special_season** | Option<**bool**> | Optional. Filter by special season. |  |
**is_missing** | Option<**bool**> | Optional. Filter by items that are missing episodes or not. |  |
**adjacent_to** | Option<**String**> | Optional. Return items that are siblings of a supplied item. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_upcoming_episodes

> crate::models::BaseItemDtoQueryResult get_upcoming_episodes(user_id, start_index, limit, fields, parent_id, enable_images, image_type_limit, enable_image_types, enable_user_data)
Gets a list of upcoming episodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user id of the user to get the upcoming episodes for. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**parent_id** | Option<**String**> | Optional. Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

