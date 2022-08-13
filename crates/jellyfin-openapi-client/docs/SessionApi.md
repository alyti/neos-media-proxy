# \SessionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_to_session**](SessionApi.md#add_user_to_session) | **POST** /Sessions/{sessionId}/User/{userId} | Adds an additional user to a session.
[**display_content**](SessionApi.md#display_content) | **POST** /Sessions/{sessionId}/Viewing | Instructs a session to browse to an item or view.
[**get_auth_providers**](SessionApi.md#get_auth_providers) | **GET** /Auth/Providers | Get all auth providers.
[**get_password_reset_providers**](SessionApi.md#get_password_reset_providers) | **GET** /Auth/PasswordResetProviders | Get all password reset providers.
[**get_sessions**](SessionApi.md#get_sessions) | **GET** /Sessions | Gets a list of sessions.
[**play**](SessionApi.md#play) | **POST** /Sessions/{sessionId}/Playing | Instructs a session to play an item.
[**post_capabilities**](SessionApi.md#post_capabilities) | **POST** /Sessions/Capabilities | Updates capabilities for a device.
[**post_full_capabilities**](SessionApi.md#post_full_capabilities) | **POST** /Sessions/Capabilities/Full | Updates capabilities for a device.
[**remove_user_from_session**](SessionApi.md#remove_user_from_session) | **DELETE** /Sessions/{sessionId}/User/{userId} | Removes an additional user from a session.
[**report_session_ended**](SessionApi.md#report_session_ended) | **POST** /Sessions/Logout | Reports that a session has ended.
[**report_viewing**](SessionApi.md#report_viewing) | **POST** /Sessions/Viewing | Reports that a session is viewing an item.
[**send_full_general_command**](SessionApi.md#send_full_general_command) | **POST** /Sessions/{sessionId}/Command | Issues a full general command to a client.
[**send_general_command**](SessionApi.md#send_general_command) | **POST** /Sessions/{sessionId}/Command/{command} | Issues a general command to a client.
[**send_message_command**](SessionApi.md#send_message_command) | **POST** /Sessions/{sessionId}/Message | Issues a command to a client to display a message to the user.
[**send_playstate_command**](SessionApi.md#send_playstate_command) | **POST** /Sessions/{sessionId}/Playing/{command} | Issues a playstate command to a client.
[**send_system_command**](SessionApi.md#send_system_command) | **POST** /Sessions/{sessionId}/System/{command} | Issues a system command to a client.



## add_user_to_session

> add_user_to_session(session_id, user_id)
Adds an additional user to a session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session id. | [required] |
**user_id** | **String** | The user id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## display_content

> display_content(session_id, item_type, item_id, item_name)
Instructs a session to browse to an item or view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session Id. | [required] |
**item_type** | [**crate::models::BaseItemKind**](.md) | The type of item to browse to. | [required] |
**item_id** | **String** | The Id of the item. | [required] |
**item_name** | **String** | The name of the item. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_auth_providers

> Vec<crate::models::NameIdPair> get_auth_providers()
Get all auth providers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::NameIdPair>**](NameIdPair.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_password_reset_providers

> Vec<crate::models::NameIdPair> get_password_reset_providers()
Get all password reset providers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::NameIdPair>**](NameIdPair.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sessions

> Vec<crate::models::SessionInfo> get_sessions(controllable_by_user_id, device_id, active_within_seconds)
Gets a list of sessions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**controllable_by_user_id** | Option<**String**> | Filter by sessions that a given user is allowed to remote control. |  |
**device_id** | Option<**String**> | Filter by device Id. |  |
**active_within_seconds** | Option<**i32**> | Optional. Filter by sessions that were active in the last n seconds. |  |

### Return type

[**Vec<crate::models::SessionInfo>**](SessionInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## play

> play(session_id, play_command, item_ids, start_position_ticks, media_source_id, audio_stream_index, subtitle_stream_index, start_index)
Instructs a session to play an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session id. | [required] |
**play_command** | [**crate::models::PlayCommand**](.md) | The type of play command to issue (PlayNow, PlayNext, PlayLast). Clients who have not yet implemented play next and play last may play now. | [required] |
**item_ids** | [**Vec<String>**](String.md) | The ids of the items to play, comma delimited. | [required] |
**start_position_ticks** | Option<**i64**> | The starting position of the first item. |  |
**media_source_id** | Option<**String**> | Optional. The media source id. |  |
**audio_stream_index** | Option<**i32**> | Optional. The index of the audio stream to play. |  |
**subtitle_stream_index** | Option<**i32**> | Optional. The index of the subtitle stream to play. |  |
**start_index** | Option<**i32**> | Optional. The start index. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_capabilities

> post_capabilities(id, playable_media_types, supported_commands, supports_media_control, supports_sync, supports_persistent_identifier)
Updates capabilities for a device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The session id. |  |
**playable_media_types** | Option<[**Vec<String>**](String.md)> | A list of playable media types, comma delimited. Audio, Video, Book, Photo. |  |
**supported_commands** | Option<[**Vec<crate::models::GeneralCommandType>**](crate::models::GeneralCommandType.md)> | A list of supported remote control commands, comma delimited. |  |
**supports_media_control** | Option<**bool**> | Determines whether media can be played remotely.. |  |[default to false]
**supports_sync** | Option<**bool**> | Determines whether sync is supported. |  |[default to false]
**supports_persistent_identifier** | Option<**bool**> | Determines whether the device supports a unique identifier. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_full_capabilities

> post_full_capabilities(post_full_capabilities_request, id)
Updates capabilities for a device.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_full_capabilities_request** | [**PostFullCapabilitiesRequest**](PostFullCapabilitiesRequest.md) | The MediaBrowser.Model.Session.ClientCapabilities. | [required] |
**id** | Option<**String**> | The session id. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_session

> remove_user_from_session(session_id, user_id)
Removes an additional user from a session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session id. | [required] |
**user_id** | **String** | The user id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_session_ended

> report_session_ended()
Reports that a session has ended.

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


## report_viewing

> report_viewing(item_id, session_id)
Reports that a session is viewing an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**session_id** | Option<**String**> | The session id. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_full_general_command

> send_full_general_command(session_id, send_full_general_command_request)
Issues a full general command to a client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session id. | [required] |
**send_full_general_command_request** | [**SendFullGeneralCommandRequest**](SendFullGeneralCommandRequest.md) | The MediaBrowser.Model.Session.GeneralCommand. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_general_command

> send_general_command(session_id, command)
Issues a general command to a client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session id. | [required] |
**command** | [**crate::models::GeneralCommandType**](.md) | The command to send. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_message_command

> send_message_command(session_id, send_message_command_request)
Issues a command to a client to display a message to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session id. | [required] |
**send_message_command_request** | [**SendMessageCommandRequest**](SendMessageCommandRequest.md) | The MediaBrowser.Model.Session.MessageCommand object containing Header, Message Text, and TimeoutMs. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_playstate_command

> send_playstate_command(session_id, command, seek_position_ticks, controlling_user_id)
Issues a playstate command to a client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session id. | [required] |
**command** | [**crate::models::PlaystateCommand**](.md) | The MediaBrowser.Model.Session.PlaystateCommand. | [required] |
**seek_position_ticks** | Option<**i64**> | The optional position ticks. |  |
**controlling_user_id** | Option<**String**> | The optional controlling user id. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_system_command

> send_system_command(session_id, command)
Issues a system command to a client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session id. | [required] |
**command** | [**crate::models::GeneralCommandType**](.md) | The command to send. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

