# UserDtoPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_administrator** | Option<**bool**> | Gets or sets a value indicating whether this instance is administrator. | [optional]
**is_hidden** | Option<**bool**> | Gets or sets a value indicating whether this instance is hidden. | [optional]
**is_disabled** | Option<**bool**> | Gets or sets a value indicating whether this instance is disabled. | [optional]
**max_parental_rating** | Option<**i32**> | Gets or sets the max parental rating. | [optional]
**blocked_tags** | Option<**Vec<String>**> |  | [optional]
**enable_user_preference_access** | Option<**bool**> |  | [optional]
**access_schedules** | Option<[**Vec<crate::models::AccessSchedule>**](AccessSchedule.md)> |  | [optional]
**block_unrated_items** | Option<[**Vec<crate::models::UnratedItem>**](UnratedItem.md)> |  | [optional]
**enable_remote_control_of_other_users** | Option<**bool**> |  | [optional]
**enable_shared_device_control** | Option<**bool**> |  | [optional]
**enable_remote_access** | Option<**bool**> |  | [optional]
**enable_live_tv_management** | Option<**bool**> |  | [optional]
**enable_live_tv_access** | Option<**bool**> |  | [optional]
**enable_media_playback** | Option<**bool**> |  | [optional]
**enable_audio_playback_transcoding** | Option<**bool**> |  | [optional]
**enable_video_playback_transcoding** | Option<**bool**> |  | [optional]
**enable_playback_remuxing** | Option<**bool**> |  | [optional]
**force_remote_source_transcoding** | Option<**bool**> |  | [optional]
**enable_content_deletion** | Option<**bool**> |  | [optional]
**enable_content_deletion_from_folders** | Option<**Vec<String>**> |  | [optional]
**enable_content_downloading** | Option<**bool**> |  | [optional]
**enable_sync_transcoding** | Option<**bool**> | Gets or sets a value indicating whether [enable synchronize]. | [optional]
**enable_media_conversion** | Option<**bool**> |  | [optional]
**enabled_devices** | Option<**Vec<String>**> |  | [optional]
**enable_all_devices** | Option<**bool**> |  | [optional]
**enabled_channels** | Option<**Vec<String>**> |  | [optional]
**enable_all_channels** | Option<**bool**> |  | [optional]
**enabled_folders** | Option<**Vec<String>**> |  | [optional]
**enable_all_folders** | Option<**bool**> |  | [optional]
**invalid_login_attempt_count** | Option<**i32**> |  | [optional]
**login_attempts_before_lockout** | Option<**i32**> |  | [optional]
**max_active_sessions** | Option<**i32**> |  | [optional]
**enable_public_sharing** | Option<**bool**> |  | [optional]
**blocked_media_folders** | Option<**Vec<String>**> |  | [optional]
**blocked_channels** | Option<**Vec<String>**> |  | [optional]
**remote_client_bitrate_limit** | Option<**i32**> |  | [optional]
**authentication_provider_id** | Option<**String**> |  | [optional]
**password_reset_provider_id** | Option<**String**> |  | [optional]
**sync_play_access** | Option<[**crate::models::SyncPlayUserAccessType**](SyncPlayUserAccessType.md)> | Gets or sets a value indicating what SyncPlay features the user can access. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


