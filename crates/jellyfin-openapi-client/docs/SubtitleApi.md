# \SubtitleApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_subtitle**](SubtitleApi.md#delete_subtitle) | **DELETE** /Videos/{itemId}/Subtitles/{index} | Deletes an external subtitle file.
[**download_remote_subtitles**](SubtitleApi.md#download_remote_subtitles) | **POST** /Items/{itemId}/RemoteSearch/Subtitles/{subtitleId} | Downloads a remote subtitle.
[**get_fallback_font**](SubtitleApi.md#get_fallback_font) | **GET** /FallbackFont/Fonts/{name} | Gets a fallback font file.
[**get_fallback_font_list**](SubtitleApi.md#get_fallback_font_list) | **GET** /FallbackFont/Fonts | Gets a list of available fallback font files.
[**get_remote_subtitles**](SubtitleApi.md#get_remote_subtitles) | **GET** /Providers/Subtitles/Subtitles/{id} | Gets the remote subtitles.
[**get_subtitle**](SubtitleApi.md#get_subtitle) | **GET** /Videos/{routeItemId}/{routeMediaSourceId}/Subtitles/{routeIndex}/Stream.{routeFormat} | Gets subtitles in a specified format.
[**get_subtitle_playlist**](SubtitleApi.md#get_subtitle_playlist) | **GET** /Videos/{itemId}/{mediaSourceId}/Subtitles/{index}/subtitles.m3u8 | Gets an HLS subtitle playlist.
[**get_subtitle_with_ticks**](SubtitleApi.md#get_subtitle_with_ticks) | **GET** /Videos/{routeItemId}/{routeMediaSourceId}/Subtitles/{routeIndex}/{routeStartPositionTicks}/Stream.{routeFormat} | Gets subtitles in a specified format.
[**search_remote_subtitles**](SubtitleApi.md#search_remote_subtitles) | **GET** /Items/{itemId}/RemoteSearch/Subtitles/{language} | Search remote subtitles.
[**upload_subtitle**](SubtitleApi.md#upload_subtitle) | **POST** /Videos/{itemId}/Subtitles | Upload an external subtitle file.



## delete_subtitle

> delete_subtitle(item_id, index)
Deletes an external subtitle file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**index** | **i32** | The index of the subtitle file. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_remote_subtitles

> download_remote_subtitles(item_id, subtitle_id)
Downloads a remote subtitle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**subtitle_id** | **String** | The subtitle id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fallback_font

> std::path::PathBuf get_fallback_font(name)
Gets a fallback font file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the fallback font file to get. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: font/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fallback_font_list

> Vec<crate::models::FontFile> get_fallback_font_list()
Gets a list of available fallback font files.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::FontFile>**](FontFile.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_remote_subtitles

> std::path::PathBuf get_remote_subtitles(id)
Gets the remote subtitles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The item id. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subtitle

> std::path::PathBuf get_subtitle(route_item_id, route_media_source_id, route_index, route_format, item_id, media_source_id, index, format, end_position_ticks, copy_timestamps, add_vtt_time_map, start_position_ticks)
Gets subtitles in a specified format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_item_id** | **String** | The (route) item id. | [required] |
**route_media_source_id** | **String** | The (route) media source id. | [required] |
**route_index** | **i32** | The (route) subtitle stream index. | [required] |
**route_format** | **String** | The (route) format of the returned subtitle. | [required] |
**item_id** | Option<**String**> | The item id. |  |
**media_source_id** | Option<**String**> | The media source id. |  |
**index** | Option<**i32**> | The subtitle stream index. |  |
**format** | Option<**String**> | The format of the returned subtitle. |  |
**end_position_ticks** | Option<**i64**> | Optional. The end position of the subtitle in ticks. |  |
**copy_timestamps** | Option<**bool**> | Optional. Whether to copy the timestamps. |  |[default to false]
**add_vtt_time_map** | Option<**bool**> | Optional. Whether to add a VTT time map. |  |[default to false]
**start_position_ticks** | Option<**i64**> | The start position of the subtitle in ticks. |  |[default to 0]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subtitle_playlist

> std::path::PathBuf get_subtitle_playlist(item_id, index, media_source_id, segment_length)
Gets an HLS subtitle playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**index** | **i32** | The subtitle stream index. | [required] |
**media_source_id** | **String** | The media source id. | [required] |
**segment_length** | **i32** | The subtitle segment length. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-mpegURL

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subtitle_with_ticks

> std::path::PathBuf get_subtitle_with_ticks(route_item_id, route_media_source_id, route_index, route_start_position_ticks, route_format, item_id, media_source_id, index, start_position_ticks, format, end_position_ticks, copy_timestamps, add_vtt_time_map)
Gets subtitles in a specified format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**route_item_id** | **String** | The (route) item id. | [required] |
**route_media_source_id** | **String** | The (route) media source id. | [required] |
**route_index** | **i32** | The (route) subtitle stream index. | [required] |
**route_start_position_ticks** | **i64** | The (route) start position of the subtitle in ticks. | [required] |
**route_format** | **String** | The (route) format of the returned subtitle. | [required] |
**item_id** | Option<**String**> | The item id. |  |
**media_source_id** | Option<**String**> | The media source id. |  |
**index** | Option<**i32**> | The subtitle stream index. |  |
**start_position_ticks** | Option<**i64**> | The start position of the subtitle in ticks. |  |
**format** | Option<**String**> | The format of the returned subtitle. |  |
**end_position_ticks** | Option<**i64**> | Optional. The end position of the subtitle in ticks. |  |
**copy_timestamps** | Option<**bool**> | Optional. Whether to copy the timestamps. |  |[default to false]
**add_vtt_time_map** | Option<**bool**> | Optional. Whether to add a VTT time map. |  |[default to false]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_remote_subtitles

> Vec<crate::models::RemoteSubtitleInfo> search_remote_subtitles(item_id, language, is_perfect_match)
Search remote subtitles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**language** | **String** | The language of the subtitles. | [required] |
**is_perfect_match** | Option<**bool**> | Optional. Only show subtitles which are a perfect match. |  |

### Return type

[**Vec<crate::models::RemoteSubtitleInfo>**](RemoteSubtitleInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_subtitle

> upload_subtitle(item_id, upload_subtitle_request)
Upload an external subtitle file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item the subtitle belongs to. | [required] |
**upload_subtitle_request** | [**UploadSubtitleRequest**](UploadSubtitleRequest.md) | The request body. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

