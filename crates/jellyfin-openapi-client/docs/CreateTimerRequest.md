# CreateTimerRequest

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
**status** | Option<[**crate::models::RecordingStatus**](RecordingStatus.md)> | Gets or sets the status. | [optional]
**series_timer_id** | Option<**String**> | Gets or sets the series timer identifier. | [optional]
**external_series_timer_id** | Option<**String**> | Gets or sets the external series timer identifier. | [optional]
**run_time_ticks** | Option<**i64**> | Gets or sets the run time ticks. | [optional]
**program_info** | Option<[**crate::models::TimerInfoDtoProgramInfo**](TimerInfoDto_ProgramInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


