# \PlaylistsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_to_playlist**](PlaylistsApi.md#add_to_playlist) | **POST** /Playlists/{playlistId}/Items | Adds items to a playlist.
[**create_playlist**](PlaylistsApi.md#create_playlist) | **POST** /Playlists | Creates a new playlist.
[**get_playlist_items**](PlaylistsApi.md#get_playlist_items) | **GET** /Playlists/{playlistId}/Items | Gets the original items of a playlist.
[**move_item**](PlaylistsApi.md#move_item) | **POST** /Playlists/{playlistId}/Items/{itemId}/Move/{newIndex} | Moves a playlist item.
[**remove_from_playlist**](PlaylistsApi.md#remove_from_playlist) | **DELETE** /Playlists/{playlistId}/Items | Removes items from a playlist.



## add_to_playlist

> add_to_playlist(playlist_id, ids, user_id)
Adds items to a playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **String** | The playlist id. | [required] |
**ids** | Option<[**Vec<String>**](String.md)> | Item id, comma delimited. |  |
**user_id** | Option<**String**> | The userId. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_playlist

> crate::models::PlaylistCreationResult create_playlist(name, ids, user_id, media_type, create_playlist_request)
Creates a new playlist.

For backwards compatibility parameters can be sent via Query or Body, with Query having higher precedence.  Query parameters are obsolete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The playlist name. |  |
**ids** | Option<[**Vec<String>**](String.md)> | The item ids. |  |
**user_id** | Option<**String**> | The user id. |  |
**media_type** | Option<**String**> | The media type. |  |
**create_playlist_request** | Option<[**CreatePlaylistRequest**](CreatePlaylistRequest.md)> | The create playlist payload. |  |

### Return type

[**crate::models::PlaylistCreationResult**](PlaylistCreationResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist_items

> crate::models::BaseItemDtoQueryResult get_playlist_items(playlist_id, user_id, start_index, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Gets the original items of a playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **String** | The playlist id. | [required] |
**user_id** | **String** | User id. | [required] |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_item

> move_item(playlist_id, item_id, new_index)
Moves a playlist item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **String** | The playlist id. | [required] |
**item_id** | **String** | The item id. | [required] |
**new_index** | **i32** | The new index. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_from_playlist

> remove_from_playlist(playlist_id, entry_ids)
Removes items from a playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **String** | The playlist id. | [required] |
**entry_ids** | Option<[**Vec<String>**](String.md)> | The item ids, comma delimited. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

