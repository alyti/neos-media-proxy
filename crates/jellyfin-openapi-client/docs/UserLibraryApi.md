# \UserLibraryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user_item_rating**](UserLibraryApi.md#delete_user_item_rating) | **DELETE** /Users/{userId}/Items/{itemId}/Rating | Deletes a user's saved personal rating for an item.
[**get_intros**](UserLibraryApi.md#get_intros) | **GET** /Users/{userId}/Items/{itemId}/Intros | Gets intros to play before the main media item plays.
[**get_item**](UserLibraryApi.md#get_item) | **GET** /Users/{userId}/Items/{itemId} | Gets an item from a user's library.
[**get_latest_media**](UserLibraryApi.md#get_latest_media) | **GET** /Users/{userId}/Items/Latest | Gets latest media.
[**get_local_trailers**](UserLibraryApi.md#get_local_trailers) | **GET** /Users/{userId}/Items/{itemId}/LocalTrailers | Gets local trailers for an item.
[**get_root_folder**](UserLibraryApi.md#get_root_folder) | **GET** /Users/{userId}/Items/Root | Gets the root folder from a user's library.
[**get_special_features**](UserLibraryApi.md#get_special_features) | **GET** /Users/{userId}/Items/{itemId}/SpecialFeatures | Gets special features for an item.
[**mark_favorite_item**](UserLibraryApi.md#mark_favorite_item) | **POST** /Users/{userId}/FavoriteItems/{itemId} | Marks an item as a favorite.
[**unmark_favorite_item**](UserLibraryApi.md#unmark_favorite_item) | **DELETE** /Users/{userId}/FavoriteItems/{itemId} | Unmarks item as a favorite.
[**update_user_item_rating**](UserLibraryApi.md#update_user_item_rating) | **POST** /Users/{userId}/Items/{itemId}/Rating | Updates a user's rating for an item.



## delete_user_item_rating

> crate::models::UserItemDataDto delete_user_item_rating(user_id, item_id)
Deletes a user's saved personal rating for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |

### Return type

[**crate::models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_intros

> crate::models::BaseItemDtoQueryResult get_intros(user_id, item_id)
Gets intros to play before the main media item plays.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item

> crate::models::BaseItemDto get_item(user_id, item_id)
Gets an item from a user's library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |

### Return type

[**crate::models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_media

> Vec<crate::models::BaseItemDto> get_latest_media(user_id, parent_id, fields, include_item_types, is_played, enable_images, image_type_limit, enable_image_types, enable_user_data, limit, group_items)
Gets latest media.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**parent_id** | Option<**String**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**include_item_types** | Option<[**Vec<crate::models::BaseItemKind>**](crate::models::BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**is_played** | Option<**bool**> | Filter by items that are played, or not. |  |
**enable_images** | Option<**bool**> | Optional. include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional. the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. include user data. |  |
**limit** | Option<**i32**> | Return item limit. |  |[default to 20]
**group_items** | Option<**bool**> | Whether or not to group items into a parent container. |  |[default to true]

### Return type

[**Vec<crate::models::BaseItemDto>**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_local_trailers

> Vec<crate::models::BaseItemDto> get_local_trailers(user_id, item_id)
Gets local trailers for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |

### Return type

[**Vec<crate::models::BaseItemDto>**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_root_folder

> crate::models::BaseItemDto get_root_folder(user_id)
Gets the root folder from a user's library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |

### Return type

[**crate::models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_special_features

> Vec<crate::models::BaseItemDto> get_special_features(user_id, item_id)
Gets special features for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |

### Return type

[**Vec<crate::models::BaseItemDto>**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_favorite_item

> crate::models::UserItemDataDto mark_favorite_item(user_id, item_id)
Marks an item as a favorite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |

### Return type

[**crate::models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unmark_favorite_item

> crate::models::UserItemDataDto unmark_favorite_item(user_id, item_id)
Unmarks item as a favorite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |

### Return type

[**crate::models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_item_rating

> crate::models::UserItemDataDto update_user_item_rating(user_id, item_id, likes)
Updates a user's rating for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**item_id** | **String** | Item id. | [required] |
**likes** | Option<**bool**> | Whether this M:Jellyfin.Api.Controllers.UserLibraryController.UpdateUserItemRating(System.Guid,System.Guid,System.Nullable{System.Boolean}) is likes. |  |

### Return type

[**crate::models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

