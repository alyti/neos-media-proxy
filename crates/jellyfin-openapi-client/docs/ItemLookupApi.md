# \ItemLookupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_search_criteria**](ItemLookupApi.md#apply_search_criteria) | **POST** /Items/RemoteSearch/Apply/{itemId} | Applies search criteria to an item and refreshes metadata.
[**get_book_remote_search_results**](ItemLookupApi.md#get_book_remote_search_results) | **POST** /Items/RemoteSearch/Book | Get book remote search.
[**get_box_set_remote_search_results**](ItemLookupApi.md#get_box_set_remote_search_results) | **POST** /Items/RemoteSearch/BoxSet | Get box set remote search.
[**get_external_id_infos**](ItemLookupApi.md#get_external_id_infos) | **GET** /Items/{itemId}/ExternalIdInfos | Get the item's external id info.
[**get_movie_remote_search_results**](ItemLookupApi.md#get_movie_remote_search_results) | **POST** /Items/RemoteSearch/Movie | Get movie remote search.
[**get_music_album_remote_search_results**](ItemLookupApi.md#get_music_album_remote_search_results) | **POST** /Items/RemoteSearch/MusicAlbum | Get music album remote search.
[**get_music_artist_remote_search_results**](ItemLookupApi.md#get_music_artist_remote_search_results) | **POST** /Items/RemoteSearch/MusicArtist | Get music artist remote search.
[**get_music_video_remote_search_results**](ItemLookupApi.md#get_music_video_remote_search_results) | **POST** /Items/RemoteSearch/MusicVideo | Get music video remote search.
[**get_person_remote_search_results**](ItemLookupApi.md#get_person_remote_search_results) | **POST** /Items/RemoteSearch/Person | Get person remote search.
[**get_series_remote_search_results**](ItemLookupApi.md#get_series_remote_search_results) | **POST** /Items/RemoteSearch/Series | Get series remote search.
[**get_trailer_remote_search_results**](ItemLookupApi.md#get_trailer_remote_search_results) | **POST** /Items/RemoteSearch/Trailer | Get trailer remote search.



## apply_search_criteria

> apply_search_criteria(item_id, apply_search_criteria_request, replace_all_images)
Applies search criteria to an item and refreshes metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**apply_search_criteria_request** | [**ApplySearchCriteriaRequest**](ApplySearchCriteriaRequest.md) | The remote search result. | [required] |
**replace_all_images** | Option<**bool**> | Optional. Whether or not to replace all images. Default: True. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_book_remote_search_results(get_book_remote_search_results_request)
Get book remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_book_remote_search_results_request** | [**GetBookRemoteSearchResultsRequest**](GetBookRemoteSearchResultsRequest.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_box_set_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_box_set_remote_search_results(get_box_set_remote_search_results_request)
Get box set remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_box_set_remote_search_results_request** | [**GetBoxSetRemoteSearchResultsRequest**](GetBoxSetRemoteSearchResultsRequest.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_id_infos

> Vec<crate::models::ExternalIdInfo> get_external_id_infos(item_id)
Get the item's external id info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |

### Return type

[**Vec<crate::models::ExternalIdInfo>**](ExternalIdInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_movie_remote_search_results(get_movie_remote_search_results_request)
Get movie remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_movie_remote_search_results_request** | [**GetMovieRemoteSearchResultsRequest**](GetMovieRemoteSearchResultsRequest.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_album_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_music_album_remote_search_results(get_music_album_remote_search_results_request)
Get music album remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_music_album_remote_search_results_request** | [**GetMusicAlbumRemoteSearchResultsRequest**](GetMusicAlbumRemoteSearchResultsRequest.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_artist_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_music_artist_remote_search_results(get_music_artist_remote_search_results_request)
Get music artist remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_music_artist_remote_search_results_request** | [**GetMusicArtistRemoteSearchResultsRequest**](GetMusicArtistRemoteSearchResultsRequest.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_video_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_music_video_remote_search_results(get_music_video_remote_search_results_request)
Get music video remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_music_video_remote_search_results_request** | [**GetMusicVideoRemoteSearchResultsRequest**](GetMusicVideoRemoteSearchResultsRequest.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_person_remote_search_results(get_person_remote_search_results_request)
Get person remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_person_remote_search_results_request** | [**GetPersonRemoteSearchResultsRequest**](GetPersonRemoteSearchResultsRequest.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_series_remote_search_results(get_series_remote_search_results_request)
Get series remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_series_remote_search_results_request** | [**GetSeriesRemoteSearchResultsRequest**](GetSeriesRemoteSearchResultsRequest.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trailer_remote_search_results

> Vec<crate::models::RemoteSearchResult> get_trailer_remote_search_results(get_trailer_remote_search_results_request)
Get trailer remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_trailer_remote_search_results_request** | [**GetTrailerRemoteSearchResultsRequest**](GetTrailerRemoteSearchResultsRequest.md) | Remote search query. | [required] |

### Return type

[**Vec<crate::models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

