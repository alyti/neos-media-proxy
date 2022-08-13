# \ScheduledTasksApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_task**](ScheduledTasksApi.md#get_task) | **GET** /ScheduledTasks/{taskId} | Get task by id.
[**get_tasks**](ScheduledTasksApi.md#get_tasks) | **GET** /ScheduledTasks | Get tasks.
[**start_task**](ScheduledTasksApi.md#start_task) | **POST** /ScheduledTasks/Running/{taskId} | Start specified task.
[**stop_task**](ScheduledTasksApi.md#stop_task) | **DELETE** /ScheduledTasks/Running/{taskId} | Stop specified task.
[**update_task**](ScheduledTasksApi.md#update_task) | **POST** /ScheduledTasks/{taskId}/Triggers | Update specified task triggers.



## get_task

> crate::models::TaskInfo get_task(task_id)
Get task by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Task Id. | [required] |

### Return type

[**crate::models::TaskInfo**](TaskInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tasks

> Vec<crate::models::TaskInfo> get_tasks(is_hidden, is_enabled)
Get tasks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_hidden** | Option<**bool**> | Optional filter tasks that are hidden, or not. |  |
**is_enabled** | Option<**bool**> | Optional filter tasks that are enabled, or not. |  |

### Return type

[**Vec<crate::models::TaskInfo>**](TaskInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_task

> start_task(task_id)
Start specified task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Task Id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_task

> stop_task(task_id)
Stop specified task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Task Id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task

> update_task(task_id, task_trigger_info)
Update specified task triggers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Task Id. | [required] |
**task_trigger_info** | [**Vec<crate::models::TaskTriggerInfo>**](TaskTriggerInfo.md) | Triggers. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

