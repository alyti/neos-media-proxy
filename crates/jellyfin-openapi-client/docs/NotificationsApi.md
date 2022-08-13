# \NotificationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_admin_notification**](NotificationsApi.md#create_admin_notification) | **POST** /Notifications/Admin | Sends a notification to all admins.
[**get_notification_services**](NotificationsApi.md#get_notification_services) | **GET** /Notifications/Services | Gets notification services.
[**get_notification_types**](NotificationsApi.md#get_notification_types) | **GET** /Notifications/Types | Gets notification types.
[**get_notifications**](NotificationsApi.md#get_notifications) | **GET** /Notifications/{userId} | Gets a user's notifications.
[**get_notifications_summary**](NotificationsApi.md#get_notifications_summary) | **GET** /Notifications/{userId}/Summary | Gets a user's notification summary.
[**set_read**](NotificationsApi.md#set_read) | **POST** /Notifications/{userId}/Read | Sets notifications as read.
[**set_unread**](NotificationsApi.md#set_unread) | **POST** /Notifications/{userId}/Unread | Sets notifications as unread.



## create_admin_notification

> create_admin_notification(create_admin_notification_request)
Sends a notification to all admins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_admin_notification_request** | [**CreateAdminNotificationRequest**](CreateAdminNotificationRequest.md) | The notification request. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_services

> Vec<crate::models::NameIdPair> get_notification_services()
Gets notification services.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::NameIdPair>**](NameIdPair.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_types

> Vec<crate::models::NotificationTypeInfo> get_notification_types()
Gets notification types.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::NotificationTypeInfo>**](NotificationTypeInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications

> crate::models::NotificationResultDto get_notifications(user_id)
Gets a user's notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::NotificationResultDto**](NotificationResultDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications_summary

> crate::models::NotificationsSummaryDto get_notifications_summary(user_id)
Gets a user's notification summary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::NotificationsSummaryDto**](NotificationsSummaryDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_read

> set_read(user_id)
Sets notifications as read.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_unread

> set_unread(user_id)
Sets notifications as unread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

