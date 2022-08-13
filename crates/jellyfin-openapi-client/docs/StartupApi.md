# \StartupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_wizard**](StartupApi.md#complete_wizard) | **POST** /Startup/Complete | Completes the startup wizard.
[**get_first_user**](StartupApi.md#get_first_user) | **GET** /Startup/User | Gets the first user.
[**get_first_user2**](StartupApi.md#get_first_user2) | **GET** /Startup/FirstUser | Gets the first user.
[**get_startup_configuration**](StartupApi.md#get_startup_configuration) | **GET** /Startup/Configuration | Gets the initial startup wizard configuration.
[**set_remote_access**](StartupApi.md#set_remote_access) | **POST** /Startup/RemoteAccess | Sets remote access and UPnP.
[**update_initial_configuration**](StartupApi.md#update_initial_configuration) | **POST** /Startup/Configuration | Sets the initial startup wizard configuration.
[**update_startup_user**](StartupApi.md#update_startup_user) | **POST** /Startup/User | Sets the user name and password.



## complete_wizard

> complete_wizard()
Completes the startup wizard.

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


## get_first_user

> crate::models::StartupUserDto get_first_user()
Gets the first user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StartupUserDto**](StartupUserDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_first_user2

> crate::models::StartupUserDto get_first_user2()
Gets the first user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StartupUserDto**](StartupUserDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_startup_configuration

> crate::models::StartupConfigurationDto get_startup_configuration()
Gets the initial startup wizard configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StartupConfigurationDto**](StartupConfigurationDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_remote_access

> set_remote_access(set_remote_access_request)
Sets remote access and UPnP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_remote_access_request** | [**SetRemoteAccessRequest**](SetRemoteAccessRequest.md) | The startup remote access dto. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_initial_configuration

> update_initial_configuration(update_initial_configuration_request)
Sets the initial startup wizard configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_initial_configuration_request** | [**UpdateInitialConfigurationRequest**](UpdateInitialConfigurationRequest.md) | The updated startup configuration. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_startup_user

> update_startup_user(update_startup_user_request)
Sets the user name and password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_startup_user_request** | Option<[**UpdateStartupUserRequest**](UpdateStartupUserRequest.md)> | The DTO containing username and password. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

