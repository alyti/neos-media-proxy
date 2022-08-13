# \DlnaApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_profile**](DlnaApi.md#create_profile) | **POST** /Dlna/Profiles | Creates a profile.
[**delete_profile**](DlnaApi.md#delete_profile) | **DELETE** /Dlna/Profiles/{profileId} | Deletes a profile.
[**get_default_profile**](DlnaApi.md#get_default_profile) | **GET** /Dlna/Profiles/Default | Gets the default profile.
[**get_profile**](DlnaApi.md#get_profile) | **GET** /Dlna/Profiles/{profileId} | Gets a single profile.
[**get_profile_infos**](DlnaApi.md#get_profile_infos) | **GET** /Dlna/ProfileInfos | Get profile infos.
[**update_profile**](DlnaApi.md#update_profile) | **POST** /Dlna/Profiles/{profileId} | Updates a profile.



## create_profile

> create_profile(create_profile_request)
Creates a profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_profile_request** | Option<[**CreateProfileRequest**](CreateProfileRequest.md)> | Device profile. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_profile

> delete_profile(profile_id)
Deletes a profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Profile id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_profile

> crate::models::DeviceProfile get_default_profile()
Gets the default profile.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DeviceProfile**](DeviceProfile.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile

> crate::models::DeviceProfile get_profile(profile_id)
Gets a single profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Profile Id. | [required] |

### Return type

[**crate::models::DeviceProfile**](DeviceProfile.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_infos

> Vec<crate::models::DeviceProfileInfo> get_profile_infos()
Get profile infos.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DeviceProfileInfo>**](DeviceProfileInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_profile

> update_profile(profile_id, create_profile_request)
Updates a profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Profile id. | [required] |
**create_profile_request** | Option<[**CreateProfileRequest**](CreateProfileRequest.md)> | Device profile. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

