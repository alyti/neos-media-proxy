# \PlaystateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mark_played_item**](PlaystateApi.md#mark_played_item) | **POST** /Users/{userId}/PlayedItems/{itemId} | Marks an item as played for user.
[**mark_unplayed_item**](PlaystateApi.md#mark_unplayed_item) | **DELETE** /Users/{userId}/PlayedItems/{itemId} | Marks an item as unplayed for user.
[**on_playback_progress**](PlaystateApi.md#on_playback_progress) | **POST** /Users/{userId}/PlayingItems/{itemId}/Progress | Reports a user's playback progress.
[**on_playback_start**](PlaystateApi.md#on_playback_start) | **POST** /Users/{userId}/PlayingItems/{itemId} | Reports that a user has begun playing an item.
[**on_playback_stopped**](PlaystateApi.md#on_playback_stopped) | **DELETE** /Users/{userId}/PlayingItems/{itemId} | Reports that a user has stopped playing an item.
[**ping_playback_session**](PlaystateApi.md#ping_playback_session) | **POST** /Sessions/Playing/Ping | Pings a playback session.
[**report_playback_progress**](PlaystateApi.md#report_playback_progress) | **POST** /Sessions/Playing/Progress | Reports playback progress within a session.
[**report_playback_start**](PlaystateApi.md#report_playback_start) | **POST** /Sessions/Playing | Reports playback has started within a session.
[**report_playback_stopped**](PlaystateApi.md#report_playback_stopped) | **POST** /Sessions/Playing/Stopped | Reports playback has stopped within a session.



## mark_played_item

> crate::models::UserItemDataDto mark_played_item(user_id, item_id, date_played)
Marks an item as played for user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |
**date_played** | Option<**String**> | Optional. The date the item was played. |  |

### Return type

[**crate::models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_unplayed_item

> crate::models::UserItemDataDto mark_unplayed_item(user_id, item_id)
Marks an item as unplayed for user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |

### Return type

[**crate::models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_playback_progress

> on_playback_progress(user_id, item_id, media_source_id, position_ticks, audio_stream_index, subtitle_stream_index, volume_level, play_method, live_stream_id, play_session_id, repeat_mode, is_paused, is_muted)
Reports a user's playback progress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |
**media_source_id** | Option<**String**> | The id of the MediaSource. |  |
**position_ticks** | Option<**i64**> | Optional. The current position, in ticks. 1 tick = 10000 ms. |  |
**audio_stream_index** | Option<**i32**> | The audio stream index. |  |
**subtitle_stream_index** | Option<**i32**> | The subtitle stream index. |  |
**volume_level** | Option<**i32**> | Scale of 0-100. |  |
**play_method** | Option<[**crate::models::PlayMethod**](.md)> | The play method. |  |
**live_stream_id** | Option<**String**> | The live stream id. |  |
**play_session_id** | Option<**String**> | The play session id. |  |
**repeat_mode** | Option<[**crate::models::RepeatMode**](.md)> | The repeat mode. |  |
**is_paused** | Option<**bool**> | Indicates if the player is paused. |  |[default to false]
**is_muted** | Option<**bool**> | Indicates if the player is muted. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_playback_start

> on_playback_start(user_id, item_id, media_source_id, audio_stream_index, subtitle_stream_index, play_method, live_stream_id, play_session_id, can_seek)
Reports that a user has begun playing an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |
**media_source_id** | Option<**String**> | The id of the MediaSource. |  |
**audio_stream_index** | Option<**i32**> | The audio stream index. |  |
**subtitle_stream_index** | Option<**i32**> | The subtitle stream index. |  |
**play_method** | Option<[**crate::models::PlayMethod**](.md)> | The play method. |  |
**live_stream_id** | Option<**String**> | The live stream id. |  |
**play_session_id** | Option<**String**> | The play session id. |  |
**can_seek** | Option<**bool**> | Indicates if the client can seek. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_playback_stopped

> on_playback_stopped(user_id, item_id, media_source_id, next_media_type, position_ticks, live_stream_id, play_session_id)
Reports that a user has stopped playing an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |
**media_source_id** | Option<**String**> | The id of the MediaSource. |  |
**next_media_type** | Option<**String**> | The next media type that will play. |  |
**position_ticks** | Option<**i64**> | Optional. The position, in ticks, where playback stopped. 1 tick = 10000 ms. |  |
**live_stream_id** | Option<**String**> | The live stream id. |  |
**play_session_id** | Option<**String**> | The play session id. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping_playback_session

> ping_playback_session(play_session_id)
Pings a playback session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**play_session_id** | **String** | Playback session id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_playback_progress

> report_playback_progress(report_playback_progress_request)
Reports playback progress within a session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_playback_progress_request** | Option<[**ReportPlaybackProgressRequest**](ReportPlaybackProgressRequest.md)> | The playback progress info. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_playback_start

> report_playback_start(report_playback_start_request)
Reports playback has started within a session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_playback_start_request** | Option<[**ReportPlaybackStartRequest**](ReportPlaybackStartRequest.md)> | The playback start info. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_playback_stopped

> report_playback_stopped(report_playback_stopped_request)
Reports playback has stopped within a session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_playback_stopped_request** | Option<[**ReportPlaybackStoppedRequest**](ReportPlaybackStoppedRequest.md)> | The playback stop info. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

