# \InstantMixApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_instant_mix_from_album**](InstantMixApi.md#get_instant_mix_from_album) | **GET** /Albums/{id}/InstantMix | Creates an instant playlist based on a given album.
[**get_instant_mix_from_artists**](InstantMixApi.md#get_instant_mix_from_artists) | **GET** /Artists/{id}/InstantMix | Creates an instant playlist based on a given artist.
[**get_instant_mix_from_artists2**](InstantMixApi.md#get_instant_mix_from_artists2) | **GET** /Artists/InstantMix | Creates an instant playlist based on a given artist.
[**get_instant_mix_from_item**](InstantMixApi.md#get_instant_mix_from_item) | **GET** /Items/{id}/InstantMix | Creates an instant playlist based on a given item.
[**get_instant_mix_from_music_genre_by_id**](InstantMixApi.md#get_instant_mix_from_music_genre_by_id) | **GET** /MusicGenres/InstantMix | Creates an instant playlist based on a given genre.
[**get_instant_mix_from_music_genre_by_name**](InstantMixApi.md#get_instant_mix_from_music_genre_by_name) | **GET** /MusicGenres/{name}/InstantMix | Creates an instant playlist based on a given genre.
[**get_instant_mix_from_playlist**](InstantMixApi.md#get_instant_mix_from_playlist) | **GET** /Playlists/{id}/InstantMix | Creates an instant playlist based on a given playlist.
[**get_instant_mix_from_song**](InstantMixApi.md#get_instant_mix_from_song) | **GET** /Songs/{id}/InstantMix | Creates an instant playlist based on a given song.



## get_instant_mix_from_album

> crate::models::BaseItemDtoQueryResult get_instant_mix_from_album(id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
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


## get_instant_mix_from_artists

> crate::models::BaseItemDtoQueryResult get_instant_mix_from_artists(id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given artist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
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


## get_instant_mix_from_artists2

> crate::models::BaseItemDtoQueryResult get_instant_mix_from_artists2(id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given artist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
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


## get_instant_mix_from_item

> crate::models::BaseItemDtoQueryResult get_instant_mix_from_item(id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
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


## get_instant_mix_from_music_genre_by_id

> crate::models::BaseItemDtoQueryResult get_instant_mix_from_music_genre_by_id(id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given genre.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
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


## get_instant_mix_from_music_genre_by_name

> crate::models::BaseItemDtoQueryResult get_instant_mix_from_music_genre_by_name(name, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given genre.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The genre name. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
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


## get_instant_mix_from_playlist

> crate::models::BaseItemDtoQueryResult get_instant_mix_from_playlist(id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
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


## get_instant_mix_from_song

> crate::models::BaseItemDtoQueryResult get_instant_mix_from_song(id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given song.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The item id. | [required] |
**user_id** | Option<**String**> | Optional. Filter by user id, and attach user data. |  |
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

