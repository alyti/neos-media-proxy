# SeriesTimerInfoDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Gets or sets the Id of the recording. | [optional]
**_type** | Option<**String**> |  | [optional]
**server_id** | Option<**String**> | Gets or sets the server identifier. | [optional]
**external_id** | Option<**String**> | Gets or sets the external identifier. | [optional]
**channel_id** | Option<**String**> | Gets or sets the channel id of the recording. | [optional]
**external_channel_id** | Option<**String**> | Gets or sets the external channel identifier. | [optional]
**channel_name** | Option<**String**> | Gets or sets the channel name of the recording. | [optional]
**channel_primary_image_tag** | Option<**String**> |  | [optional]
**program_id** | Option<**String**> | Gets or sets the program identifier. | [optional]
**external_program_id** | Option<**String**> | Gets or sets the external program identifier. | [optional]
**name** | Option<**String**> | Gets or sets the name of the recording. | [optional]
**overview** | Option<**String**> | Gets or sets the description of the recording. | [optional]
**start_date** | Option<**String**> | Gets or sets the start date of the recording, in UTC. | [optional]
**end_date** | Option<**String**> | Gets or sets the end date of the recording, in UTC. | [optional]
**service_name** | Option<**String**> | Gets or sets the name of the service. | [optional]
**priority** | Option<**i32**> | Gets or sets the priority. | [optional]
**pre_padding_seconds** | Option<**i32**> | Gets or sets the pre padding seconds. | [optional]
**post_padding_seconds** | Option<**i32**> | Gets or sets the post padding seconds. | [optional]
**is_pre_padding_required** | Option<**bool**> | Gets or sets a value indicating whether this instance is pre padding required. | [optional]
**parent_backdrop_item_id** | Option<**String**> | Gets or sets the Id of the Parent that has a backdrop if the item does not have one. | [optional]
**parent_backdrop_image_tags** | Option<**Vec<String>**> | Gets or sets the parent backdrop image tags. | [optional]
**is_post_padding_required** | Option<**bool**> | Gets or sets a value indicating whether this instance is post padding required. | [optional]
**keep_until** | Option<[**crate::models::KeepUntil**](KeepUntil.md)> |  | [optional]
**record_any_time** | Option<**bool**> | Gets or sets a value indicating whether [record any time]. | [optional]
**skip_episodes_in_library** | Option<**bool**> |  | [optional]
**record_any_channel** | Option<**bool**> | Gets or sets a value indicating whether [record any channel]. | [optional]
**keep_up_to** | Option<**i32**> |  | [optional]
**record_new_only** | Option<**bool**> | Gets or sets a value indicating whether [record new only]. | [optional]
**days** | Option<[**Vec<crate::models::DayOfWeek>**](DayOfWeek.md)> | Gets or sets the days. | [optional]
**day_pattern** | Option<[**crate::models::DayPattern**](DayPattern.md)> | Gets or sets the day pattern. | [optional]
**image_tags** | Option<**::std::collections::HashMap<String, String>**> | Gets or sets the image tags. | [optional]
**parent_thumb_item_id** | Option<**String**> | Gets or sets the parent thumb item id. | [optional]
**parent_thumb_image_tag** | Option<**String**> | Gets or sets the parent thumb image tag. | [optional]
**parent_primary_image_item_id** | Option<**String**> | Gets or sets the parent primary image item identifier. | [optional]
**parent_primary_image_tag** | Option<**String**> | Gets or sets the parent primary image tag. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


