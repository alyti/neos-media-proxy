# \HlsSegmentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_hls_audio_segment_legacy_aac**](HlsSegmentApi.md#get_hls_audio_segment_legacy_aac) | **GET** /Audio/{itemId}/hls/{segmentId}/stream.aac | Gets the specified audio segment for an audio item.
[**get_hls_audio_segment_legacy_mp3**](HlsSegmentApi.md#get_hls_audio_segment_legacy_mp3) | **GET** /Audio/{itemId}/hls/{segmentId}/stream.mp3 | Gets the specified audio segment for an audio item.
[**get_hls_playlist_legacy**](HlsSegmentApi.md#get_hls_playlist_legacy) | **GET** /Videos/{itemId}/hls/{playlistId}/stream.m3u8 | Gets a hls video playlist.
[**get_hls_video_segment_legacy**](HlsSegmentApi.md#get_hls_video_segment_legacy) | **GET** /Videos/{itemId}/hls/{playlistId}/{segmentId}.{segmentContainer} | Gets a hls video segment.
[**stop_encoding_process**](HlsSegmentApi.md#stop_encoding_process) | **DELETE** /Videos/ActiveEncodings | Stops an active encoding.



## get_hls_audio_segment_legacy_aac

> std::path::PathBuf get_hls_audio_segment_legacy_aac(item_id, segment_id)
Gets the specified audio segment for an audio item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**segment_id** | **String** | The segment id. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hls_audio_segment_legacy_mp3

> std::path::PathBuf get_hls_audio_segment_legacy_mp3(item_id, segment_id)
Gets the specified audio segment for an audio item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**segment_id** | **String** | The segment id. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hls_playlist_legacy

> std::path::PathBuf get_hls_playlist_legacy(item_id, playlist_id)
Gets a hls video playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The video id. | [required] |
**playlist_id** | **String** | The playlist id. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-mpegURL

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hls_video_segment_legacy

> std::path::PathBuf get_hls_video_segment_legacy(item_id, playlist_id, segment_id, segment_container)
Gets a hls video segment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**playlist_id** | **String** | The playlist id. | [required] |
**segment_id** | **String** | The segment id. | [required] |
**segment_container** | **String** | The segment container. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: video/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_encoding_process

> stop_encoding_process(device_id, play_session_id)
Stops an active encoding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | The device id of the client requesting. Used to stop encoding processes when needed. | [required] |
**play_session_id** | **String** | The play session id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

