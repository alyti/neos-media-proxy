# \ActivityLogApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_log_entries**](ActivityLogApi.md#get_log_entries) | **GET** /System/ActivityLog/Entries | Gets activity log entries.



## get_log_entries

> crate::models::ActivityLogEntryQueryResult get_log_entries(start_index, limit, min_date, has_user_id)
Gets activity log entries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**min_date** | Option<**String**> | Optional. The minimum date. Format = ISO. |  |
**has_user_id** | Option<**bool**> | Optional. Filter log entries if it has user id, or not. |  |

### Return type

[**crate::models::ActivityLogEntryQueryResult**](ActivityLogEntryQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

