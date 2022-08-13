# \AudioApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_audio_stream**](AudioApi.md#get_audio_stream) | **GET** /Audio/{itemId}/stream | Gets an audio stream.
[**get_audio_stream_by_container**](AudioApi.md#get_audio_stream_by_container) | **GET** /Audio/{itemId}/stream.{container} | Gets an audio stream.
[**head_audio_stream**](AudioApi.md#head_audio_stream) | **HEAD** /Audio/{itemId}/stream | Gets an audio stream.
[**head_audio_stream_by_container**](AudioApi.md#head_audio_stream_by_container) | **HEAD** /Audio/{itemId}/stream.{container} | Gets an audio stream.



## get_audio_stream

> std::path::PathBuf get_audio_stream(item_id, container, _static, params, tag, device_profile_id, play_session_id, segment_container, segment_length, min_segments, media_source_id, device_id, audio_codec, enable_auto_stream_copy, allow_video_stream_copy, allow_audio_stream_copy, break_on_non_key_frames, audio_sample_rate, max_audio_bit_depth, audio_bit_rate, audio_channels, max_audio_channels, profile, level, framerate, max_framerate, copy_timestamps, start_time_ticks, width, height, video_bit_rate, subtitle_stream_index, subtitle_method, max_ref_frames, max_video_bit_depth, require_avc, de_interlace, require_non_anamorphic, transcoding_max_audio_channels, cpu_core_limit, live_stream_id, enable_mpegts_m2_ts_mode, video_codec, subtitle_codec, transcode_reasons, audio_stream_index, video_stream_index, context, stream_options)
Gets an audio stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**container** | Option<**String**> | The audio container. |  |
**_static** | Option<**bool**> | Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false. |  |
**params** | Option<**String**> | The streaming parameters. |  |
**tag** | Option<**String**> | The tag. |  |
**device_profile_id** | Option<**String**> | Optional. The dlna device profile id to utilize. |  |
**play_session_id** | Option<**String**> | The play session id. |  |
**segment_container** | Option<**String**> | The segment container. |  |
**segment_length** | Option<**i32**> | The segment length. |  |
**min_segments** | Option<**i32**> | The minimum number of segments. |  |
**media_source_id** | Option<**String**> | The media version id, if playing an alternate version. |  |
**device_id** | Option<**String**> | The device id of the client requesting. Used to stop encoding processes when needed. |  |
**audio_codec** | Option<**String**> | Optional. Specify a audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension. Options: aac, mp3, vorbis, wma. |  |
**enable_auto_stream_copy** | Option<**bool**> | Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true. |  |
**allow_video_stream_copy** | Option<**bool**> | Whether or not to allow copying of the video stream url. |  |
**allow_audio_stream_copy** | Option<**bool**> | Whether or not to allow copying of the audio stream url. |  |
**break_on_non_key_frames** | Option<**bool**> | Optional. Whether to break on non key frames. |  |
**audio_sample_rate** | Option<**i32**> | Optional. Specify a specific audio sample rate, e.g. 44100. |  |
**max_audio_bit_depth** | Option<**i32**> | Optional. The maximum audio bit depth. |  |
**audio_bit_rate** | Option<**i32**> | Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults. |  |
**audio_channels** | Option<**i32**> | Optional. Specify a specific number of audio channels to encode to, e.g. 2. |  |
**max_audio_channels** | Option<**i32**> | Optional. Specify a maximum number of audio channels to encode to, e.g. 2. |  |
**profile** | Option<**String**> | Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high. |  |
**level** | Option<**String**> | Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1. |  |
**framerate** | Option<**f32**> | Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements. |  |
**max_framerate** | Option<**f32**> | Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements. |  |
**copy_timestamps** | Option<**bool**> | Whether or not to copy timestamps when transcoding with an offset. Defaults to false. |  |
**start_time_ticks** | Option<**i64**> | Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms. |  |
**width** | Option<**i32**> | Optional. The fixed horizontal resolution of the encoded video. |  |
**height** | Option<**i32**> | Optional. The fixed vertical resolution of the encoded video. |  |
**video_bit_rate** | Option<**i32**> | Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults. |  |
**subtitle_stream_index** | Option<**i32**> | Optional. The index of the subtitle stream to use. If omitted no subtitles will be used. |  |
**subtitle_method** | Option<[**crate::models::SubtitleDeliveryMethod**](.md)> | Optional. Specify the subtitle delivery method. |  |
**max_ref_frames** | Option<**i32**> | Optional. |  |
**max_video_bit_depth** | Option<**i32**> | Optional. The maximum video bit depth. |  |
**require_avc** | Option<**bool**> | Optional. Whether to require avc. |  |
**de_interlace** | Option<**bool**> | Optional. Whether to deinterlace the video. |  |
**require_non_anamorphic** | Option<**bool**> | Optional. Whether to require a non anamorphic stream. |  |
**transcoding_max_audio_channels** | Option<**i32**> | Optional. The maximum number of audio channels to transcode. |  |
**cpu_core_limit** | Option<**i32**> | Optional. The limit of how many cpu cores to use. |  |
**live_stream_id** | Option<**String**> | The live stream id. |  |
**enable_mpegts_m2_ts_mode** | Option<**bool**> | Optional. Whether to enable the MpegtsM2Ts mode. |  |
**video_codec** | Option<**String**> | Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension. Options: h265, h264, mpeg4, theora, vp8, vp9, vpx (deprecated), wmv. |  |
**subtitle_codec** | Option<**String**> | Optional. Specify a subtitle codec to encode to. |  |
**transcode_reasons** | Option<**String**> | Optional. The transcoding reason. |  |
**audio_stream_index** | Option<**i32**> | Optional. The index of the audio stream to use. If omitted the first audio stream will be used. |  |
**video_stream_index** | Option<**i32**> | Optional. The index of the video stream to use. If omitted the first video stream will be used. |  |
**context** | Option<[**crate::models::EncodingContext**](.md)> | Optional. The MediaBrowser.Model.Dlna.EncodingContext. |  |
**stream_options** | Option<[**::std::collections::HashMap<String, String>**](String.md)> | Optional. The streaming options. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audio_stream_by_container

> std::path::PathBuf get_audio_stream_by_container(item_id, container, _static, params, tag, device_profile_id, play_session_id, segment_container, segment_length, min_segments, media_source_id, device_id, audio_codec, enable_auto_stream_copy, allow_video_stream_copy, allow_audio_stream_copy, break_on_non_key_frames, audio_sample_rate, max_audio_bit_depth, audio_bit_rate, audio_channels, max_audio_channels, profile, level, framerate, max_framerate, copy_timestamps, start_time_ticks, width, height, video_bit_rate, subtitle_stream_index, subtitle_method, max_ref_frames, max_video_bit_depth, require_avc, de_interlace, require_non_anamorphic, transcoding_max_audio_channels, cpu_core_limit, live_stream_id, enable_mpegts_m2_ts_mode, video_codec, subtitle_codec, transcode_reasons, audio_stream_index, video_stream_index, context, stream_options)
Gets an audio stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**container** | **String** | The audio container. | [required] |
**_static** | Option<**bool**> | Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false. |  |
**params** | Option<**String**> | The streaming parameters. |  |
**tag** | Option<**String**> | The tag. |  |
**device_profile_id** | Option<**String**> | Optional. The dlna device profile id to utilize. |  |
**play_session_id** | Option<**String**> | The play session id. |  |
**segment_container** | Option<**String**> | The segment container. |  |
**segment_length** | Option<**i32**> | The segment lenght. |  |
**min_segments** | Option<**i32**> | The minimum number of segments. |  |
**media_source_id** | Option<**String**> | The media version id, if playing an alternate version. |  |
**device_id** | Option<**String**> | The device id of the client requesting. Used to stop encoding processes when needed. |  |
**audio_codec** | Option<**String**> | Optional. Specify a audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension. Options: aac, mp3, vorbis, wma. |  |
**enable_auto_stream_copy** | Option<**bool**> | Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true. |  |
**allow_video_stream_copy** | Option<**bool**> | Whether or not to allow copying of the video stream url. |  |
**allow_audio_stream_copy** | Option<**bool**> | Whether or not to allow copying of the audio stream url. |  |
**break_on_non_key_frames** | Option<**bool**> | Optional. Whether to break on non key frames. |  |
**audio_sample_rate** | Option<**i32**> | Optional. Specify a specific audio sample rate, e.g. 44100. |  |
**max_audio_bit_depth** | Option<**i32**> | Optional. The maximum audio bit depth. |  |
**audio_bit_rate** | Option<**i32**> | Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults. |  |
**audio_channels** | Option<**i32**> | Optional. Specify a specific number of audio channels to encode to, e.g. 2. |  |
**max_audio_channels** | Option<**i32**> | Optional. Specify a maximum number of audio channels to encode to, e.g. 2. |  |
**profile** | Option<**String**> | Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high. |  |
**level** | Option<**String**> | Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1. |  |
**framerate** | Option<**f32**> | Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements. |  |
**max_framerate** | Option<**f32**> | Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements. |  |
**copy_timestamps** | Option<**bool**> | Whether or not to copy timestamps when transcoding with an offset. Defaults to false. |  |
**start_time_ticks** | Option<**i64**> | Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms. |  |
**width** | Option<**i32**> | Optional. The fixed horizontal resolution of the encoded video. |  |
**height** | Option<**i32**> | Optional. The fixed vertical resolution of the encoded video. |  |
**video_bit_rate** | Option<**i32**> | Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults. |  |
**subtitle_stream_index** | Option<**i32**> | Optional. The index of the subtitle stream to use. If omitted no subtitles will be used. |  |
**subtitle_method** | Option<[**crate::models::SubtitleDeliveryMethod**](.md)> | Optional. Specify the subtitle delivery method. |  |
**max_ref_frames** | Option<**i32**> | Optional. |  |
**max_video_bit_depth** | Option<**i32**> | Optional. The maximum video bit depth. |  |
**require_avc** | Option<**bool**> | Optional. Whether to require avc. |  |
**de_interlace** | Option<**bool**> | Optional. Whether to deinterlace the video. |  |
**require_non_anamorphic** | Option<**bool**> | Optional. Whether to require a non anamporphic stream. |  |
**transcoding_max_audio_channels** | Option<**i32**> | Optional. The maximum number of audio channels to transcode. |  |
**cpu_core_limit** | Option<**i32**> | Optional. The limit of how many cpu cores to use. |  |
**live_stream_id** | Option<**String**> | The live stream id. |  |
**enable_mpegts_m2_ts_mode** | Option<**bool**> | Optional. Whether to enable the MpegtsM2Ts mode. |  |
**video_codec** | Option<**String**> | Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension. Options: h265, h264, mpeg4, theora, vp8, vp9, vpx (deprecated), wmv. |  |
**subtitle_codec** | Option<**String**> | Optional. Specify a subtitle codec to encode to. |  |
**transcode_reasons** | Option<**String**> | Optional. The transcoding reason. |  |
**audio_stream_index** | Option<**i32**> | Optional. The index of the audio stream to use. If omitted the first audio stream will be used. |  |
**video_stream_index** | Option<**i32**> | Optional. The index of the video stream to use. If omitted the first video stream will be used. |  |
**context** | Option<[**crate::models::EncodingContext**](.md)> | Optional. The MediaBrowser.Model.Dlna.EncodingContext. |  |
**stream_options** | Option<[**::std::collections::HashMap<String, String>**](String.md)> | Optional. The streaming options. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_audio_stream

> std::path::PathBuf head_audio_stream(item_id, container, _static, params, tag, device_profile_id, play_session_id, segment_container, segment_length, min_segments, media_source_id, device_id, audio_codec, enable_auto_stream_copy, allow_video_stream_copy, allow_audio_stream_copy, break_on_non_key_frames, audio_sample_rate, max_audio_bit_depth, audio_bit_rate, audio_channels, max_audio_channels, profile, level, framerate, max_framerate, copy_timestamps, start_time_ticks, width, height, video_bit_rate, subtitle_stream_index, subtitle_method, max_ref_frames, max_video_bit_depth, require_avc, de_interlace, require_non_anamorphic, transcoding_max_audio_channels, cpu_core_limit, live_stream_id, enable_mpegts_m2_ts_mode, video_codec, subtitle_codec, transcode_reasons, audio_stream_index, video_stream_index, context, stream_options)
Gets an audio stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**container** | Option<**String**> | The audio container. |  |
**_static** | Option<**bool**> | Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false. |  |
**params** | Option<**String**> | The streaming parameters. |  |
**tag** | Option<**String**> | The tag. |  |
**device_profile_id** | Option<**String**> | Optional. The dlna device profile id to utilize. |  |
**play_session_id** | Option<**String**> | The play session id. |  |
**segment_container** | Option<**String**> | The segment container. |  |
**segment_length** | Option<**i32**> | The segment length. |  |
**min_segments** | Option<**i32**> | The minimum number of segments. |  |
**media_source_id** | Option<**String**> | The media version id, if playing an alternate version. |  |
**device_id** | Option<**String**> | The device id of the client requesting. Used to stop encoding processes when needed. |  |
**audio_codec** | Option<**String**> | Optional. Specify a audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension. Options: aac, mp3, vorbis, wma. |  |
**enable_auto_stream_copy** | Option<**bool**> | Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true. |  |
**allow_video_stream_copy** | Option<**bool**> | Whether or not to allow copying of the video stream url. |  |
**allow_audio_stream_copy** | Option<**bool**> | Whether or not to allow copying of the audio stream url. |  |
**break_on_non_key_frames** | Option<**bool**> | Optional. Whether to break on non key frames. |  |
**audio_sample_rate** | Option<**i32**> | Optional. Specify a specific audio sample rate, e.g. 44100. |  |
**max_audio_bit_depth** | Option<**i32**> | Optional. The maximum audio bit depth. |  |
**audio_bit_rate** | Option<**i32**> | Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults. |  |
**audio_channels** | Option<**i32**> | Optional. Specify a specific number of audio channels to encode to, e.g. 2. |  |
**max_audio_channels** | Option<**i32**> | Optional. Specify a maximum number of audio channels to encode to, e.g. 2. |  |
**profile** | Option<**String**> | Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high. |  |
**level** | Option<**String**> | Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1. |  |
**framerate** | Option<**f32**> | Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements. |  |
**max_framerate** | Option<**f32**> | Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements. |  |
**copy_timestamps** | Option<**bool**> | Whether or not to copy timestamps when transcoding with an offset. Defaults to false. |  |
**start_time_ticks** | Option<**i64**> | Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms. |  |
**width** | Option<**i32**> | Optional. The fixed horizontal resolution of the encoded video. |  |
**height** | Option<**i32**> | Optional. The fixed vertical resolution of the encoded video. |  |
**video_bit_rate** | Option<**i32**> | Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults. |  |
**subtitle_stream_index** | Option<**i32**> | Optional. The index of the subtitle stream to use. If omitted no subtitles will be used. |  |
**subtitle_method** | Option<[**crate::models::SubtitleDeliveryMethod**](.md)> | Optional. Specify the subtitle delivery method. |  |
**max_ref_frames** | Option<**i32**> | Optional. |  |
**max_video_bit_depth** | Option<**i32**> | Optional. The maximum video bit depth. |  |
**require_avc** | Option<**bool**> | Optional. Whether to require avc. |  |
**de_interlace** | Option<**bool**> | Optional. Whether to deinterlace the video. |  |
**require_non_anamorphic** | Option<**bool**> | Optional. Whether to require a non anamorphic stream. |  |
**transcoding_max_audio_channels** | Option<**i32**> | Optional. The maximum number of audio channels to transcode. |  |
**cpu_core_limit** | Option<**i32**> | Optional. The limit of how many cpu cores to use. |  |
**live_stream_id** | Option<**String**> | The live stream id. |  |
**enable_mpegts_m2_ts_mode** | Option<**bool**> | Optional. Whether to enable the MpegtsM2Ts mode. |  |
**video_codec** | Option<**String**> | Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension. Options: h265, h264, mpeg4, theora, vp8, vp9, vpx (deprecated), wmv. |  |
**subtitle_codec** | Option<**String**> | Optional. Specify a subtitle codec to encode to. |  |
**transcode_reasons** | Option<**String**> | Optional. The transcoding reason. |  |
**audio_stream_index** | Option<**i32**> | Optional. The index of the audio stream to use. If omitted the first audio stream will be used. |  |
**video_stream_index** | Option<**i32**> | Optional. The index of the video stream to use. If omitted the first video stream will be used. |  |
**context** | Option<[**crate::models::EncodingContext**](.md)> | Optional. The MediaBrowser.Model.Dlna.EncodingContext. |  |
**stream_options** | Option<[**::std::collections::HashMap<String, String>**](String.md)> | Optional. The streaming options. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_audio_stream_by_container

> std::path::PathBuf head_audio_stream_by_container(item_id, container, _static, params, tag, device_profile_id, play_session_id, segment_container, segment_length, min_segments, media_source_id, device_id, audio_codec, enable_auto_stream_copy, allow_video_stream_copy, allow_audio_stream_copy, break_on_non_key_frames, audio_sample_rate, max_audio_bit_depth, audio_bit_rate, audio_channels, max_audio_channels, profile, level, framerate, max_framerate, copy_timestamps, start_time_ticks, width, height, video_bit_rate, subtitle_stream_index, subtitle_method, max_ref_frames, max_video_bit_depth, require_avc, de_interlace, require_non_anamorphic, transcoding_max_audio_channels, cpu_core_limit, live_stream_id, enable_mpegts_m2_ts_mode, video_codec, subtitle_codec, transcode_reasons, audio_stream_index, video_stream_index, context, stream_options)
Gets an audio stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | The item id. | [required] |
**container** | **String** | The audio container. | [required] |
**_static** | Option<**bool**> | Optional. If true, the original file will be streamed statically without any encoding. Use either no url extension or the original file extension. true/false. |  |
**params** | Option<**String**> | The streaming parameters. |  |
**tag** | Option<**String**> | The tag. |  |
**device_profile_id** | Option<**String**> | Optional. The dlna device profile id to utilize. |  |
**play_session_id** | Option<**String**> | The play session id. |  |
**segment_container** | Option<**String**> | The segment container. |  |
**segment_length** | Option<**i32**> | The segment lenght. |  |
**min_segments** | Option<**i32**> | The minimum number of segments. |  |
**media_source_id** | Option<**String**> | The media version id, if playing an alternate version. |  |
**device_id** | Option<**String**> | The device id of the client requesting. Used to stop encoding processes when needed. |  |
**audio_codec** | Option<**String**> | Optional. Specify a audio codec to encode to, e.g. mp3. If omitted the server will auto-select using the url's extension. Options: aac, mp3, vorbis, wma. |  |
**enable_auto_stream_copy** | Option<**bool**> | Whether or not to allow automatic stream copy if requested values match the original source. Defaults to true. |  |
**allow_video_stream_copy** | Option<**bool**> | Whether or not to allow copying of the video stream url. |  |
**allow_audio_stream_copy** | Option<**bool**> | Whether or not to allow copying of the audio stream url. |  |
**break_on_non_key_frames** | Option<**bool**> | Optional. Whether to break on non key frames. |  |
**audio_sample_rate** | Option<**i32**> | Optional. Specify a specific audio sample rate, e.g. 44100. |  |
**max_audio_bit_depth** | Option<**i32**> | Optional. The maximum audio bit depth. |  |
**audio_bit_rate** | Option<**i32**> | Optional. Specify an audio bitrate to encode to, e.g. 128000. If omitted this will be left to encoder defaults. |  |
**audio_channels** | Option<**i32**> | Optional. Specify a specific number of audio channels to encode to, e.g. 2. |  |
**max_audio_channels** | Option<**i32**> | Optional. Specify a maximum number of audio channels to encode to, e.g. 2. |  |
**profile** | Option<**String**> | Optional. Specify a specific an encoder profile (varies by encoder), e.g. main, baseline, high. |  |
**level** | Option<**String**> | Optional. Specify a level for the encoder profile (varies by encoder), e.g. 3, 3.1. |  |
**framerate** | Option<**f32**> | Optional. A specific video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements. |  |
**max_framerate** | Option<**f32**> | Optional. A specific maximum video framerate to encode to, e.g. 23.976. Generally this should be omitted unless the device has specific requirements. |  |
**copy_timestamps** | Option<**bool**> | Whether or not to copy timestamps when transcoding with an offset. Defaults to false. |  |
**start_time_ticks** | Option<**i64**> | Optional. Specify a starting offset, in ticks. 1 tick = 10000 ms. |  |
**width** | Option<**i32**> | Optional. The fixed horizontal resolution of the encoded video. |  |
**height** | Option<**i32**> | Optional. The fixed vertical resolution of the encoded video. |  |
**video_bit_rate** | Option<**i32**> | Optional. Specify a video bitrate to encode to, e.g. 500000. If omitted this will be left to encoder defaults. |  |
**subtitle_stream_index** | Option<**i32**> | Optional. The index of the subtitle stream to use. If omitted no subtitles will be used. |  |
**subtitle_method** | Option<[**crate::models::SubtitleDeliveryMethod**](.md)> | Optional. Specify the subtitle delivery method. |  |
**max_ref_frames** | Option<**i32**> | Optional. |  |
**max_video_bit_depth** | Option<**i32**> | Optional. The maximum video bit depth. |  |
**require_avc** | Option<**bool**> | Optional. Whether to require avc. |  |
**de_interlace** | Option<**bool**> | Optional. Whether to deinterlace the video. |  |
**require_non_anamorphic** | Option<**bool**> | Optional. Whether to require a non anamporphic stream. |  |
**transcoding_max_audio_channels** | Option<**i32**> | Optional. The maximum number of audio channels to transcode. |  |
**cpu_core_limit** | Option<**i32**> | Optional. The limit of how many cpu cores to use. |  |
**live_stream_id** | Option<**String**> | The live stream id. |  |
**enable_mpegts_m2_ts_mode** | Option<**bool**> | Optional. Whether to enable the MpegtsM2Ts mode. |  |
**video_codec** | Option<**String**> | Optional. Specify a video codec to encode to, e.g. h264. If omitted the server will auto-select using the url's extension. Options: h265, h264, mpeg4, theora, vp8, vp9, vpx (deprecated), wmv. |  |
**subtitle_codec** | Option<**String**> | Optional. Specify a subtitle codec to encode to. |  |
**transcode_reasons** | Option<**String**> | Optional. The transcoding reason. |  |
**audio_stream_index** | Option<**i32**> | Optional. The index of the audio stream to use. If omitted the first audio stream will be used. |  |
**video_stream_index** | Option<**i32**> | Optional. The index of the video stream to use. If omitted the first video stream will be used. |  |
**context** | Option<[**crate::models::EncodingContext**](.md)> | Optional. The MediaBrowser.Model.Dlna.EncodingContext. |  |
**stream_options** | Option<[**::std::collections::HashMap<String, String>**](String.md)> | Optional. The streaming options. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

