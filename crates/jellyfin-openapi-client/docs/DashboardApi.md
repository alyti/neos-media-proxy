# \DashboardApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_configuration_pages**](DashboardApi.md#get_configuration_pages) | **GET** /web/ConfigurationPages | Gets the configuration pages.
[**get_dashboard_configuration_page**](DashboardApi.md#get_dashboard_configuration_page) | **GET** /web/ConfigurationPage | Gets a dashboard configuration page.



## get_configuration_pages

> Vec<crate::models::ConfigurationPageInfo> get_configuration_pages(enable_in_main_menu)
Gets the configuration pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enable_in_main_menu** | Option<**bool**> | Whether to enable in the main menu. |  |

### Return type

[**Vec<crate::models::ConfigurationPageInfo>**](ConfigurationPageInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_configuration_page

> std::path::PathBuf get_dashboard_configuration_page(name)
Gets a dashboard configuration page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the page. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html, application/x-javascript, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

