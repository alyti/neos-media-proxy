# \ImageApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_custom_splashscreen**](ImageApi.md#delete_custom_splashscreen) | **DELETE** /Branding/Splashscreen | Delete a custom splashscreen.
[**delete_item_image**](ImageApi.md#delete_item_image) | **DELETE** /Items/{itemId}/Images/{imageType} | Delete an item's image.
[**delete_item_image_by_index**](ImageApi.md#delete_item_image_by_index) | **DELETE** /Items/{itemId}/Images/{imageType}/{imageIndex} | Delete an item's image.
[**delete_user_image**](ImageApi.md#delete_user_image) | **DELETE** /Users/{userId}/Images/{imageType} | Delete the user's image.
[**delete_user_image_by_index**](ImageApi.md#delete_user_image_by_index) | **DELETE** /Users/{userId}/Images/{imageType}/{index} | Delete the user's image.
[**get_artist_image**](ImageApi.md#get_artist_image) | **GET** /Artists/{name}/Images/{imageType}/{imageIndex} | Get artist image by name.
[**get_genre_image**](ImageApi.md#get_genre_image) | **GET** /Genres/{name}/Images/{imageType} | Get genre image by name.
[**get_genre_image_by_index**](ImageApi.md#get_genre_image_by_index) | **GET** /Genres/{name}/Images/{imageType}/{imageIndex} | Get genre image by name.
[**get_item_image**](ImageApi.md#get_item_image) | **GET** /Items/{itemId}/Images/{imageType} | Gets the item's image.
[**get_item_image2**](ImageApi.md#get_item_image2) | **GET** /Items/{itemId}/Images/{imageType}/{imageIndex}/{tag}/{format}/{maxWidth}/{maxHeight}/{percentPlayed}/{unplayedCount} | Gets the item's image.
[**get_item_image_by_index**](ImageApi.md#get_item_image_by_index) | **GET** /Items/{itemId}/Images/{imageType}/{imageIndex} | Gets the item's image.
[**get_item_image_infos**](ImageApi.md#get_item_image_infos) | **GET** /Items/{itemId}/Images | Get item image infos.
[**get_music_genre_image**](ImageApi.md#get_music_genre_image) | **GET** /MusicGenres/{name}/Images/{imageType} | Get music genre image by name.
[**get_music_genre_image_by_index**](ImageApi.md#get_music_genre_image_by_index) | **GET** /MusicGenres/{name}/Images/{imageType}/{imageIndex} | Get music genre image by name.
[**get_person_image**](ImageApi.md#get_person_image) | **GET** /Persons/{name}/Images/{imageType} | Get person image by name.
[**get_person_image_by_index**](ImageApi.md#get_person_image_by_index) | **GET** /Persons/{name}/Images/{imageType}/{imageIndex} | Get person image by name.
[**get_splashscreen**](ImageApi.md#get_splashscreen) | **GET** /Branding/Splashscreen | Generates or gets the splashscreen.
[**get_studio_image**](ImageApi.md#get_studio_image) | **GET** /Studios/{name}/Images/{imageType} | Get studio image by name.
[**get_studio_image_by_index**](ImageApi.md#get_studio_image_by_index) | **GET** /Studios/{name}/Images/{imageType}/{imageIndex} | Get studio image by name.
[**get_user_image**](ImageApi.md#get_user_image) | **GET** /Users/{userId}/Images/{imageType} | Get user profile image.
[**get_user_image_by_index**](ImageApi.md#get_user_image_by_index) | **GET** /Users/{userId}/Images/{imageType}/{imageIndex} | Get user profile image.
[**head_artist_image**](ImageApi.md#head_artist_image) | **HEAD** /Artists/{name}/Images/{imageType}/{imageIndex} | Get artist image by name.
[**head_genre_image**](ImageApi.md#head_genre_image) | **HEAD** /Genres/{name}/Images/{imageType} | Get genre image by name.
[**head_genre_image_by_index**](ImageApi.md#head_genre_image_by_index) | **HEAD** /Genres/{name}/Images/{imageType}/{imageIndex} | Get genre image by name.
[**head_item_image**](ImageApi.md#head_item_image) | **HEAD** /Items/{itemId}/Images/{imageType} | Gets the item's image.
[**head_item_image2**](ImageApi.md#head_item_image2) | **HEAD** /Items/{itemId}/Images/{imageType}/{imageIndex}/{tag}/{format}/{maxWidth}/{maxHeight}/{percentPlayed}/{unplayedCount} | Gets the item's image.
[**head_item_image_by_index**](ImageApi.md#head_item_image_by_index) | **HEAD** /Items/{itemId}/Images/{imageType}/{imageIndex} | Gets the item's image.
[**head_music_genre_image**](ImageApi.md#head_music_genre_image) | **HEAD** /MusicGenres/{name}/Images/{imageType} | Get music genre image by name.
[**head_music_genre_image_by_index**](ImageApi.md#head_music_genre_image_by_index) | **HEAD** /MusicGenres/{name}/Images/{imageType}/{imageIndex} | Get music genre image by name.
[**head_person_image**](ImageApi.md#head_person_image) | **HEAD** /Persons/{name}/Images/{imageType} | Get person image by name.
[**head_person_image_by_index**](ImageApi.md#head_person_image_by_index) | **HEAD** /Persons/{name}/Images/{imageType}/{imageIndex} | Get person image by name.
[**head_studio_image**](ImageApi.md#head_studio_image) | **HEAD** /Studios/{name}/Images/{imageType} | Get studio image by name.
[**head_studio_image_by_index**](ImageApi.md#head_studio_image_by_index) | **HEAD** /Studios/{name}/Images/{imageType}/{imageIndex} | Get studio image by name.
[**head_user_image**](ImageApi.md#head_user_image) | **HEAD** /Users/{userId}/Images/{imageType} | Get user profile image.
[**head_user_image_by_index**](ImageApi.md#head_user_image_by_index) | **HEAD** /Users/{userId}/Images/{imageType}/{imageIndex} | Get user profile image.
[**post_user_image**](ImageApi.md#post_user_image) | **POST** /Users/{userId}/Images/{imageType} | Sets the user image.
[**post_user_image_by_index**](ImageApi.md#post_user_image_by_index) | **POST** /Users/{userId}/Images/{imageType}/{index} | Sets the user image.
[**set_item_image**](ImageApi.md#set_item_image) | **POST** /Items/{itemId}/Images/{imageType} | Set item image.
[**set_item_image_by_index**](ImageApi.md#set_item_image_by_index) | **POST** /Items/{itemId}/Images/{imageType}/{imageIndex} | Set item image.
[**update_item_image_index**](ImageApi.md#update_item_image_index) | **POST** /Items/{itemId}/Images/{imageType}/{imageIndex}/Index | Updates the index for an item image.
[**upload_custom_splashscreen**](ImageApi.md#upload_custom_splashscreen) | **POST** /Branding/Splashscreen | Uploads a custom splashscreen.  The body is expected to the image contents base64 encoded.



## delete_custom_splashscreen

> delete_custom_splashscreen()
Delete a custom splashscreen.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_item_image

> delete_item_image(item_id, image_type, image_index)
Delete an item's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | Option<**i32**> | The image index. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_item_image_by_index

> delete_item_image_by_index(item_id, image_type, image_index)
Delete an item's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | The image index. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_image

> delete_user_image(user_id, image_type, index)
Delete the user's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User Id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | (Unused) Image type. | [required] |
**index** | Option<**i32**> | (Unused) Image index. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_image_by_index

> delete_user_image_by_index(user_id, image_type, index)
Delete the user's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User Id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | (Unused) Image type. | [required] |
**index** | **i32** | (Unused) Image index. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_image

> std::path::PathBuf get_artist_image(name, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get artist image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Artist name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_genre_image

> std::path::PathBuf get_genre_image(name, image_type, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer, image_index)
Get genre image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Genre name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_genre_image_by_index

> std::path::PathBuf get_genre_image_by_index(name, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get genre image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Genre name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_image

> std::path::PathBuf get_item_image(item_id, image_type, max_width, max_height, width, height, quality, fill_width, fill_height, tag, crop_whitespace, format, add_played_indicator, percent_played, unplayed_count, blur, background_color, foreground_layer, image_index)
Gets the item's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_image2

> std::path::PathBuf get_item_image2(item_id, image_type, max_width, max_height, tag, format, percent_played, unplayed_count, image_index, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Gets the item's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**max_width** | **i32** | The maximum image width to return. | [required] |
**max_height** | **i32** | The maximum image height to return. | [required] |
**tag** | **String** | Optional. Supply the cache tag from the item object to receive strong caching headers. | [required] |
**format** | [**crate::models::ImageFormat**](.md) | Determines the output format of the image - original,gif,jpg,png. | [required] |
**percent_played** | **f64** | Optional. Percent to render for the percent played overlay. | [required] |
**unplayed_count** | **i32** | Optional. Unplayed count overlay to render. | [required] |
**image_index** | **i32** | Image index. | [required] |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_image_by_index

> std::path::PathBuf get_item_image_by_index(item_id, image_type, image_index, max_width, max_height, width, height, quality, fill_width, fill_height, tag, crop_whitespace, format, add_played_indicator, percent_played, unplayed_count, blur, background_color, foreground_layer)
Gets the item's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_image_infos

> Vec<crate::models::ImageInfo> get_item_image_infos(item_id)
Get item image infos.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |

### Return type

[**Vec<crate::models::ImageInfo>**](ImageInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_genre_image

> std::path::PathBuf get_music_genre_image(name, image_type, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer, image_index)
Get music genre image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Music genre name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_genre_image_by_index

> std::path::PathBuf get_music_genre_image_by_index(name, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get music genre image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Music genre name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_image

> std::path::PathBuf get_person_image(name, image_type, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer, image_index)
Get person image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Person name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_image_by_index

> std::path::PathBuf get_person_image_by_index(name, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get person image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Person name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_splashscreen

> std::path::PathBuf get_splashscreen(tag, format, max_width, max_height, width, height, fill_width, fill_height, blur, background_color, foreground_layer, quality)
Generates or gets the splashscreen.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | Option<**String**> | Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**blur** | Option<**i32**> | Blur image. |  |
**background_color** | Option<**String**> | Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Apply a foreground layer on top of the image. |  |
**quality** | Option<**i32**> | Quality setting, from 0-100. |  |[default to 90]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_studio_image

> std::path::PathBuf get_studio_image(name, image_type, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer, image_index)
Get studio image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Studio name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_studio_image_by_index

> std::path::PathBuf get_studio_image_by_index(name, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get studio image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Studio name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_image

> std::path::PathBuf get_user_image(user_id, image_type, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer, image_index)
Get user profile image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_image_by_index

> std::path::PathBuf get_user_image_by_index(user_id, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get user profile image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_artist_image

> std::path::PathBuf head_artist_image(name, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get artist image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Artist name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_genre_image

> std::path::PathBuf head_genre_image(name, image_type, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer, image_index)
Get genre image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Genre name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_genre_image_by_index

> std::path::PathBuf head_genre_image_by_index(name, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get genre image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Genre name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_item_image

> std::path::PathBuf head_item_image(item_id, image_type, max_width, max_height, width, height, quality, fill_width, fill_height, tag, crop_whitespace, format, add_played_indicator, percent_played, unplayed_count, blur, background_color, foreground_layer, image_index)
Gets the item's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_item_image2

> std::path::PathBuf head_item_image2(item_id, image_type, max_width, max_height, tag, format, percent_played, unplayed_count, image_index, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Gets the item's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**max_width** | **i32** | The maximum image width to return. | [required] |
**max_height** | **i32** | The maximum image height to return. | [required] |
**tag** | **String** | Optional. Supply the cache tag from the item object to receive strong caching headers. | [required] |
**format** | [**crate::models::ImageFormat**](.md) | Determines the output format of the image - original,gif,jpg,png. | [required] |
**percent_played** | **f64** | Optional. Percent to render for the percent played overlay. | [required] |
**unplayed_count** | **i32** | Optional. Unplayed count overlay to render. | [required] |
**image_index** | **i32** | Image index. | [required] |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_item_image_by_index

> std::path::PathBuf head_item_image_by_index(item_id, image_type, image_index, max_width, max_height, width, height, quality, fill_width, fill_height, tag, crop_whitespace, format, add_played_indicator, percent_played, unplayed_count, blur, background_color, foreground_layer)
Gets the item's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Optional. The MediaBrowser.Model.Drawing.ImageFormat of the returned image. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_music_genre_image

> std::path::PathBuf head_music_genre_image(name, image_type, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer, image_index)
Get music genre image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Music genre name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_music_genre_image_by_index

> std::path::PathBuf head_music_genre_image_by_index(name, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get music genre image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Music genre name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_person_image

> std::path::PathBuf head_person_image(name, image_type, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer, image_index)
Get person image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Person name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_person_image_by_index

> std::path::PathBuf head_person_image_by_index(name, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get person image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Person name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_studio_image

> std::path::PathBuf head_studio_image(name, image_type, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer, image_index)
Get studio image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Studio name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_studio_image_by_index

> std::path::PathBuf head_studio_image_by_index(name, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get studio image by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Studio name. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_user_image

> std::path::PathBuf head_user_image(user_id, image_type, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer, image_index)
Get user profile image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |
**image_index** | Option<**i32**> | Image index. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_user_image_by_index

> std::path::PathBuf head_user_image_by_index(user_id, image_type, image_index, tag, format, max_width, max_height, percent_played, unplayed_count, width, height, quality, fill_width, fill_height, crop_whitespace, add_played_indicator, blur, background_color, foreground_layer)
Get user profile image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Image index. | [required] |
**tag** | Option<**String**> | Optional. Supply the cache tag from the item object to receive strong caching headers. |  |
**format** | Option<[**crate::models::ImageFormat**](.md)> | Determines the output format of the image - original,gif,jpg,png. |  |
**max_width** | Option<**i32**> | The maximum image width to return. |  |
**max_height** | Option<**i32**> | The maximum image height to return. |  |
**percent_played** | Option<**f64**> | Optional. Percent to render for the percent played overlay. |  |
**unplayed_count** | Option<**i32**> | Optional. Unplayed count overlay to render. |  |
**width** | Option<**i32**> | The fixed image width to return. |  |
**height** | Option<**i32**> | The fixed image height to return. |  |
**quality** | Option<**i32**> | Optional. Quality setting, from 0-100. Defaults to 90 and should suffice in most cases. |  |
**fill_width** | Option<**i32**> | Width of box to fill. |  |
**fill_height** | Option<**i32**> | Height of box to fill. |  |
**crop_whitespace** | Option<**bool**> | Optional. Specify if whitespace should be cropped out of the image. True/False. If unspecified, whitespace will be cropped from logos and clear art. |  |
**add_played_indicator** | Option<**bool**> | Optional. Add a played indicator. |  |
**blur** | Option<**i32**> | Optional. Blur image. |  |
**background_color** | Option<**String**> | Optional. Apply a background color for transparent images. |  |
**foreground_layer** | Option<**String**> | Optional. Apply a foreground layer on top of the image. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_user_image

> post_user_image(user_id, image_type, index, body)
Sets the user image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User Id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | (Unused) Image type. | [required] |
**index** | Option<**i32**> | (Unused) Image index. |  |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: image/*
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_user_image_by_index

> post_user_image_by_index(user_id, image_type, index, body)
Sets the user image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User Id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | (Unused) Image type. | [required] |
**index** | **i32** | (Unused) Image index. | [required] |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: image/*
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_item_image

> set_item_image(item_id, image_type, body)
Set item image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: image/*
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_item_image_by_index

> set_item_image_by_index(item_id, image_type, image_index, body)
Set item image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | (Unused) Image index. | [required] |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: image/*
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_item_image_index

> update_item_image_index(item_id, image_type, image_index, new_index)
Updates the index for an item image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** | Item id. | [required] |
**image_type** | [**crate::models::ImageType**](.md) | Image type. | [required] |
**image_index** | **i32** | Old image index. | [required] |
**new_index** | **i32** | New image index. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_custom_splashscreen

> upload_custom_splashscreen(body)
Uploads a custom splashscreen.  The body is expected to the image contents base64 encoded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: image/*
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

