# \ImageByNameApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_general_image**](ImageByNameApi.md#get_general_image) | **GET** /Images/General/{name}/{type} | Get General Image.
[**get_general_images**](ImageByNameApi.md#get_general_images) | **GET** /Images/General | Get all general images.
[**get_media_info_image**](ImageByNameApi.md#get_media_info_image) | **GET** /Images/MediaInfo/{theme}/{name} | Get media info image.
[**get_media_info_images**](ImageByNameApi.md#get_media_info_images) | **GET** /Images/MediaInfo | Get all media info images.
[**get_rating_image**](ImageByNameApi.md#get_rating_image) | **GET** /Images/Ratings/{theme}/{name} | Get rating image.
[**get_rating_images**](ImageByNameApi.md#get_rating_images) | **GET** /Images/Ratings | Get all general images.



## get_general_image

> std::path::PathBuf get_general_image(name, _type)
Get General Image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the image. | [required] |
**_type** | **String** | Image Type (primary, backdrop, logo, etc). | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/octet-stream, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_general_images

> Vec<crate::models::ImageByNameInfo> get_general_images()
Get all general images.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ImageByNameInfo>**](ImageByNameInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_info_image

> std::path::PathBuf get_media_info_image(theme, name)
Get media info image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme** | **String** | The theme to get the image from. | [required] |
**name** | **String** | The name of the image. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/octet-stream, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_info_images

> Vec<crate::models::ImageByNameInfo> get_media_info_images()
Get all media info images.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ImageByNameInfo>**](ImageByNameInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rating_image

> std::path::PathBuf get_rating_image(theme, name)
Get rating image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme** | **String** | The theme to get the image from. | [required] |
**name** | **String** | The name of the image. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/octet-stream, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rating_images

> Vec<crate::models::ImageByNameInfo> get_rating_images()
Get all general images.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ImageByNameInfo>**](ImageByNameInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

