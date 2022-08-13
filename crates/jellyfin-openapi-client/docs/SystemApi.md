# \SystemApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_endpoint_info**](SystemApi.md#get_endpoint_info) | **GET** /System/Endpoint | Gets information about the request endpoint.
[**get_log_file**](SystemApi.md#get_log_file) | **GET** /System/Logs/Log | Gets a log file.
[**get_ping_system**](SystemApi.md#get_ping_system) | **GET** /System/Ping | Pings the system.
[**get_public_system_info**](SystemApi.md#get_public_system_info) | **GET** /System/Info/Public | Gets public information about the server.
[**get_server_logs**](SystemApi.md#get_server_logs) | **GET** /System/Logs | Gets a list of available server log files.
[**get_system_info**](SystemApi.md#get_system_info) | **GET** /System/Info | Gets information about the server.
[**get_wake_on_lan_info**](SystemApi.md#get_wake_on_lan_info) | **GET** /System/WakeOnLanInfo | Gets wake on lan information.
[**post_ping_system**](SystemApi.md#post_ping_system) | **POST** /System/Ping | Pings the system.
[**restart_application**](SystemApi.md#restart_application) | **POST** /System/Restart | Restarts the application.
[**shutdown_application**](SystemApi.md#shutdown_application) | **POST** /System/Shutdown | Shuts down the application.



## get_endpoint_info

> crate::models::EndPointInfo get_endpoint_info()
Gets information about the request endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EndPointInfo**](EndPointInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_log_file

> std::path::PathBuf get_log_file(name)
Gets a log file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the log file to get. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ping_system

> String get_ping_system()
Pings the system.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_system_info

> crate::models::PublicSystemInfo get_public_system_info()
Gets public information about the server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PublicSystemInfo**](PublicSystemInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_logs

> Vec<crate::models::LogFile> get_server_logs()
Gets a list of available server log files.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LogFile>**](LogFile.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_info

> crate::models::SystemInfo get_system_info()
Gets information about the server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SystemInfo**](SystemInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wake_on_lan_info

> Vec<crate::models::WakeOnLanInfo> get_wake_on_lan_info()
Gets wake on lan information.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::WakeOnLanInfo>**](WakeOnLanInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ping_system

> String post_ping_system()
Pings the system.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_application

> restart_application()
Restarts the application.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shutdown_application

> shutdown_application()
Shuts down the application.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

