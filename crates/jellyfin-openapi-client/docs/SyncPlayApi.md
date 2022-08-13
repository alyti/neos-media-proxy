# \SyncPlayApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sync_play_buffering**](SyncPlayApi.md#sync_play_buffering) | **POST** /SyncPlay/Buffering | Notify SyncPlay group that member is buffering.
[**sync_play_create_group**](SyncPlayApi.md#sync_play_create_group) | **POST** /SyncPlay/New | Create a new SyncPlay group.
[**sync_play_get_groups**](SyncPlayApi.md#sync_play_get_groups) | **GET** /SyncPlay/List | Gets all SyncPlay groups.
[**sync_play_join_group**](SyncPlayApi.md#sync_play_join_group) | **POST** /SyncPlay/Join | Join an existing SyncPlay group.
[**sync_play_leave_group**](SyncPlayApi.md#sync_play_leave_group) | **POST** /SyncPlay/Leave | Leave the joined SyncPlay group.
[**sync_play_move_playlist_item**](SyncPlayApi.md#sync_play_move_playlist_item) | **POST** /SyncPlay/MovePlaylistItem | Request to move an item in the playlist in SyncPlay group.
[**sync_play_next_item**](SyncPlayApi.md#sync_play_next_item) | **POST** /SyncPlay/NextItem | Request next item in SyncPlay group.
[**sync_play_pause**](SyncPlayApi.md#sync_play_pause) | **POST** /SyncPlay/Pause | Request pause in SyncPlay group.
[**sync_play_ping**](SyncPlayApi.md#sync_play_ping) | **POST** /SyncPlay/Ping | Update session ping.
[**sync_play_previous_item**](SyncPlayApi.md#sync_play_previous_item) | **POST** /SyncPlay/PreviousItem | Request previous item in SyncPlay group.
[**sync_play_queue**](SyncPlayApi.md#sync_play_queue) | **POST** /SyncPlay/Queue | Request to queue items to the playlist of a SyncPlay group.
[**sync_play_ready**](SyncPlayApi.md#sync_play_ready) | **POST** /SyncPlay/Ready | Notify SyncPlay group that member is ready for playback.
[**sync_play_remove_from_playlist**](SyncPlayApi.md#sync_play_remove_from_playlist) | **POST** /SyncPlay/RemoveFromPlaylist | Request to remove items from the playlist in SyncPlay group.
[**sync_play_seek**](SyncPlayApi.md#sync_play_seek) | **POST** /SyncPlay/Seek | Request seek in SyncPlay group.
[**sync_play_set_ignore_wait**](SyncPlayApi.md#sync_play_set_ignore_wait) | **POST** /SyncPlay/SetIgnoreWait | Request SyncPlay group to ignore member during group-wait.
[**sync_play_set_new_queue**](SyncPlayApi.md#sync_play_set_new_queue) | **POST** /SyncPlay/SetNewQueue | Request to set new playlist in SyncPlay group.
[**sync_play_set_playlist_item**](SyncPlayApi.md#sync_play_set_playlist_item) | **POST** /SyncPlay/SetPlaylistItem | Request to change playlist item in SyncPlay group.
[**sync_play_set_repeat_mode**](SyncPlayApi.md#sync_play_set_repeat_mode) | **POST** /SyncPlay/SetRepeatMode | Request to set repeat mode in SyncPlay group.
[**sync_play_set_shuffle_mode**](SyncPlayApi.md#sync_play_set_shuffle_mode) | **POST** /SyncPlay/SetShuffleMode | Request to set shuffle mode in SyncPlay group.
[**sync_play_stop**](SyncPlayApi.md#sync_play_stop) | **POST** /SyncPlay/Stop | Request stop in SyncPlay group.
[**sync_play_unpause**](SyncPlayApi.md#sync_play_unpause) | **POST** /SyncPlay/Unpause | Request unpause in SyncPlay group.



## sync_play_buffering

> sync_play_buffering(sync_play_buffering_request)
Notify SyncPlay group that member is buffering.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_buffering_request** | [**SyncPlayBufferingRequest**](SyncPlayBufferingRequest.md) | The player status. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_create_group

> sync_play_create_group(sync_play_create_group_request)
Create a new SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_create_group_request** | [**SyncPlayCreateGroupRequest**](SyncPlayCreateGroupRequest.md) | The settings of the new group. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_get_groups

> Vec<crate::models::GroupInfoDto> sync_play_get_groups()
Gets all SyncPlay groups.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GroupInfoDto>**](GroupInfoDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_join_group

> sync_play_join_group(sync_play_join_group_request)
Join an existing SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_join_group_request** | [**SyncPlayJoinGroupRequest**](SyncPlayJoinGroupRequest.md) | The group to join. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_leave_group

> sync_play_leave_group()
Leave the joined SyncPlay group.

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


## sync_play_move_playlist_item

> sync_play_move_playlist_item(sync_play_move_playlist_item_request)
Request to move an item in the playlist in SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_move_playlist_item_request** | [**SyncPlayMovePlaylistItemRequest**](SyncPlayMovePlaylistItemRequest.md) | The new position for the item. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_next_item

> sync_play_next_item(sync_play_next_item_request)
Request next item in SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_next_item_request** | [**SyncPlayNextItemRequest**](SyncPlayNextItemRequest.md) | The current item information. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_pause

> sync_play_pause()
Request pause in SyncPlay group.

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


## sync_play_ping

> sync_play_ping(sync_play_ping_request)
Update session ping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_ping_request** | [**SyncPlayPingRequest**](SyncPlayPingRequest.md) | The new ping. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_previous_item

> sync_play_previous_item(sync_play_previous_item_request)
Request previous item in SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_previous_item_request** | [**SyncPlayPreviousItemRequest**](SyncPlayPreviousItemRequest.md) | The current item information. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_queue

> sync_play_queue(sync_play_queue_request)
Request to queue items to the playlist of a SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_queue_request** | [**SyncPlayQueueRequest**](SyncPlayQueueRequest.md) | The items to add. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_ready

> sync_play_ready(sync_play_ready_request)
Notify SyncPlay group that member is ready for playback.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_ready_request** | [**SyncPlayReadyRequest**](SyncPlayReadyRequest.md) | The player status. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_remove_from_playlist

> sync_play_remove_from_playlist(sync_play_remove_from_playlist_request)
Request to remove items from the playlist in SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_remove_from_playlist_request** | [**SyncPlayRemoveFromPlaylistRequest**](SyncPlayRemoveFromPlaylistRequest.md) | The items to remove. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_seek

> sync_play_seek(sync_play_seek_request)
Request seek in SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_seek_request** | [**SyncPlaySeekRequest**](SyncPlaySeekRequest.md) | The new playback position. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_set_ignore_wait

> sync_play_set_ignore_wait(sync_play_set_ignore_wait_request)
Request SyncPlay group to ignore member during group-wait.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_set_ignore_wait_request** | [**SyncPlaySetIgnoreWaitRequest**](SyncPlaySetIgnoreWaitRequest.md) | The settings to set. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_set_new_queue

> sync_play_set_new_queue(sync_play_set_new_queue_request)
Request to set new playlist in SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_set_new_queue_request** | [**SyncPlaySetNewQueueRequest**](SyncPlaySetNewQueueRequest.md) | The new playlist to play in the group. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_set_playlist_item

> sync_play_set_playlist_item(sync_play_set_playlist_item_request)
Request to change playlist item in SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_set_playlist_item_request** | [**SyncPlaySetPlaylistItemRequest**](SyncPlaySetPlaylistItemRequest.md) | The new item to play. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_set_repeat_mode

> sync_play_set_repeat_mode(sync_play_set_repeat_mode_request)
Request to set repeat mode in SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_set_repeat_mode_request** | [**SyncPlaySetRepeatModeRequest**](SyncPlaySetRepeatModeRequest.md) | The new repeat mode. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_set_shuffle_mode

> sync_play_set_shuffle_mode(sync_play_set_shuffle_mode_request)
Request to set shuffle mode in SyncPlay group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_play_set_shuffle_mode_request** | [**SyncPlaySetShuffleModeRequest**](SyncPlaySetShuffleModeRequest.md) | The new shuffle mode. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_play_stop

> sync_play_stop()
Request stop in SyncPlay group.

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


## sync_play_unpause

> sync_play_unpause()
Request unpause in SyncPlay group.

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

