# \PluginsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disable_plugin**](PluginsApi.md#disable_plugin) | **POST** /Plugins/{pluginId}/{version}/Disable | Disable a plugin.
[**enable_plugin**](PluginsApi.md#enable_plugin) | **POST** /Plugins/{pluginId}/{version}/Enable | Enables a disabled plugin.
[**get_plugin_configuration**](PluginsApi.md#get_plugin_configuration) | **GET** /Plugins/{pluginId}/Configuration | Gets plugin configuration.
[**get_plugin_image**](PluginsApi.md#get_plugin_image) | **GET** /Plugins/{pluginId}/{version}/Image | Gets a plugin's image.
[**get_plugin_manifest**](PluginsApi.md#get_plugin_manifest) | **POST** /Plugins/{pluginId}/Manifest | Gets a plugin's manifest.
[**get_plugins**](PluginsApi.md#get_plugins) | **GET** /Plugins | Gets a list of currently installed plugins.
[**uninstall_plugin**](PluginsApi.md#uninstall_plugin) | **DELETE** /Plugins/{pluginId} | Uninstalls a plugin.
[**uninstall_plugin_by_version**](PluginsApi.md#uninstall_plugin_by_version) | **DELETE** /Plugins/{pluginId}/{version} | Uninstalls a plugin by version.
[**update_plugin_configuration**](PluginsApi.md#update_plugin_configuration) | **POST** /Plugins/{pluginId}/Configuration | Updates plugin configuration.



## disable_plugin

> disable_plugin(plugin_id, version)
Disable a plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | Plugin id. | [required] |
**version** | **String** | Plugin version. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_plugin

> enable_plugin(plugin_id, version)
Enables a disabled plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | Plugin id. | [required] |
**version** | **String** | Plugin version. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_configuration

> serde_json::Value get_plugin_configuration(plugin_id)
Gets plugin configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | Plugin id. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_image

> std::path::PathBuf get_plugin_image(plugin_id, version)
Gets a plugin's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | Plugin id. | [required] |
**version** | **String** | Plugin version. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_manifest

> get_plugin_manifest(plugin_id)
Gets a plugin's manifest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | Plugin id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugins

> Vec<crate::models::PluginInfo> get_plugins()
Gets a list of currently installed plugins.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PluginInfo>**](PluginInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_plugin

> uninstall_plugin(plugin_id)
Uninstalls a plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | Plugin id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_plugin_by_version

> uninstall_plugin_by_version(plugin_id, version)
Uninstalls a plugin by version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | Plugin id. | [required] |
**version** | **String** | Plugin version. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_plugin_configuration

> update_plugin_configuration(plugin_id)
Updates plugin configuration.

Accepts plugin configuration as JSON body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **String** | Plugin id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

