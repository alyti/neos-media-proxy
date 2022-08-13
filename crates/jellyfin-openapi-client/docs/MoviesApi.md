# \MoviesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_movie_recommendations**](MoviesApi.md#get_movie_recommendations) | **GET** /Movies/Recommendations | Gets movie recommendations.



## get_movie_recommendations

> Vec<crate::models::RecommendationDto> get_movie_recommendations(user_id, parent_id, fields, category_limit, item_limit)
Gets movie recommendations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
**parent_id** | Option<**String**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. The fields to return. |  |
**category_limit** | Option<**i32**> | The max number of categories to return. |  |[default to 5]
**item_limit** | Option<**i32**> | The max number of items to return per category. |  |[default to 8]

### Return type

[**Vec<crate::models::RecommendationDto>**](RecommendationDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

