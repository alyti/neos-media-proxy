# NotificationOption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<**String**> |  | [optional]
**disabled_monitor_users** | Option<**Vec<String>**> | Gets or sets user Ids to not monitor (it's opt out). | [optional]
**send_to_users** | Option<**Vec<String>**> | Gets or sets user Ids to send to (if SendToUserMode == Custom). | [optional]
**enabled** | Option<**bool**> | Gets or sets a value indicating whether this MediaBrowser.Model.Notifications.NotificationOption is enabled. | [optional]
**disabled_services** | Option<**Vec<String>**> | Gets or sets the disabled services. | [optional]
**send_to_user_mode** | Option<[**crate::models::SendToUserType**](SendToUserType.md)> | Gets or sets the send to user mode. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


