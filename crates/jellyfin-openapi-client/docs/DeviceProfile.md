# DeviceProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Gets or sets the name of this device profile. | [optional]
**id** | Option<**String**> | Gets or sets the Id. | [optional]
**identification** | Option<[**crate::models::DeviceProfileIdentification**](DeviceProfile_Identification.md)> |  | [optional]
**friendly_name** | Option<**String**> | Gets or sets the friendly name of the device profile, which can be shown to users. | [optional]
**manufacturer** | Option<**String**> | Gets or sets the manufacturer of the device which this profile represents. | [optional]
**manufacturer_url** | Option<**String**> | Gets or sets an url for the manufacturer of the device which this profile represents. | [optional]
**model_name** | Option<**String**> | Gets or sets the model name of the device which this profile represents. | [optional]
**model_description** | Option<**String**> | Gets or sets the model description of the device which this profile represents. | [optional]
**model_number** | Option<**String**> | Gets or sets the model number of the device which this profile represents. | [optional]
**model_url** | Option<**String**> | Gets or sets the ModelUrl. | [optional]
**serial_number** | Option<**String**> | Gets or sets the serial number of the device which this profile represents. | [optional]
**enable_album_art_in_didl** | Option<**bool**> | Gets or sets a value indicating whether EnableAlbumArtInDidl. | [optional][default to false]
**enable_single_album_art_limit** | Option<**bool**> | Gets or sets a value indicating whether EnableSingleAlbumArtLimit. | [optional][default to false]
**enable_single_subtitle_limit** | Option<**bool**> | Gets or sets a value indicating whether EnableSingleSubtitleLimit. | [optional][default to false]
**supported_media_types** | Option<**String**> | Gets or sets the SupportedMediaTypes. | [optional]
**user_id** | Option<**String**> | Gets or sets the UserId. | [optional]
**album_art_pn** | Option<**String**> | Gets or sets the AlbumArtPn. | [optional]
**max_album_art_width** | Option<**i32**> | Gets or sets the MaxAlbumArtWidth. | [optional]
**max_album_art_height** | Option<**i32**> | Gets or sets the MaxAlbumArtHeight. | [optional]
**max_icon_width** | Option<**i32**> | Gets or sets the maximum allowed width of embedded icons. | [optional]
**max_icon_height** | Option<**i32**> | Gets or sets the maximum allowed height of embedded icons. | [optional]
**max_streaming_bitrate** | Option<**i32**> | Gets or sets the maximum allowed bitrate for all streamed content. | [optional]
**max_static_bitrate** | Option<**i32**> | Gets or sets the maximum allowed bitrate for statically streamed content (= direct played files). | [optional]
**music_streaming_transcoding_bitrate** | Option<**i32**> | Gets or sets the maximum allowed bitrate for transcoded music streams. | [optional]
**max_static_music_bitrate** | Option<**i32**> | Gets or sets the maximum allowed bitrate for statically streamed (= direct played) music files. | [optional]
**sony_aggregation_flags** | Option<**String**> | Gets or sets the content of the aggregationFlags element in the urn:schemas-sonycom:av namespace. | [optional]
**protocol_info** | Option<**String**> | Gets or sets the ProtocolInfo. | [optional]
**timeline_offset_seconds** | Option<**i32**> | Gets or sets the TimelineOffsetSeconds. | [optional][default to 0]
**requires_plain_video_items** | Option<**bool**> | Gets or sets a value indicating whether RequiresPlainVideoItems. | [optional][default to false]
**requires_plain_folders** | Option<**bool**> | Gets or sets a value indicating whether RequiresPlainFolders. | [optional][default to false]
**enable_ms_media_receiver_registrar** | Option<**bool**> | Gets or sets a value indicating whether EnableMSMediaReceiverRegistrar. | [optional][default to false]
**ignore_transcode_byte_range_requests** | Option<**bool**> | Gets or sets a value indicating whether IgnoreTranscodeByteRangeRequests. | [optional][default to false]
**xml_root_attributes** | Option<[**Vec<crate::models::XmlAttribute>**](XmlAttribute.md)> | Gets or sets the XmlRootAttributes. | [optional]
**direct_play_profiles** | Option<[**Vec<crate::models::DirectPlayProfile>**](DirectPlayProfile.md)> | Gets or sets the direct play profiles. | [optional]
**transcoding_profiles** | Option<[**Vec<crate::models::TranscodingProfile>**](TranscodingProfile.md)> | Gets or sets the transcoding profiles. | [optional]
**container_profiles** | Option<[**Vec<crate::models::ContainerProfile>**](ContainerProfile.md)> | Gets or sets the container profiles. | [optional]
**codec_profiles** | Option<[**Vec<crate::models::CodecProfile>**](CodecProfile.md)> | Gets or sets the codec profiles. | [optional]
**response_profiles** | Option<[**Vec<crate::models::ResponseProfile>**](ResponseProfile.md)> | Gets or sets the ResponseProfiles. | [optional]
**subtitle_profiles** | Option<[**Vec<crate::models::SubtitleProfile>**](SubtitleProfile.md)> | Gets or sets the subtitle profiles. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


