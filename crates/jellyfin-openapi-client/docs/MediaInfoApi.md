# \MediaInfoApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**close_live_stream**](MediaInfoApi.md#close_live_stream) | **POST** /LiveStreams/Close | Closes a media source.
[**get_bitrate_test_bytes**](MediaInfoApi.md#get_bitrate_test_bytes) | **GET** /Playback/BitrateTest | Tests the network with a request with the size of the bitrate.
[**get_playback_info**](MediaInfoApi.md#get_playback_info) | **GET** /Items/{itemId}/PlaybackInfo | Gets live playback media info for an item.
[**get_posted_playback_info**](MediaInfoApi.md#get_posted_playback_info) | **POST** /Items/{itemId}/PlaybackInfo | Gets live playback media info for an item.
[**open_live_stream**](MediaInfoApi.md#open_live_stream) | **POST** /LiveStreams/Open | Opens a media source.



## close_live_stream

> close_live_stream(live_stream_id)
Closes a media source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**live_stream_id** | **String** | The livestream id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bitrate_test_bytes

> std::path::PathBuf get_bitrate_test_bytes(size)
Tests the network with a request with the size of the bitrate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**size** | Option<**i32**> | The bitrate. Defaults to 102400. |  |[default to 102400]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playback_info

> crate::models::PlaybackInfoResponse get_playback_info(item_id, user_id)
Gets live playback media info for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**user_id** | **String** | The user id. | [required] |

### Return type

[**crate::models::PlaybackInfoResponse**](PlaybackInfoResponse.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_posted_playback_info

> crate::models::PlaybackInfoResponse get_posted_playback_info(item_id, user_id, max_streaming_bitrate, start_time_ticks, audio_stream_index, subtitle_stream_index, max_audio_channels, media_source_id, live_stream_id, auto_open_live_stream, enable_direct_play, enable_direct_stream, enable_transcoding, allow_video_stream_copy, allow_audio_stream_copy, get_posted_playback_info_request)
Gets live playback media info for an item.

For backwards compatibility parameters can be sent via Query or Body, with Query having higher precedence.  Query parameters are obsolete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | The user id. |  |
**max_streaming_bitrate** | Option<**i32**> | The maximum streaming bitrate. |  |
**start_time_ticks** | Option<**i64**> | The start time in ticks. |  |
**audio_stream_index** | Option<**i32**> | The audio stream index. |  |
**subtitle_stream_index** | Option<**i32**> | The subtitle stream index. |  |
**max_audio_channels** | Option<**i32**> | The maximum number of audio channels. |  |
**media_source_id** | Option<**String**> | The media source id. |  |
**live_stream_id** | Option<**String**> | The livestream id. |  |
**auto_open_live_stream** | Option<**bool**> | Whether to auto open the livestream. |  |
**enable_direct_play** | Option<**bool**> | Whether to enable direct play. Default: true. |  |
**enable_direct_stream** | Option<**bool**> | Whether to enable direct stream. Default: true. |  |
**enable_transcoding** | Option<**bool**> | Whether to enable transcoding. Default: true. |  |
**allow_video_stream_copy** | Option<**bool**> | Whether to allow to copy the video stream. Default: true. |  |
**allow_audio_stream_copy** | Option<**bool**> | Whether to allow to copy the audio stream. Default: true. |  |
**get_posted_playback_info_request** | Option<[**GetPostedPlaybackInfoRequest**](GetPostedPlaybackInfoRequest.md)> | The playback info. |  |

### Return type

[**crate::models::PlaybackInfoResponse**](PlaybackInfoResponse.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open_live_stream

> crate::models::LiveStreamResponse open_live_stream(open_token, user_id, play_session_id, max_streaming_bitrate, start_time_ticks, audio_stream_index, subtitle_stream_index, max_audio_channels, item_id, enable_direct_play, enable_direct_stream, open_live_stream_request)
Opens a media source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**open_token** | Option<**String**> | The open token. |  |
**user_id** | Option<**String**> | The user id. |  |
**play_session_id** | Option<**String**> | The play session id. |  |
**max_streaming_bitrate** | Option<**i32**> | The maximum streaming bitrate. |  |
**start_time_ticks** | Option<**i64**> | The start time in ticks. |  |
**audio_stream_index** | Option<**i32**> | The audio stream index. |  |
**subtitle_stream_index** | Option<**i32**> | The subtitle stream index. |  |
**max_audio_channels** | Option<**i32**> | The maximum number of audio channels. |  |
**item_id** | Option<**String**> | The item id. |  |
**enable_direct_play** | Option<**bool**> | Whether to enable direct play. Default: true. |  |
**enable_direct_stream** | Option<**bool**> | Whether to enable direct stream. Default: true. |  |
**open_live_stream_request** | Option<[**OpenLiveStreamRequest**](OpenLiveStreamRequest.md)> | The open live stream dto. |  |

### Return type

[**crate::models::LiveStreamResponse**](LiveStreamResponse.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

