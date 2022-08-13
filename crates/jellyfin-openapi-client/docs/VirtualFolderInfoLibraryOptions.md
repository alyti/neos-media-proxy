# VirtualFolderInfoLibraryOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable_photos** | Option<**bool**> |  | [optional]
**enable_realtime_monitor** | Option<**bool**> |  | [optional]
**enable_chapter_image_extraction** | Option<**bool**> |  | [optional]
**extract_chapter_images_during_library_scan** | Option<**bool**> |  | [optional]
**path_infos** | Option<[**Vec<crate::models::MediaPathInfo>**](MediaPathInfo.md)> |  | [optional]
**save_local_metadata** | Option<**bool**> |  | [optional]
**enable_internet_providers** | Option<**bool**> |  | [optional]
**enable_automatic_series_grouping** | Option<**bool**> |  | [optional]
**enable_embedded_titles** | Option<**bool**> |  | [optional]
**enable_embedded_episode_infos** | Option<**bool**> |  | [optional]
**automatic_refresh_interval_days** | Option<**i32**> |  | [optional]
**preferred_metadata_language** | Option<**String**> | Gets or sets the preferred metadata language. | [optional]
**metadata_country_code** | Option<**String**> | Gets or sets the metadata country code. | [optional]
**season_zero_display_name** | Option<**String**> |  | [optional]
**metadata_savers** | Option<**Vec<String>**> |  | [optional]
**disabled_local_metadata_readers** | Option<**Vec<String>**> |  | [optional]
**local_metadata_reader_order** | Option<**Vec<String>**> |  | [optional]
**disabled_subtitle_fetchers** | Option<**Vec<String>**> |  | [optional]
**subtitle_fetcher_order** | Option<**Vec<String>**> |  | [optional]
**skip_subtitles_if_embedded_subtitles_present** | Option<**bool**> |  | [optional]
**skip_subtitles_if_audio_track_matches** | Option<**bool**> |  | [optional]
**subtitle_download_languages** | Option<**Vec<String>**> |  | [optional]
**require_perfect_subtitle_match** | Option<**bool**> |  | [optional]
**save_subtitles_with_media** | Option<**bool**> |  | [optional]
**automatically_add_to_collection** | Option<**bool**> |  | [optional]
**allow_embedded_subtitles** | Option<[**crate::models::EmbeddedSubtitleOptions**](EmbeddedSubtitleOptions.md)> | An enum representing the options to disable embedded subs. | [optional]
**type_options** | Option<[**Vec<crate::models::TypeOptions>**](TypeOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


