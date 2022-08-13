# \PackageApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_package_installation**](PackageApi.md#cancel_package_installation) | **DELETE** /Packages/Installing/{packageId} | Cancels a package installation.
[**get_package_info**](PackageApi.md#get_package_info) | **GET** /Packages/{name} | Gets a package by name or assembly GUID.
[**get_packages**](PackageApi.md#get_packages) | **GET** /Packages | Gets available packages.
[**get_repositories**](PackageApi.md#get_repositories) | **GET** /Repositories | Gets all package repositories.
[**install_package**](PackageApi.md#install_package) | **POST** /Packages/Installed/{name} | Installs a package.
[**set_repositories**](PackageApi.md#set_repositories) | **POST** /Repositories | Sets the enabled and existing package repositories.



## cancel_package_installation

> cancel_package_installation(package_id)
Cancels a package installation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_id** | **String** | Installation Id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_package_info

> crate::models::PackageInfo get_package_info(name, assembly_guid)
Gets a package by name or assembly GUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the package. | [required] |
**assembly_guid** | Option<**String**> | The GUID of the associated assembly. |  |

### Return type

[**crate::models::PackageInfo**](PackageInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_packages

> Vec<crate::models::PackageInfo> get_packages()
Gets available packages.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PackageInfo>**](PackageInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repositories

> Vec<crate::models::RepositoryInfo> get_repositories()
Gets all package repositories.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RepositoryInfo>**](RepositoryInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## install_package

> install_package(name, assembly_guid, version, repository_url)
Installs a package.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Package name. | [required] |
**assembly_guid** | Option<**String**> | GUID of the associated assembly. |  |
**version** | Option<**String**> | Optional version. Defaults to latest version. |  |
**repository_url** | Option<**String**> | Optional. Specify the repository to install from. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_repositories

> set_repositories(repository_info)
Sets the enabled and existing package repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository_info** | [**Vec<crate::models::RepositoryInfo>**](RepositoryInfo.md) | The list of package repositories. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

