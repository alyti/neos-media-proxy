# \LibraryStructureApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_media_path**](LibraryStructureApi.md#add_media_path) | **POST** /Library/VirtualFolders/Paths | Add a media path to a library.
[**add_virtual_folder**](LibraryStructureApi.md#add_virtual_folder) | **POST** /Library/VirtualFolders | Adds a virtual folder.
[**get_virtual_folders**](LibraryStructureApi.md#get_virtual_folders) | **GET** /Library/VirtualFolders | Gets all virtual folders.
[**remove_media_path**](LibraryStructureApi.md#remove_media_path) | **DELETE** /Library/VirtualFolders/Paths | Remove a media path.
[**remove_virtual_folder**](LibraryStructureApi.md#remove_virtual_folder) | **DELETE** /Library/VirtualFolders | Removes a virtual folder.
[**rename_virtual_folder**](LibraryStructureApi.md#rename_virtual_folder) | **POST** /Library/VirtualFolders/Name | Renames a virtual folder.
[**update_library_options**](LibraryStructureApi.md#update_library_options) | **POST** /Library/VirtualFolders/LibraryOptions | Update library options.
[**update_media_path**](LibraryStructureApi.md#update_media_path) | **POST** /Library/VirtualFolders/Paths/Update | Updates a media path.



## add_media_path

> add_media_path(add_media_path_request, refresh_library)
Add a media path to a library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_media_path_request** | [**AddMediaPathRequest**](AddMediaPathRequest.md) | The media path dto. | [required] |
**refresh_library** | Option<**bool**> | Whether to refresh the library. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_virtual_folder

> add_virtual_folder(name, collection_type, paths, refresh_library, add_virtual_folder_request)
Adds a virtual folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the virtual folder. |  |
**collection_type** | Option<[**crate::models::CollectionTypeOptions**](.md)> | The type of the collection. |  |
**paths** | Option<[**Vec<String>**](String.md)> | The paths of the virtual folder. |  |
**refresh_library** | Option<**bool**> | Whether to refresh the library. |  |[default to false]
**add_virtual_folder_request** | Option<[**AddVirtualFolderRequest**](AddVirtualFolderRequest.md)> | The library options. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_virtual_folders

> Vec<crate::models::VirtualFolderInfo> get_virtual_folders()
Gets all virtual folders.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::VirtualFolderInfo>**](VirtualFolderInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_media_path

> remove_media_path(name, path, refresh_library)
Remove a media path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the library. |  |
**path** | Option<**String**> | The path to remove. |  |
**refresh_library** | Option<**bool**> | Whether to refresh the library. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_virtual_folder

> remove_virtual_folder(name, refresh_library)
Removes a virtual folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the folder. |  |
**refresh_library** | Option<**bool**> | Whether to refresh the library. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rename_virtual_folder

> rename_virtual_folder(name, new_name, refresh_library)
Renames a virtual folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the virtual folder. |  |
**new_name** | Option<**String**> | The new name. |  |
**refresh_library** | Option<**bool**> | Whether to refresh the library. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_library_options

> update_library_options(update_library_options_request)
Update library options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_library_options_request** | Option<[**UpdateLibraryOptionsRequest**](UpdateLibraryOptionsRequest.md)> | The library name and options. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_media_path

> update_media_path(update_media_path_request)
Updates a media path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_media_path_request** | [**UpdateMediaPathRequest**](UpdateMediaPathRequest.md) | The name of the library and path infos. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

