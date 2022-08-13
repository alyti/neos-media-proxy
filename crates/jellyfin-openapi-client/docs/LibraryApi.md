# \LibraryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_item**](LibraryApi.md#delete_item) | **DELETE** /Items/{itemId} | Deletes an item from the library and filesystem.
[**delete_items**](LibraryApi.md#delete_items) | **DELETE** /Items | Deletes items from the library and filesystem.
[**get_ancestors**](LibraryApi.md#get_ancestors) | **GET** /Items/{itemId}/Ancestors | Gets all parents of an item.
[**get_critic_reviews**](LibraryApi.md#get_critic_reviews) | **GET** /Items/{itemId}/CriticReviews | Gets critic review for an item.
[**get_download**](LibraryApi.md#get_download) | **GET** /Items/{itemId}/Download | Downloads item media.
[**get_file**](LibraryApi.md#get_file) | **GET** /Items/{itemId}/File | Get the original file of an item.
[**get_item_counts**](LibraryApi.md#get_item_counts) | **GET** /Items/Counts | Get item counts.
[**get_library_options_info**](LibraryApi.md#get_library_options_info) | **GET** /Libraries/AvailableOptions | Gets the library options info.
[**get_media_folders**](LibraryApi.md#get_media_folders) | **GET** /Library/MediaFolders | Gets all user media folders.
[**get_physical_paths**](LibraryApi.md#get_physical_paths) | **GET** /Library/PhysicalPaths | Gets a list of physical paths from virtual folders.
[**get_similar_albums**](LibraryApi.md#get_similar_albums) | **GET** /Albums/{itemId}/Similar | Gets similar items.
[**get_similar_artists**](LibraryApi.md#get_similar_artists) | **GET** /Artists/{itemId}/Similar | Gets similar items.
[**get_similar_items**](LibraryApi.md#get_similar_items) | **GET** /Items/{itemId}/Similar | Gets similar items.
[**get_similar_movies**](LibraryApi.md#get_similar_movies) | **GET** /Movies/{itemId}/Similar | Gets similar items.
[**get_similar_shows**](LibraryApi.md#get_similar_shows) | **GET** /Shows/{itemId}/Similar | Gets similar items.
[**get_similar_trailers**](LibraryApi.md#get_similar_trailers) | **GET** /Trailers/{itemId}/Similar | Gets similar items.
[**get_theme_media**](LibraryApi.md#get_theme_media) | **GET** /Items/{itemId}/ThemeMedia | Get theme songs and videos for an item.
[**get_theme_songs**](LibraryApi.md#get_theme_songs) | **GET** /Items/{itemId}/ThemeSongs | Get theme songs for an item.
[**get_theme_videos**](LibraryApi.md#get_theme_videos) | **GET** /Items/{itemId}/ThemeVideos | Get theme videos for an item.
[**post_added_movies**](LibraryApi.md#post_added_movies) | **POST** /Library/Movies/Added | Reports that new movies have been added by an external source.
[**post_added_series**](LibraryApi.md#post_added_series) | **POST** /Library/Series/Added | Reports that new episodes of a series have been added by an external source.
[**post_updated_media**](LibraryApi.md#post_updated_media) | **POST** /Library/Media/Updated | Reports that new movies have been added by an external source.
[**post_updated_movies**](LibraryApi.md#post_updated_movies) | **POST** /Library/Movies/Updated | Reports that new movies have been added by an external source.
[**post_updated_series**](LibraryApi.md#post_updated_series) | **POST** /Library/Series/Updated | Reports that new episodes of a series have been added by an external source.
[**refresh_library**](LibraryApi.md#refresh_library) | **POST** /Library/Refresh | Starts a library scan.



## delete_item

> delete_item(item_id)
Deletes an item from the library and filesystem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_items

> delete_items(ids)
Deletes items from the library and filesystem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | The item ids. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ancestors

> Vec<crate::models::BaseItemDto> get_ancestors(item_id, user_id)
Gets all parents of an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |

### Return type

[**Vec<crate::models::BaseItemDto>**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_critic_reviews

> crate::models::BaseItemDtoQueryResult get_critic_reviews(item_id)
Gets critic review for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** |  | [required] |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_download

> std::path::PathBuf get_download(item_id)
Downloads item media.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: video/*, audio/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file

> std::path::PathBuf get_file(item_id)
Get the original file of an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: video/*, audio/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_counts

> crate::models::ItemCounts get_item_counts(user_id, is_favorite)
Get item counts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | Optional. Get counts from a specific user's library. |  |
**is_favorite** | Option<**bool**> | Optional. Get counts of favorite items. |  |

### Return type

[**crate::models::ItemCounts**](ItemCounts.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_library_options_info

> crate::models::LibraryOptionsResultDto get_library_options_info(library_content_type, is_new_library)
Gets the library options info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_content_type** | Option<**String**> | Library content type. |  |
**is_new_library** | Option<**bool**> | Whether this is a new library. |  |[default to false]

### Return type

[**crate::models::LibraryOptionsResultDto**](LibraryOptionsResultDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_folders

> crate::models::BaseItemDtoQueryResult get_media_folders(is_hidden)
Gets all user media folders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_hidden** | Option<**bool**> | Optional. Filter by folders that are marked hidden, or not. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_physical_paths

> Vec<String> get_physical_paths()
Gets a list of physical paths from virtual folders.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_albums

> crate::models::BaseItemDtoQueryResult get_similar_albums(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<String>**](String.md)> | Exclude artist ids. |  |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_artists

> crate::models::BaseItemDtoQueryResult get_similar_artists(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<String>**](String.md)> | Exclude artist ids. |  |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_items

> crate::models::BaseItemDtoQueryResult get_similar_items(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<String>**](String.md)> | Exclude artist ids. |  |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_movies

> crate::models::BaseItemDtoQueryResult get_similar_movies(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<String>**](String.md)> | Exclude artist ids. |  |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_shows

> crate::models::BaseItemDtoQueryResult get_similar_shows(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<String>**](String.md)> | Exclude artist ids. |  |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_trailers

> crate::models::BaseItemDtoQueryResult get_similar_trailers(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<String>**](String.md)> | Exclude artist ids. |  |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_theme_media

> crate::models::AllThemeMediaResult get_theme_media(item_id, user_id, inherit_from_parent)
Get theme songs and videos for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
**inherit_from_parent** | Option<**bool**> | Optional. Determines whether or not parent items should be searched for theme media. |  |[default to false]

### Return type

[**crate::models::AllThemeMediaResult**](AllThemeMediaResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_theme_songs

> crate::models::ThemeMediaResult get_theme_songs(item_id, user_id, inherit_from_parent)
Get theme songs for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
**inherit_from_parent** | Option<**bool**> | Optional. Determines whether or not parent items should be searched for theme media. |  |[default to false]

### Return type

[**crate::models::ThemeMediaResult**](ThemeMediaResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_theme_videos

> crate::models::ThemeMediaResult get_theme_videos(item_id, user_id, inherit_from_parent)
Get theme videos for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
**inherit_from_parent** | Option<**bool**> | Optional. Determines whether or not parent items should be searched for theme media. |  |[default to false]

### Return type

[**crate::models::ThemeMediaResult**](ThemeMediaResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_added_movies

> post_added_movies(tmdb_id, imdb_id)
Reports that new movies have been added by an external source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tmdb_id** | Option<**String**> | The tmdbId. |  |
**imdb_id** | Option<**String**> | The imdbId. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_added_series

> post_added_series(tvdb_id)
Reports that new episodes of a series have been added by an external source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tvdb_id** | Option<**String**> | The tvdbId. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_updated_media

> post_updated_media(post_updated_media_request)
Reports that new movies have been added by an external source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_updated_media_request** | [**PostUpdatedMediaRequest**](PostUpdatedMediaRequest.md) | The update paths. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_updated_movies

> post_updated_movies(tmdb_id, imdb_id)
Reports that new movies have been added by an external source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tmdb_id** | Option<**String**> | The tmdbId. |  |
**imdb_id** | Option<**String**> | The imdbId. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_updated_series

> post_updated_series(tvdb_id)
Reports that new episodes of a series have been added by an external source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tvdb_id** | Option<**String**> | The tvdbId. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_library

> refresh_library()
Starts a library scan.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

