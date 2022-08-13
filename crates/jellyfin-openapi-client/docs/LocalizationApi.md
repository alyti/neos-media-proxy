# \LocalizationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_countries**](LocalizationApi.md#get_countries) | **GET** /Localization/Countries | Gets known countries.
[**get_cultures**](LocalizationApi.md#get_cultures) | **GET** /Localization/Cultures | Gets known cultures.
[**get_localization_options**](LocalizationApi.md#get_localization_options) | **GET** /Localization/Options | Gets localization options.
[**get_parental_ratings**](LocalizationApi.md#get_parental_ratings) | **GET** /Localization/ParentalRatings | Gets known parental ratings.



## get_countries

> Vec<crate::models::CountryInfo> get_countries()
Gets known countries.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CountryInfo>**](CountryInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cultures

> Vec<crate::models::CultureDto> get_cultures()
Gets known cultures.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CultureDto>**](CultureDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_localization_options

> Vec<crate::models::LocalizationOption> get_localization_options()
Gets localization options.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LocalizationOption>**](LocalizationOption.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_parental_ratings

> Vec<crate::models::ParentalRating> get_parental_ratings()
Gets known parental ratings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ParentalRating>**](ParentalRating.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

