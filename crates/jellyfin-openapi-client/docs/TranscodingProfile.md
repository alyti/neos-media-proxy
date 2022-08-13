# TranscodingProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container** | Option<**String**> |  | [optional]
**_type** | Option<[**crate::models::DlnaProfileType**](DlnaProfileType.md)> |  | [optional]
**video_codec** | Option<**String**> |  | [optional]
**audio_codec** | Option<**String**> |  | [optional]
**protocol** | Option<**String**> |  | [optional]
**estimate_content_length** | Option<**bool**> |  | [optional][default to false]
**enable_mpegts_m2_ts_mode** | Option<**bool**> |  | [optional][default to false]
**transcode_seek_info** | Option<[**crate::models::TranscodeSeekInfo**](TranscodeSeekInfo.md)> |  | [optional]
**copy_timestamps** | Option<**bool**> |  | [optional][default to false]
**context** | Option<[**crate::models::EncodingContext**](EncodingContext.md)> |  | [optional]
**enable_subtitles_in_manifest** | Option<**bool**> |  | [optional][default to false]
**max_audio_channels** | Option<**String**> |  | [optional]
**min_segments** | Option<**i32**> |  | [optional][default to 0]
**segment_length** | Option<**i32**> |  | [optional][default to 0]
**break_on_non_key_frames** | Option<**bool**> |  | [optional][default to false]
**conditions** | Option<[**Vec<crate::models::ProfileCondition>**](ProfileCondition.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


