# \RemoteImageApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_remote_image**](RemoteImageApi.md#download_remote_image) | **POST** /Items/{itemId}/RemoteImages/Download | Downloads a remote image for an item.
[**get_remote_image_providers**](RemoteImageApi.md#get_remote_image_providers) | **GET** /Items/{itemId}/RemoteImages/Providers | Gets available remote image providers for an item.
[**get_remote_images**](RemoteImageApi.md#get_remote_images) | **GET** /Items/{itemId}/RemoteImages | Gets available remote images for an item.



## download_remote_image

> download_remote_image(item_id, _type, image_url)
Downloads a remote image for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item Id. | [required] |
**_type** | [**crate::models::ImageType**](.md) | The image type. | [required] |
**image_url** | Option<**String**> | The image url. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_remote_image_providers

> Vec<crate::models::ImageProviderInfo> get_remote_image_providers(item_id)
Gets available remote image providers for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item Id. | [required] |

### Return type

[**Vec<crate::models::ImageProviderInfo>**](ImageProviderInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_remote_images

> crate::models::RemoteImageResult get_remote_images(item_id, _type, start_index, limit, provider_name, include_all_languages)
Gets available remote images for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item Id. | [required] |
**_type** | Option<[**crate::models::ImageType**](.md)> | The image type. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**provider_name** | Option<**String**> | Optional. The image provider to use. |  |
**include_all_languages** | Option<**bool**> | Optional. Include all languages. |  |[default to false]

### Return type

[**crate::models::RemoteImageResult**](RemoteImageResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

