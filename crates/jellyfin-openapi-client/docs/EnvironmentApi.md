# \EnvironmentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_default_directory_browser**](EnvironmentApi.md#get_default_directory_browser) | **GET** /Environment/DefaultDirectoryBrowser | Get Default directory browser.
[**get_directory_contents**](EnvironmentApi.md#get_directory_contents) | **GET** /Environment/DirectoryContents | Gets the contents of a given directory in the file system.
[**get_drives**](EnvironmentApi.md#get_drives) | **GET** /Environment/Drives | Gets available drives from the server's file system.
[**get_network_shares**](EnvironmentApi.md#get_network_shares) | **GET** /Environment/NetworkShares | Gets network paths.
[**get_parent_path**](EnvironmentApi.md#get_parent_path) | **GET** /Environment/ParentPath | Gets the parent path of a given path.
[**validate_path**](EnvironmentApi.md#validate_path) | **POST** /Environment/ValidatePath | Validates path.



## get_default_directory_browser

> crate::models::DefaultDirectoryBrowserInfoDto get_default_directory_browser()
Get Default directory browser.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DefaultDirectoryBrowserInfoDto**](DefaultDirectoryBrowserInfoDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_directory_contents

> Vec<crate::models::FileSystemEntryInfo> get_directory_contents(path, include_files, include_directories)
Gets the contents of a given directory in the file system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | The path. | [required] |
**include_files** | Option<**bool**> | An optional filter to include or exclude files from the results. true/false. |  |[default to false]
**include_directories** | Option<**bool**> | An optional filter to include or exclude folders from the results. true/false. |  |[default to false]

### Return type

[**Vec<crate::models::FileSystemEntryInfo>**](FileSystemEntryInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_drives

> Vec<crate::models::FileSystemEntryInfo> get_drives()
Gets available drives from the server's file system.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::FileSystemEntryInfo>**](FileSystemEntryInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_network_shares

> Vec<crate::models::FileSystemEntryInfo> get_network_shares()
Gets network paths.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::FileSystemEntryInfo>**](FileSystemEntryInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_parent_path

> String get_parent_path(path)
Gets the parent path of a given path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | The path. | [required] |

### Return type

**String**

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_path

> validate_path(validate_path_request)
Validates path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_path_request** | [**ValidatePathRequest**](ValidatePathRequest.md) | Validate request object. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

