# \ItemsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_items**](ItemsApi.md#get_items) | **GET** /Items | Gets items based on a query.
[**get_items_by_user_id**](ItemsApi.md#get_items_by_user_id) | **GET** /Users/{userId}/Items | Gets items based on a query.
[**get_resume_items**](ItemsApi.md#get_resume_items) | **GET** /Users/{userId}/Items/Resume | Gets items based on a query.



## get_items

> crate::models::BaseItemDtoQueryResult get_items(user_id, max_official_rating, has_theme_song, has_theme_video, has_subtitles, has_special_feature, has_trailer, adjacent_to, parent_index_number, has_parental_rating, is_hd, is4_k, location_types, exclude_location_types, is_missing, is_unaired, min_community_rating, min_critic_rating, min_premiere_date, min_date_last_saved, min_date_last_saved_for_user, max_premiere_date, has_overview, has_imdb_id, has_tmdb_id, has_tvdb_id, is_movie, is_series, is_news, is_kids, is_sports, exclude_item_ids, start_index, limit, recursive, search_term, sort_order, parent_id, fields, exclude_item_types, include_item_types, filters, is_favorite, media_types, image_types, sort_by, is_played, genres, official_ratings, tags, years, enable_user_data, image_type_limit, enable_image_types, person, person_ids, person_types, studios, artists, exclude_artist_ids, artist_ids, album_artist_ids, contributing_artist_ids, albums, album_ids, ids, video_types, min_official_rating, is_locked, is_place_holder, has_official_rating, collapse_box_set_items, min_width, min_height, max_width, max_height, is3_d, series_status, name_starts_with_or_greater, name_starts_with, name_less_than, studio_ids, genre_ids, enable_total_record_count, enable_images)
Gets items based on a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | The user id supplied as query parameter. |  |
**max_official_rating** | Option<**String**> | Optional filter by maximum official rating (PG, PG-13, TV-MA, etc). |  |
**has_theme_song** | Option<**bool**> | Optional filter by items with theme songs. |  |
**has_theme_video** | Option<**bool**> | Optional filter by items with theme videos. |  |
**has_subtitles** | Option<**bool**> | Optional filter by items with subtitles. |  |
**has_special_feature** | Option<**bool**> | Optional filter by items with special features. |  |
**has_trailer** | Option<**bool**> | Optional filter by items with trailers. |  |
**adjacent_to** | Option<**String**> | Optional. Return items that are siblings of a supplied item. |  |
**parent_index_number** | Option<**i32**> | Optional filter by parent index number. |  |
**has_parental_rating** | Option<**bool**> | Optional filter by items that have or do not have a parental rating. |  |
**is_hd** | Option<**bool**> | Optional filter by items that are HD or not. |  |
**is4_k** | Option<**bool**> | Optional filter by items that are 4K or not. |  |
**location_types** | Option<[**Vec<crate::models::LocationType>**](crate::models::LocationType.md)> | Optional. If specified, results will be filtered based on LocationType. This allows multiple, comma delimited. |  |
**exclude_location_types** | Option<[**Vec<crate::models::LocationType>**](crate::models::LocationType.md)> | Optional. If specified, results will be filtered based on the LocationType. This allows multiple, comma delimited. |  |
**is_missing** | Option<**bool**> | Optional filter by items that are missing episodes or not. |  |
**is_unaired** | Option<**bool**> | Optional filter by items that are unaired episodes or not. |  |
**min_community_rating** | Option<**f64**> | Optional filter by minimum community rating. |  |
**min_critic_rating** | Option<**f64**> | Optional filter by minimum critic rating. |  |
**min_premiere_date** | Option<**String**> | Optional. The minimum premiere date. Format = ISO. |  |
**min_date_last_saved** | Option<**String**> | Optional. The minimum last saved date. Format = ISO. |  |
**min_date_last_saved_for_user** | Option<**String**> | Optional. The minimum last saved date for the current user. Format = ISO. |  |
**max_premiere_date** | Option<**String**> | Optional. The maximum premiere date. Format = ISO. |  |
**has_overview** | Option<**bool**> | Optional filter by items that have an overview or not. |  |
**has_imdb_id** | Option<**bool**> | Optional filter by items that have an imdb id or not. |  |
**has_tmdb_id** | Option<**bool**> | Optional filter by items that have a tmdb id or not. |  |
**has_tvdb_id** | Option<**bool**> | Optional filter by items that have a tvdb id or not. |  |
**is_movie** | Option<**bool**> | Optional filter for live tv movies. |  |
**is_series** | Option<**bool**> | Optional filter for live tv series. |  |
**is_news** | Option<**bool**> | Optional filter for live tv news. |  |
**is_kids** | Option<**bool**> | Optional filter for live tv kids. |  |
**is_sports** | Option<**bool**> | Optional filter for live tv sports. |  |
**exclude_item_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered by excluding item ids. This allows multiple, comma delimited. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**recursive** | Option<**bool**> | When searching within folders, this determines whether or not the search will be recursive. true/false. |  |
**search_term** | Option<**String**> | Optional. Filter based on a search term. |  |
**sort_order** | Option<[**Vec<crate::models::SortOrder>**](crate::models::SortOrder.md)> | Sort Order - Ascending,Descending. |  |
**parent_id** | Option<**String**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines. |  |
**exclude_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**include_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on the item type. This allows multiple, comma delimited. |  |
**filters** | Option<[**Vec<crate::models::ItemFilter>**](crate::models::ItemFilter.md)> | Optional. Specify additional filters to apply. This allows multiple, comma delimited. Options: IsFolder, IsNotFolder, IsUnplayed, IsPlayed, IsFavorite, IsResumable, Likes, Dislikes. |  |
**is_favorite** | Option<**bool**> | Optional filter by items that are marked as favorite, or not. |  |
**media_types** | Option<[**Vec<String>**](String.md)> | Optional filter by MediaType. Allows multiple, comma delimited. |  |
**image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. If specified, results will be filtered based on those containing image types. This allows multiple, comma delimited. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime. |  |
**is_played** | Option<**bool**> | Optional filter by items that are played, or not. |  |
**genres** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited. |  |
**official_ratings** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited. |  |
**years** | Option<[**Vec<i32>**](i32.md)> | Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited. |  |
**enable_user_data** | Option<**bool**> | Optional, include user data. |  |
**image_type_limit** | Option<**i32**> | Optional, the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**person** | Option<**String**> | Optional. If specified, results will be filtered to include only those containing the specified person. |  |
**person_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered to include only those containing the specified person id. |  |
**person_types** | Option<[**Vec<String>**](String.md)> | Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited. |  |
**studios** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited. |  |
**artists** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on artists. This allows multiple, pipe delimited. |  |
**exclude_artist_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on artist id. This allows multiple, pipe delimited. |  |
**artist_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered to include only those containing the specified artist id. |  |
**album_artist_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered to include only those containing the specified album artist id. |  |
**contributing_artist_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered to include only those containing the specified contributing artist id. |  |
**albums** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on album. This allows multiple, pipe delimited. |  |
**album_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on album id. This allows multiple, pipe delimited. |  |
**ids** | Option<[**Vec<String>**](String.md)> | Optional. If specific items are needed, specify a list of item id's to retrieve. This allows multiple, comma delimited. |  |
**video_types** | Option<[**Vec<crate::models::VideoType>**](crate::models::VideoType.md)> | Optional filter by VideoType (videofile, dvd, bluray, iso). Allows multiple, comma delimited. |  |
**min_official_rating** | Option<**String**> | Optional filter by minimum official rating (PG, PG-13, TV-MA, etc). |  |
**is_locked** | Option<**bool**> | Optional filter by items that are locked. |  |
**is_place_holder** | Option<**bool**> | Optional filter by items that are placeholders. |  |
**has_official_rating** | Option<**bool**> | Optional filter by items that have official ratings. |  |
**collapse_box_set_items** | Option<**bool**> | Whether or not to hide items behind their boxsets. |  |
**min_width** | Option<**i32**> | Optional. Filter by the minimum width of the item. |  |
**min_height** | Option<**i32**> | Optional. Filter by the minimum height of the item. |  |
**max_width** | Option<**i32**> | Optional. Filter by the maximum width of the item. |  |
**max_height** | Option<**i32**> | Optional. Filter by the maximum height of the item. |  |
**is3_d** | Option<**bool**> | Optional filter by items that are 3D, or not. |  |
**series_status** | Option<[**Vec<crate::models::SeriesStatus>**](crate::models::SeriesStatus.md)> | Optional filter by Series Status. Allows multiple, comma delimited. |  |
**name_starts_with_or_greater** | Option<**String**> | Optional filter by items whose name is sorted equally or greater than a given input string. |  |
**name_starts_with** | Option<**String**> | Optional filter by items whose name is sorted equally than a given input string. |  |
**name_less_than** | Option<**String**> | Optional filter by items whose name is equally or lesser than a given input string. |  |
**studio_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited. |  |
**genre_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited. |  |
**enable_total_record_count** | Option<**bool**> | Optional. Enable the total record count. |  |[default to true]
**enable_images** | Option<**bool**> | Optional, include image information in output. |  |[default to true]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_items_by_user_id

> crate::models::BaseItemDtoQueryResult get_items_by_user_id(user_id, max_official_rating, has_theme_song, has_theme_video, has_subtitles, has_special_feature, has_trailer, adjacent_to, parent_index_number, has_parental_rating, is_hd, is4_k, location_types, exclude_location_types, is_missing, is_unaired, min_community_rating, min_critic_rating, min_premiere_date, min_date_last_saved, min_date_last_saved_for_user, max_premiere_date, has_overview, has_imdb_id, has_tmdb_id, has_tvdb_id, is_movie, is_series, is_news, is_kids, is_sports, exclude_item_ids, start_index, limit, recursive, search_term, sort_order, parent_id, fields, exclude_item_types, include_item_types, filters, is_favorite, media_types, image_types, sort_by, is_played, genres, official_ratings, tags, years, enable_user_data, image_type_limit, enable_image_types, person, person_ids, person_types, studios, artists, exclude_artist_ids, artist_ids, album_artist_ids, contributing_artist_ids, albums, album_ids, ids, video_types, min_official_rating, is_locked, is_place_holder, has_official_rating, collapse_box_set_items, min_width, min_height, max_width, max_height, is3_d, series_status, name_starts_with_or_greater, name_starts_with, name_less_than, studio_ids, genre_ids, enable_total_record_count, enable_images)
Gets items based on a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The user id supplied as query parameter. | [required] |
**max_official_rating** | Option<**String**> | Optional filter by maximum official rating (PG, PG-13, TV-MA, etc). |  |
**has_theme_song** | Option<**bool**> | Optional filter by items with theme songs. |  |
**has_theme_video** | Option<**bool**> | Optional filter by items with theme videos. |  |
**has_subtitles** | Option<**bool**> | Optional filter by items with subtitles. |  |
**has_special_feature** | Option<**bool**> | Optional filter by items with special features. |  |
**has_trailer** | Option<**bool**> | Optional filter by items with trailers. |  |
**adjacent_to** | Option<**String**> | Optional. Return items that are siblings of a supplied item. |  |
**parent_index_number** | Option<**i32**> | Optional filter by parent index number. |  |
**has_parental_rating** | Option<**bool**> | Optional filter by items that have or do not have a parental rating. |  |
**is_hd** | Option<**bool**> | Optional filter by items that are HD or not. |  |
**is4_k** | Option<**bool**> | Optional filter by items that are 4K or not. |  |
**location_types** | Option<[**Vec<crate::models::LocationType>**](crate::models::LocationType.md)> | Optional. If specified, results will be filtered based on LocationType. This allows multiple, comma delimited. |  |
**exclude_location_types** | Option<[**Vec<crate::models::LocationType>**](crate::models::LocationType.md)> | Optional. If specified, results will be filtered based on the LocationType. This allows multiple, comma delimited. |  |
**is_missing** | Option<**bool**> | Optional filter by items that are missing episodes or not. |  |
**is_unaired** | Option<**bool**> | Optional filter by items that are unaired episodes or not. |  |
**min_community_rating** | Option<**f64**> | Optional filter by minimum community rating. |  |
**min_critic_rating** | Option<**f64**> | Optional filter by minimum critic rating. |  |
**min_premiere_date** | Option<**String**> | Optional. The minimum premiere date. Format = ISO. |  |
**min_date_last_saved** | Option<**String**> | Optional. The minimum last saved date. Format = ISO. |  |
**min_date_last_saved_for_user** | Option<**String**> | Optional. The minimum last saved date for the current user. Format = ISO. |  |
**max_premiere_date** | Option<**String**> | Optional. The maximum premiere date. Format = ISO. |  |
**has_overview** | Option<**bool**> | Optional filter by items that have an overview or not. |  |
**has_imdb_id** | Option<**bool**> | Optional filter by items that have an imdb id or not. |  |
**has_tmdb_id** | Option<**bool**> | Optional filter by items that have a tmdb id or not. |  |
**has_tvdb_id** | Option<**bool**> | Optional filter by items that have a tvdb id or not. |  |
**is_movie** | Option<**bool**> | Optional filter for live tv movies. |  |
**is_series** | Option<**bool**> | Optional filter for live tv series. |  |
**is_news** | Option<**bool**> | Optional filter for live tv news. |  |
**is_kids** | Option<**bool**> | Optional filter for live tv kids. |  |
**is_sports** | Option<**bool**> | Optional filter for live tv sports. |  |
**exclude_item_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered by excluding item ids. This allows multiple, comma delimited. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**recursive** | Option<**bool**> | When searching within folders, this determines whether or not the search will be recursive. true/false. |  |
**search_term** | Option<**String**> | Optional. Filter based on a search term. |  |
**sort_order** | Option<[**Vec<crate::models::SortOrder>**](crate::models::SortOrder.md)> | Sort Order - Ascending,Descending. |  |
**parent_id** | Option<**String**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines. |  |
**exclude_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**include_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on the item type. This allows multiple, comma delimited. |  |
**filters** | Option<[**Vec<crate::models::ItemFilter>**](crate::models::ItemFilter.md)> | Optional. Specify additional filters to apply. This allows multiple, comma delimited. Options: IsFolder, IsNotFolder, IsUnplayed, IsPlayed, IsFavorite, IsResumable, Likes, Dislikes. |  |
**is_favorite** | Option<**bool**> | Optional filter by items that are marked as favorite, or not. |  |
**media_types** | Option<[**Vec<String>**](String.md)> | Optional filter by MediaType. Allows multiple, comma delimited. |  |
**image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. If specified, results will be filtered based on those containing image types. This allows multiple, comma delimited. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime. |  |
**is_played** | Option<**bool**> | Optional filter by items that are played, or not. |  |
**genres** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited. |  |
**official_ratings** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited. |  |
**years** | Option<[**Vec<i32>**](i32.md)> | Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited. |  |
**enable_user_data** | Option<**bool**> | Optional, include user data. |  |
**image_type_limit** | Option<**i32**> | Optional, the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**person** | Option<**String**> | Optional. If specified, results will be filtered to include only those containing the specified person. |  |
**person_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered to include only those containing the specified person id. |  |
**person_types** | Option<[**Vec<String>**](String.md)> | Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited. |  |
**studios** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited. |  |
**artists** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on artists. This allows multiple, pipe delimited. |  |
**exclude_artist_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on artist id. This allows multiple, pipe delimited. |  |
**artist_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered to include only those containing the specified artist id. |  |
**album_artist_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered to include only those containing the specified album artist id. |  |
**contributing_artist_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered to include only those containing the specified contributing artist id. |  |
**albums** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on album. This allows multiple, pipe delimited. |  |
**album_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on album id. This allows multiple, pipe delimited. |  |
**ids** | Option<[**Vec<String>**](String.md)> | Optional. If specific items are needed, specify a list of item id's to retrieve. This allows multiple, comma delimited. |  |
**video_types** | Option<[**Vec<crate::models::VideoType>**](crate::models::VideoType.md)> | Optional filter by VideoType (videofile, dvd, bluray, iso). Allows multiple, comma delimited. |  |
**min_official_rating** | Option<**String**> | Optional filter by minimum official rating (PG, PG-13, TV-MA, etc). |  |
**is_locked** | Option<**bool**> | Optional filter by items that are locked. |  |
**is_place_holder** | Option<**bool**> | Optional filter by items that are placeholders. |  |
**has_official_rating** | Option<**bool**> | Optional filter by items that have official ratings. |  |
**collapse_box_set_items** | Option<**bool**> | Whether or not to hide items behind their boxsets. |  |
**min_width** | Option<**i32**> | Optional. Filter by the minimum width of the item. |  |
**min_height** | Option<**i32**> | Optional. Filter by the minimum height of the item. |  |
**max_width** | Option<**i32**> | Optional. Filter by the maximum width of the item. |  |
**max_height** | Option<**i32**> | Optional. Filter by the maximum height of the item. |  |
**is3_d** | Option<**bool**> | Optional filter by items that are 3D, or not. |  |
**series_status** | Option<[**Vec<crate::models::SeriesStatus>**](crate::models::SeriesStatus.md)> | Optional filter by Series Status. Allows multiple, comma delimited. |  |
**name_starts_with_or_greater** | Option<**String**> | Optional filter by items whose name is sorted equally or greater than a given input string. |  |
**name_starts_with** | Option<**String**> | Optional filter by items whose name is sorted equally than a given input string. |  |
**name_less_than** | Option<**String**> | Optional filter by items whose name is equally or lesser than a given input string. |  |
**studio_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited. |  |
**genre_ids** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited. |  |
**enable_total_record_count** | Option<**bool**> | Optional. Enable the total record count. |  |[default to true]
**enable_images** | Option<**bool**> | Optional, include image information in output. |  |[default to true]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resume_items

> crate::models::BaseItemDtoQueryResult get_resume_items(user_id, start_index, limit, search_term, parent_id, fields, media_types, enable_user_data, image_type_limit, enable_image_types, exclude_item_types, include_item_types, enable_total_record_count, enable_images, exclude_active_sessions)
Gets items based on a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The user id. | [required] |
**start_index** | Option<**i32**> | The start index. |  |
**limit** | Option<**i32**> | The item limit. |  |
**search_term** | Option<**String**> | The search term. |  |
**parent_id** | Option<**String**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines. |  |
**media_types** | Option<[**Vec<String>**](String.md)> | Optional. Filter by MediaType. Allows multiple, comma delimited. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**exclude_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**include_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on the item type. This allows multiple, comma delimited. |  |
**enable_total_record_count** | Option<**bool**> | Optional. Enable the total record count. |  |[default to true]
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |[default to true]
**exclude_active_sessions** | Option<**bool**> | Optional. Whether to exclude the currently active sessions. |  |[default to false]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

