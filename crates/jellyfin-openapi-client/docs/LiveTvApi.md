# \LiveTvApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_listing_provider**](LiveTvApi.md#add_listing_provider) | **POST** /LiveTv/ListingProviders | Adds a listings provider.
[**add_tuner_host**](LiveTvApi.md#add_tuner_host) | **POST** /LiveTv/TunerHosts | Adds a tuner host.
[**cancel_series_timer**](LiveTvApi.md#cancel_series_timer) | **DELETE** /LiveTv/SeriesTimers/{timerId} | Cancels a live tv series timer.
[**cancel_timer**](LiveTvApi.md#cancel_timer) | **DELETE** /LiveTv/Timers/{timerId} | Cancels a live tv timer.
[**create_series_timer**](LiveTvApi.md#create_series_timer) | **POST** /LiveTv/SeriesTimers | Creates a live tv series timer.
[**create_timer**](LiveTvApi.md#create_timer) | **POST** /LiveTv/Timers | Creates a live tv timer.
[**delete_listing_provider**](LiveTvApi.md#delete_listing_provider) | **DELETE** /LiveTv/ListingProviders | Delete listing provider.
[**delete_recording**](LiveTvApi.md#delete_recording) | **DELETE** /LiveTv/Recordings/{recordingId} | Deletes a live tv recording.
[**delete_tuner_host**](LiveTvApi.md#delete_tuner_host) | **DELETE** /LiveTv/TunerHosts | Deletes a tuner host.
[**discover_tuners**](LiveTvApi.md#discover_tuners) | **GET** /LiveTv/Tuners/Discover | Discover tuners.
[**discvover_tuners**](LiveTvApi.md#discvover_tuners) | **GET** /LiveTv/Tuners/Discvover | Discover tuners.
[**get_channel**](LiveTvApi.md#get_channel) | **GET** /LiveTv/Channels/{channelId} | Gets a live tv channel.
[**get_channel_mapping_options**](LiveTvApi.md#get_channel_mapping_options) | **GET** /LiveTv/ChannelMappingOptions | Get channel mapping options.
[**get_default_listing_provider**](LiveTvApi.md#get_default_listing_provider) | **GET** /LiveTv/ListingProviders/Default | Gets default listings provider info.
[**get_default_timer**](LiveTvApi.md#get_default_timer) | **GET** /LiveTv/Timers/Defaults | Gets the default values for a new timer.
[**get_guide_info**](LiveTvApi.md#get_guide_info) | **GET** /LiveTv/GuideInfo | Get guid info.
[**get_lineups**](LiveTvApi.md#get_lineups) | **GET** /LiveTv/ListingProviders/Lineups | Gets available lineups.
[**get_live_recording_file**](LiveTvApi.md#get_live_recording_file) | **GET** /LiveTv/LiveRecordings/{recordingId}/stream | Gets a live tv recording stream.
[**get_live_stream_file**](LiveTvApi.md#get_live_stream_file) | **GET** /LiveTv/LiveStreamFiles/{streamId}/stream.{container} | Gets a live tv channel stream.
[**get_live_tv_channels**](LiveTvApi.md#get_live_tv_channels) | **GET** /LiveTv/Channels | Gets available live tv channels.
[**get_live_tv_info**](LiveTvApi.md#get_live_tv_info) | **GET** /LiveTv/Info | Gets available live tv services.
[**get_live_tv_programs**](LiveTvApi.md#get_live_tv_programs) | **GET** /LiveTv/Programs | Gets available live tv epgs.
[**get_program**](LiveTvApi.md#get_program) | **GET** /LiveTv/Programs/{programId} | Gets a live tv program.
[**get_programs**](LiveTvApi.md#get_programs) | **POST** /LiveTv/Programs | Gets available live tv epgs.
[**get_recommended_programs**](LiveTvApi.md#get_recommended_programs) | **GET** /LiveTv/Programs/Recommended | Gets recommended live tv epgs.
[**get_recording**](LiveTvApi.md#get_recording) | **GET** /LiveTv/Recordings/{recordingId} | Gets a live tv recording.
[**get_recording_folders**](LiveTvApi.md#get_recording_folders) | **GET** /LiveTv/Recordings/Folders | Gets recording folders.
[**get_recording_group**](LiveTvApi.md#get_recording_group) | **GET** /LiveTv/Recordings/Groups/{groupId} | Get recording group.
[**get_recording_groups**](LiveTvApi.md#get_recording_groups) | **GET** /LiveTv/Recordings/Groups | Gets live tv recording groups.
[**get_recordings**](LiveTvApi.md#get_recordings) | **GET** /LiveTv/Recordings | Gets live tv recordings.
[**get_recordings_series**](LiveTvApi.md#get_recordings_series) | **GET** /LiveTv/Recordings/Series | Gets live tv recording series.
[**get_schedules_direct_countries**](LiveTvApi.md#get_schedules_direct_countries) | **GET** /LiveTv/ListingProviders/SchedulesDirect/Countries | Gets available countries.
[**get_series_timer**](LiveTvApi.md#get_series_timer) | **GET** /LiveTv/SeriesTimers/{timerId} | Gets a live tv series timer.
[**get_series_timers**](LiveTvApi.md#get_series_timers) | **GET** /LiveTv/SeriesTimers | Gets live tv series timers.
[**get_timer**](LiveTvApi.md#get_timer) | **GET** /LiveTv/Timers/{timerId} | Gets a timer.
[**get_timers**](LiveTvApi.md#get_timers) | **GET** /LiveTv/Timers | Gets the live tv timers.
[**get_tuner_host_types**](LiveTvApi.md#get_tuner_host_types) | **GET** /LiveTv/TunerHosts/Types | Get tuner host types.
[**reset_tuner**](LiveTvApi.md#reset_tuner) | **POST** /LiveTv/Tuners/{tunerId}/Reset | Resets a tv tuner.
[**set_channel_mapping**](LiveTvApi.md#set_channel_mapping) | **POST** /LiveTv/ChannelMappings | Set channel mappings.
[**update_series_timer**](LiveTvApi.md#update_series_timer) | **POST** /LiveTv/SeriesTimers/{timerId} | Updates a live tv series timer.
[**update_timer**](LiveTvApi.md#update_timer) | **POST** /LiveTv/Timers/{timerId} | Updates a live tv timer.



## add_listing_provider

> crate::models::ListingsProviderInfo add_listing_provider(pw, validate_listings, validate_login, add_listing_provider_request)
Adds a listings provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pw** | Option<**String**> | Password. |  |
**validate_listings** | Option<**bool**> | Validate listings. |  |[default to false]
**validate_login** | Option<**bool**> | Validate login. |  |[default to false]
**add_listing_provider_request** | Option<[**AddListingProviderRequest**](AddListingProviderRequest.md)> | New listings info. |  |

### Return type

[**crate::models::ListingsProviderInfo**](ListingsProviderInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_tuner_host

> crate::models::TunerHostInfo add_tuner_host(add_tuner_host_request)
Adds a tuner host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_tuner_host_request** | Option<[**AddTunerHostRequest**](AddTunerHostRequest.md)> | New tuner host. |  |

### Return type

[**crate::models::TunerHostInfo**](TunerHostInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_series_timer

> cancel_series_timer(timer_id)
Cancels a live tv series timer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timer_id** | **String** | Timer id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_timer

> cancel_timer(timer_id)
Cancels a live tv timer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timer_id** | **String** | Timer id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_series_timer

> create_series_timer(create_series_timer_request)
Creates a live tv series timer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_series_timer_request** | Option<[**CreateSeriesTimerRequest**](CreateSeriesTimerRequest.md)> | New series timer info. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_timer

> create_timer(create_timer_request)
Creates a live tv timer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_timer_request** | Option<[**CreateTimerRequest**](CreateTimerRequest.md)> | New timer info. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_listing_provider

> delete_listing_provider(id)
Delete listing provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | Listing provider id. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording

> delete_recording(recording_id)
Deletes a live tv recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_id** | **String** | Recording id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tuner_host

> delete_tuner_host(id)
Deletes a tuner host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | Tuner host id. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discover_tuners

> Vec<crate::models::TunerHostInfo> discover_tuners(new_devices_only)
Discover tuners.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_devices_only** | Option<**bool**> | Only discover new tuners. |  |[default to false]

### Return type

[**Vec<crate::models::TunerHostInfo>**](TunerHostInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discvover_tuners

> Vec<crate::models::TunerHostInfo> discvover_tuners(new_devices_only)
Discover tuners.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_devices_only** | Option<**bool**> | Only discover new tuners. |  |[default to false]

### Return type

[**Vec<crate::models::TunerHostInfo>**](TunerHostInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel

> crate::models::BaseItemDto get_channel(channel_id, user_id)
Gets a live tv channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Channel id. | [required] |
**user_id** | Option<**String**> | Optional. Attach user data. |  |

### Return type

[**crate::models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_mapping_options

> crate::models::ChannelMappingOptionsDto get_channel_mapping_options(provider_id)
Get channel mapping options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_id** | Option<**String**> | Provider id. |  |

### Return type

[**crate::models::ChannelMappingOptionsDto**](ChannelMappingOptionsDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_listing_provider

> crate::models::ListingsProviderInfo get_default_listing_provider()
Gets default listings provider info.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListingsProviderInfo**](ListingsProviderInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_timer

> crate::models::SeriesTimerInfoDto get_default_timer(program_id)
Gets the default values for a new timer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | Option<**String**> | Optional. To attach default values based on a program. |  |

### Return type

[**crate::models::SeriesTimerInfoDto**](SeriesTimerInfoDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guide_info

> crate::models::GuideInfo get_guide_info()
Get guid info.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GuideInfo**](GuideInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lineups

> Vec<crate::models::NameIdPair> get_lineups(id, _type, location, country)
Gets available lineups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | Provider id. |  |
**_type** | Option<**String**> | Provider type. |  |
**location** | Option<**String**> | Location. |  |
**country** | Option<**String**> | Country. |  |

### Return type

[**Vec<crate::models::NameIdPair>**](NameIdPair.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_live_recording_file

> std::path::PathBuf get_live_recording_file(recording_id)
Gets a live tv recording stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_id** | **String** | Recording id. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: video/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_live_stream_file

> std::path::PathBuf get_live_stream_file(stream_id, container)
Gets a live tv channel stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **String** | Stream id. | [required] |
**container** | **String** | Container type. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: video/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_live_tv_channels

> crate::models::BaseItemDtoQueryResult get_live_tv_channels(_type, user_id, start_index, is_movie, is_series, is_news, is_kids, is_sports, limit, is_favorite, is_liked, is_disliked, enable_images, image_type_limit, enable_image_types, fields, enable_user_data, sort_by, sort_order, enable_favorite_sorting, add_current_program)
Gets available live tv channels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**crate::models::ChannelType**](.md)> | Optional. Filter by channel type. |  |
**user_id** | Option<**String**> | Optional. Filter by user and attach user data. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**is_movie** | Option<**bool**> | Optional. Filter for movies. |  |
**is_series** | Option<**bool**> | Optional. Filter for series. |  |
**is_news** | Option<**bool**> | Optional. Filter for news. |  |
**is_kids** | Option<**bool**> | Optional. Filter for kids. |  |
**is_sports** | Option<**bool**> | Optional. Filter for sports. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**is_favorite** | Option<**bool**> | Optional. Filter by channels that are favorites, or not. |  |
**is_liked** | Option<**bool**> | Optional. Filter by channels that are liked, or not. |  |
**is_disliked** | Option<**bool**> | Optional. Filter by channels that are disliked, or not. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | \"Optional. The image types to include in the output. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Optional. Key to sort by. |  |
**sort_order** | Option<[**crate::models::SortOrder**](.md)> | Optional. Sort order. |  |
**enable_favorite_sorting** | Option<**bool**> | Optional. Incorporate favorite and like status into channel sorting. |  |[default to false]
**add_current_program** | Option<**bool**> | Optional. Adds current program info to each channel. |  |[default to true]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_live_tv_info

> crate::models::LiveTvInfo get_live_tv_info()
Gets available live tv services.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LiveTvInfo**](LiveTvInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_live_tv_programs

> crate::models::BaseItemDtoQueryResult get_live_tv_programs(channel_ids, user_id, min_start_date, has_aired, is_airing, max_start_date, min_end_date, max_end_date, is_movie, is_series, is_news, is_kids, is_sports, start_index, limit, sort_by, sort_order, genres, genre_ids, enable_images, image_type_limit, enable_image_types, enable_user_data, series_timer_id, library_series_id, fields, enable_total_record_count)
Gets available live tv epgs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_ids** | Option<[**Vec<String>**](String.md)> | The channels to return guide information for. |  |
**user_id** | Option<**String**> | Optional. Filter by user id. |  |
**min_start_date** | Option<**String**> | Optional. The minimum premiere start date. |  |
**has_aired** | Option<**bool**> | Optional. Filter by programs that have completed airing, or not. |  |
**is_airing** | Option<**bool**> | Optional. Filter by programs that are currently airing, or not. |  |
**max_start_date** | Option<**String**> | Optional. The maximum premiere start date. |  |
**min_end_date** | Option<**String**> | Optional. The minimum premiere end date. |  |
**max_end_date** | Option<**String**> | Optional. The maximum premiere end date. |  |
**is_movie** | Option<**bool**> | Optional. Filter for movies. |  |
**is_series** | Option<**bool**> | Optional. Filter for series. |  |
**is_news** | Option<**bool**> | Optional. Filter for news. |  |
**is_kids** | Option<**bool**> | Optional. Filter for kids. |  |
**is_sports** | Option<**bool**> | Optional. Filter for sports. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Optional. Specify one or more sort orders, comma delimited. Options: Name, StartDate. |  |
**sort_order** | Option<[**Vec<crate::models::SortOrder>**](crate::models::SortOrder.md)> | Sort Order - Ascending,Descending. |  |
**genres** | Option<[**Vec<String>**](String.md)> | The genres to return guide information for. |  |
**genre_ids** | Option<[**Vec<String>**](String.md)> | The genre ids to return guide information for. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**series_timer_id** | Option<**String**> | Optional. Filter by series timer id. |  |
**library_series_id** | Option<**String**> | Optional. Filter by library series id. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_total_record_count** | Option<**bool**> | Retrieve total record count. |  |[default to true]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_program

> crate::models::BaseItemDto get_program(program_id, user_id)
Gets a live tv program.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | **String** | Program id. | [required] |
**user_id** | Option<**String**> | Optional. Attach user data. |  |

### Return type

[**crate::models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_programs

> crate::models::BaseItemDtoQueryResult get_programs(get_programs_request)
Gets available live tv epgs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_programs_request** | Option<[**GetProgramsRequest**](GetProgramsRequest.md)> | Request body. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recommended_programs

> crate::models::BaseItemDtoQueryResult get_recommended_programs(user_id, limit, is_airing, has_aired, is_series, is_movie, is_news, is_kids, is_sports, enable_images, image_type_limit, enable_image_types, genre_ids, fields, enable_user_data, enable_total_record_count)
Gets recommended live tv epgs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | Optional. filter by user id. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**is_airing** | Option<**bool**> | Optional. Filter by programs that are currently airing, or not. |  |
**has_aired** | Option<**bool**> | Optional. Filter by programs that have completed airing, or not. |  |
**is_series** | Option<**bool**> | Optional. Filter for series. |  |
**is_movie** | Option<**bool**> | Optional. Filter for movies. |  |
**is_news** | Option<**bool**> | Optional. Filter for news. |  |
**is_kids** | Option<**bool**> | Optional. Filter for kids. |  |
**is_sports** | Option<**bool**> | Optional. Filter for sports. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**genre_ids** | Option<[**Vec<String>**](String.md)> | The genres to return guide information for. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. include user data. |  |
**enable_total_record_count** | Option<**bool**> | Retrieve total record count. |  |[default to true]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording

> crate::models::BaseItemDto get_recording(recording_id, user_id)
Gets a live tv recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_id** | **String** | Recording id. | [required] |
**user_id** | Option<**String**> | Optional. Attach user data. |  |

### Return type

[**crate::models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_folders

> crate::models::BaseItemDtoQueryResult get_recording_folders(user_id)
Gets recording folders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | Optional. Filter by user and attach user data. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_group

> get_recording_group(group_id)
Get recording group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recording_groups

> crate::models::BaseItemDtoQueryResult get_recording_groups(user_id)
Gets live tv recording groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> | Optional. Filter by user and attach user data. |  |

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recordings

> crate::models::BaseItemDtoQueryResult get_recordings(channel_id, user_id, start_index, limit, status, is_in_progress, series_timer_id, enable_images, image_type_limit, enable_image_types, fields, enable_user_data, is_movie, is_series, is_kids, is_sports, is_news, is_library_item, enable_total_record_count)
Gets live tv recordings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | Option<**String**> | Optional. Filter by channel id. |  |
**user_id** | Option<**String**> | Optional. Filter by user and attach user data. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**status** | Option<[**crate::models::RecordingStatus**](.md)> | Optional. Filter by recording status. |  |
**is_in_progress** | Option<**bool**> | Optional. Filter by recordings that are in progress, or not. |  |
**series_timer_id** | Option<**String**> | Optional. Filter by recordings belonging to a series timer. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**is_movie** | Option<**bool**> | Optional. Filter for movies. |  |
**is_series** | Option<**bool**> | Optional. Filter for series. |  |
**is_kids** | Option<**bool**> | Optional. Filter for kids. |  |
**is_sports** | Option<**bool**> | Optional. Filter for sports. |  |
**is_news** | Option<**bool**> | Optional. Filter for news. |  |
**is_library_item** | Option<**bool**> | Optional. Filter for is library item. |  |
**enable_total_record_count** | Option<**bool**> | Optional. Return total record count. |  |[default to true]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recordings_series

> crate::models::BaseItemDtoQueryResult get_recordings_series(channel_id, user_id, group_id, start_index, limit, status, is_in_progress, series_timer_id, enable_images, image_type_limit, enable_image_types, fields, enable_user_data, enable_total_record_count)
Gets live tv recording series.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | Option<**String**> | Optional. Filter by channel id. |  |
**user_id** | Option<**String**> | Optional. Filter by user and attach user data. |  |
**group_id** | Option<**String**> | Optional. Filter by recording group. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**status** | Option<[**crate::models::RecordingStatus**](.md)> | Optional. Filter by recording status. |  |
**is_in_progress** | Option<**bool**> | Optional. Filter by recordings that are in progress, or not. |  |
**series_timer_id** | Option<**String**> | Optional. Filter by recordings belonging to a series timer. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](crate::models::ImageType.md)> | Optional. The image types to include in the output. |  |
**fields** | Option<[**Vec<crate::models::ItemFields>**](crate::models::ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**enable_total_record_count** | Option<**bool**> | Optional. Return total record count. |  |[default to true]

### Return type

[**crate::models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schedules_direct_countries

> std::path::PathBuf get_schedules_direct_countries()
Gets available countries.

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_timer

> crate::models::SeriesTimerInfoDto get_series_timer(timer_id)
Gets a live tv series timer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timer_id** | **String** | Timer id. | [required] |

### Return type

[**crate::models::SeriesTimerInfoDto**](SeriesTimerInfoDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_timers

> crate::models::SeriesTimerInfoDtoQueryResult get_series_timers(sort_by, sort_order)
Gets live tv series timers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort_by** | Option<**String**> | Optional. Sort by SortName or Priority. |  |
**sort_order** | Option<[**crate::models::SortOrder**](.md)> | Optional. Sort in Ascending or Descending order. |  |

### Return type

[**crate::models::SeriesTimerInfoDtoQueryResult**](SeriesTimerInfoDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_timer

> crate::models::TimerInfoDto get_timer(timer_id)
Gets a timer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timer_id** | **String** | Timer id. | [required] |

### Return type

[**crate::models::TimerInfoDto**](TimerInfoDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_timers

> crate::models::TimerInfoDtoQueryResult get_timers(channel_id, series_timer_id, is_active, is_scheduled)
Gets the live tv timers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | Option<**String**> | Optional. Filter by channel id. |  |
**series_timer_id** | Option<**String**> | Optional. Filter by timers belonging to a series timer. |  |
**is_active** | Option<**bool**> | Optional. Filter by timers that are active. |  |
**is_scheduled** | Option<**bool**> | Optional. Filter by timers that are scheduled. |  |

### Return type

[**crate::models::TimerInfoDtoQueryResult**](TimerInfoDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tuner_host_types

> Vec<crate::models::NameIdPair> get_tuner_host_types()
Get tuner host types.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::NameIdPair>**](NameIdPair.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_tuner

> reset_tuner(tuner_id)
Resets a tv tuner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tuner_id** | **String** | Tuner id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_channel_mapping

> crate::models::TunerChannelMapping set_channel_mapping(set_channel_mapping_request)
Set channel mappings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_channel_mapping_request** | [**SetChannelMappingRequest**](SetChannelMappingRequest.md) | The set channel mapping dto. | [required] |

### Return type

[**crate::models::TunerChannelMapping**](TunerChannelMapping.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_series_timer

> update_series_timer(timer_id, create_series_timer_request)
Updates a live tv series timer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timer_id** | **String** | Timer id. | [required] |
**create_series_timer_request** | Option<[**CreateSeriesTimerRequest**](CreateSeriesTimerRequest.md)> | New series timer info. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_timer

> update_timer(timer_id, create_timer_request)
Updates a live tv timer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timer_id** | **String** | Timer id. | [required] |
**create_timer_request** | Option<[**CreateTimerRequest**](CreateTimerRequest.md)> | New timer info. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

