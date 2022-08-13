# LiveStreamResponseMediaSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | Option<[**crate::models::MediaProtocol**](MediaProtocol.md)> |  | [optional]
**id** | Option<**String**> |  | [optional]
**path** | Option<**String**> |  | [optional]
**encoder_path** | Option<**String**> |  | [optional]
**encoder_protocol** | Option<[**crate::models::MediaProtocol**](MediaProtocol.md)> |  | [optional]
**_type** | Option<[**crate::models::MediaSourceType**](MediaSourceType.md)> |  | [optional]
**container** | Option<**String**> |  | [optional]
**size** | Option<**i64**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**is_remote** | Option<**bool**> | Gets or sets a value indicating whether the media is remote.  Differentiate internet url vs local network. | [optional]
**e_tag** | Option<**String**> |  | [optional]
**run_time_ticks** | Option<**i64**> |  | [optional]
**read_at_native_framerate** | Option<**bool**> |  | [optional]
**ignore_dts** | Option<**bool**> |  | [optional]
**ignore_index** | Option<**bool**> |  | [optional]
**gen_pts_input** | Option<**bool**> |  | [optional]
**supports_transcoding** | Option<**bool**> |  | [optional]
**supports_direct_stream** | Option<**bool**> |  | [optional]
**supports_direct_play** | Option<**bool**> |  | [optional]
**is_infinite_stream** | Option<**bool**> |  | [optional]
**requires_opening** | Option<**bool**> |  | [optional]
**open_token** | Option<**String**> |  | [optional]
**requires_closing** | Option<**bool**> |  | [optional]
**live_stream_id** | Option<**String**> |  | [optional]
**buffer_ms** | Option<**i32**> |  | [optional]
**requires_looping** | Option<**bool**> |  | [optional]
**supports_probing** | Option<**bool**> |  | [optional]
**video_type** | Option<[**crate::models::VideoType**](VideoType.md)> |  | [optional]
**iso_type** | Option<[**crate::models::IsoType**](IsoType.md)> |  | [optional]
**video3_d_format** | Option<[**crate::models::Video3DFormat**](Video3DFormat.md)> |  | [optional]
**media_streams** | Option<[**Vec<crate::models::MediaStream>**](MediaStream.md)> |  | [optional]
**media_attachments** | Option<[**Vec<crate::models::MediaAttachment>**](MediaAttachment.md)> |  | [optional]
**formats** | Option<**Vec<String>**> |  | [optional]
**bitrate** | Option<**i32**> |  | [optional]
**timestamp** | Option<[**crate::models::TransportStreamTimestamp**](TransportStreamTimestamp.md)> |  | [optional]
**required_http_headers** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**transcoding_url** | Option<**String**> |  | [optional]
**transcoding_sub_protocol** | Option<**String**> |  | [optional]
**transcoding_container** | Option<**String**> |  | [optional]
**analyze_duration_ms** | Option<**i32**> |  | [optional]
**default_audio_stream_index** | Option<**i32**> |  | [optional]
**default_subtitle_stream_index** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


