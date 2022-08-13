# \ConfigurationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_configuration**](ConfigurationApi.md#get_configuration) | **GET** /System/Configuration | Gets application configuration.
[**get_default_metadata_options**](ConfigurationApi.md#get_default_metadata_options) | **GET** /System/Configuration/MetadataOptions/Default | Gets a default MetadataOptions object.
[**get_named_configuration**](ConfigurationApi.md#get_named_configuration) | **GET** /System/Configuration/{key} | Gets a named configuration.
[**update_configuration**](ConfigurationApi.md#update_configuration) | **POST** /System/Configuration | Updates application configuration.
[**update_media_encoder_path**](ConfigurationApi.md#update_media_encoder_path) | **POST** /System/MediaEncoder/Path | Updates the path to the media encoder.
[**update_named_configuration**](ConfigurationApi.md#update_named_configuration) | **POST** /System/Configuration/{key} | Updates named configuration.



## get_configuration

> crate::models::ServerConfiguration get_configuration()
Gets application configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ServerConfiguration**](ServerConfiguration.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_metadata_options

> crate::models::MetadataOptions get_default_metadata_options()
Gets a default MetadataOptions object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MetadataOptions**](MetadataOptions.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_named_configuration

> std::path::PathBuf get_named_configuration(key)
Gets a named configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | Configuration key. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_configuration

> update_configuration(update_configuration_request)
Updates application configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_configuration_request** | [**UpdateConfigurationRequest**](UpdateConfigurationRequest.md) | Configuration. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_media_encoder_path

> update_media_encoder_path(update_media_encoder_path_request)
Updates the path to the media encoder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_media_encoder_path_request** | [**UpdateMediaEncoderPathRequest**](UpdateMediaEncoderPathRequest.md) | Media encoder path form body. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_named_configuration

> update_named_configuration(key, body)
Updates named configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | Configuration key. | [required] |
**body** | Option<**serde_json::Value**> | Configuration. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

