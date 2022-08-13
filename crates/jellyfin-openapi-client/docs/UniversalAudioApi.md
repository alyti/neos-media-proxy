# \UniversalAudioApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_universal_audio_stream**](UniversalAudioApi.md#get_universal_audio_stream) | **GET** /Audio/{itemId}/universal | Gets an audio stream.
[**head_universal_audio_stream**](UniversalAudioApi.md#head_universal_audio_stream) | **HEAD** /Audio/{itemId}/universal | Gets an audio stream.



## get_universal_audio_stream

> std::path::PathBuf get_universal_audio_stream(item_id, container, media_source_id, device_id, user_id, audio_codec, max_audio_channels, transcoding_audio_channels, max_streaming_bitrate, audio_bit_rate, start_time_ticks, transcoding_container, transcoding_protocol, max_audio_sample_rate, max_audio_bit_depth, enable_remote_media, break_on_non_key_frames, enable_redirection)
Gets an audio stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**container** | Option<[**Vec<String>**](String.md)> | Optional. The audio container. |  |
**media_source_id** | Option<**String**> | The media version id, if playing an alternate version. |  |
**device_id** | Option<**String**> | The device id of the client requesting. Used to stop encoding processes when needed. |  |
**user_id** | Option<**String**> | Optional. The user id. |  |
**audio_codec** | Option<**String**> | Optional. The audio codec to transcode to. |  |
**max_audio_channels** | Option<**i32**> | Optional. The maximum number of audio channels. |  |
**transcoding_audio_channels** | Option<**i32**> | Optional. The number of how many audio channels to transcode to. |  |
**max_streaming_bitrate** | Option<**i32**> | Optional. The maximum streaming bitrate. |  |
**audio_bit_rate** | Option<**i32**> | Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults. |  |
**start_time_ticks** | Option<**i64**> | Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms. |  |
**transcoding_container** | Option<**String**> | Optional. The container to transcode to. |  |
**transcoding_protocol** | Option<**String**> | Optional. The transcoding protocol. |  |
**max_audio_sample_rate** | Option<**i32**> | Optional. The maximum audio sample rate. |  |
**max_audio_bit_depth** | Option<**i32**> | Optional. The maximum audio bit depth. |  |
**enable_remote_media** | Option<**bool**> | Optional. Whether to enable remote media. |  |
**break_on_non_key_frames** | Option<**bool**> | Optional. Whether to break on non key frames. |  |[default to false]
**enable_redirection** | Option<**bool**> | Whether to enable redirection. Defaults to true. |  |[default to true]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_universal_audio_stream

> std::path::PathBuf head_universal_audio_stream(item_id, container, media_source_id, device_id, user_id, audio_codec, max_audio_channels, transcoding_audio_channels, max_streaming_bitrate, audio_bit_rate, start_time_ticks, transcoding_container, transcoding_protocol, max_audio_sample_rate, max_audio_bit_depth, enable_remote_media, break_on_non_key_frames, enable_redirection)
Gets an audio stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**container** | Option<[**Vec<String>**](String.md)> | Optional. The audio container. |  |
**media_source_id** | Option<**String**> | The media version id, if playing an alternate version. |  |
**device_id** | Option<**String**> | The device id of the client requesting. Used to stop encoding processes when needed. |  |
**user_id** | Option<**String**> | Optional. The user id. |  |
**audio_codec** | Option<**String**> | Optional. The audio codec to transcode to. |  |
**max_audio_channels** | Option<**i32**> | Optional. The maximum number of audio channels. |  |
**transcoding_audio_channels** | Option<**i32**> | Optional. The number of how many audio channels to transcode to. |  |
**max_streaming_bitrate** | Option<**i32**> | Optional. The maximum streaming bitrate. |  |
**audio_bit_rate** | Option<**i32**> | Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults. |  |
**start_time_ticks** | Option<**i64**> | Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms. |  |
**transcoding_container** | Option<**String**> | Optional. The container to transcode to. |  |
**transcoding_protocol** | Option<**String**> | Optional. The transcoding protocol. |  |
**max_audio_sample_rate** | Option<**i32**> | Optional. The maximum audio sample rate. |  |
**max_audio_bit_depth** | Option<**i32**> | Optional. The maximum audio bit depth. |  |
**enable_remote_media** | Option<**bool**> | Optional. Whether to enable remote media. |  |
**break_on_non_key_frames** | Option<**bool**> | Optional. Whether to break on non key frames. |  |[default to false]
**enable_redirection** | Option<**bool**> | Whether to enable redirection. Defaults to true. |  |[default to true]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

