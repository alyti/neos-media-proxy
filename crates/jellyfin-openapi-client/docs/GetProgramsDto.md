# GetProgramsDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel_ids** | Option<**Vec<String>**> | Gets or sets the channels to return guide information for. | [optional]
**user_id** | Option<**String**> | Gets or sets optional. Filter by user id. | [optional]
**min_start_date** | Option<**String**> | Gets or sets the minimum premiere start date.  Optional. | [optional]
**has_aired** | Option<**bool**> | Gets or sets filter by programs that have completed airing, or not.  Optional. | [optional]
**is_airing** | Option<**bool**> | Gets or sets filter by programs that are currently airing, or not.  Optional. | [optional]
**max_start_date** | Option<**String**> | Gets or sets the maximum premiere start date.  Optional. | [optional]
**min_end_date** | Option<**String**> | Gets or sets the minimum premiere end date.  Optional. | [optional]
**max_end_date** | Option<**String**> | Gets or sets the maximum premiere end date.  Optional. | [optional]
**is_movie** | Option<**bool**> | Gets or sets filter for movies.  Optional. | [optional]
**is_series** | Option<**bool**> | Gets or sets filter for series.  Optional. | [optional]
**is_news** | Option<**bool**> | Gets or sets filter for news.  Optional. | [optional]
**is_kids** | Option<**bool**> | Gets or sets filter for kids.  Optional. | [optional]
**is_sports** | Option<**bool**> | Gets or sets filter for sports.  Optional. | [optional]
**start_index** | Option<**i32**> | Gets or sets the record index to start at. All items with a lower index will be dropped from the results.  Optional. | [optional]
**limit** | Option<**i32**> | Gets or sets the maximum number of records to return.  Optional. | [optional]
**sort_by** | Option<**Vec<String>**> | Gets or sets specify one or more sort orders, comma delimited. Options: Name, StartDate.  Optional. | [optional]
**sort_order** | Option<[**Vec<crate::models::SortOrder>**](SortOrder.md)> | Gets or sets sort Order - Ascending,Descending. | [optional]
**genres** | Option<**Vec<String>**> | Gets or sets the genres to return guide information for. | [optional]
**genre_ids** | Option<**Vec<String>**> | Gets or sets the genre ids to return guide information for. | [optional]
**enable_images** | Option<**bool**> | Gets or sets include image information in output.  Optional. | [optional]
**enable_total_record_count** | Option<**bool**> | Gets or sets a value indicating whether retrieve total record count. | [optional]
**image_type_limit** | Option<**i32**> | Gets or sets the max number of images to return, per image type.  Optional. | [optional]
**enable_image_types** | Option<[**Vec<crate::models::ImageType>**](ImageType.md)> | Gets or sets the image types to include in the output.  Optional. | [optional]
**enable_user_data** | Option<**bool**> | Gets or sets include user data.  Optional. | [optional]
**series_timer_id** | Option<**String**> | Gets or sets filter by series timer id.  Optional. | [optional]
**library_series_id** | Option<**String**> | Gets or sets filter by library series id.  Optional. | [optional]
**fields** | Option<[**Vec<crate::models::ItemFields>**](ItemFields.md)> | Gets or sets specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines.  Optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


