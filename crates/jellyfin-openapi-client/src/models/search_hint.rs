/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SearchHint : Class SearchHintResult.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SearchHint {
    /// Gets or sets the item id.
    #[serde(rename = "ItemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Gets or sets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the matched term.
    #[serde(rename = "MatchedTerm", skip_serializing_if = "Option::is_none")]
    pub matched_term: Option<String>,
    /// Gets or sets the index number.
    #[serde(rename = "IndexNumber", skip_serializing_if = "Option::is_none")]
    pub index_number: Option<i32>,
    /// Gets or sets the production year.
    #[serde(rename = "ProductionYear", skip_serializing_if = "Option::is_none")]
    pub production_year: Option<i32>,
    /// Gets or sets the parent index number.
    #[serde(rename = "ParentIndexNumber", skip_serializing_if = "Option::is_none")]
    pub parent_index_number: Option<i32>,
    /// Gets or sets the image tag.
    #[serde(rename = "PrimaryImageTag", skip_serializing_if = "Option::is_none")]
    pub primary_image_tag: Option<String>,
    /// Gets or sets the thumb image tag.
    #[serde(rename = "ThumbImageTag", skip_serializing_if = "Option::is_none")]
    pub thumb_image_tag: Option<String>,
    /// Gets or sets the thumb image item identifier.
    #[serde(rename = "ThumbImageItemId", skip_serializing_if = "Option::is_none")]
    pub thumb_image_item_id: Option<String>,
    /// Gets or sets the backdrop image tag.
    #[serde(rename = "BackdropImageTag", skip_serializing_if = "Option::is_none")]
    pub backdrop_image_tag: Option<String>,
    /// Gets or sets the backdrop image item identifier.
    #[serde(rename = "BackdropImageItemId", skip_serializing_if = "Option::is_none")]
    pub backdrop_image_item_id: Option<String>,
    /// Gets or sets the type.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "IsFolder", skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<bool>,
    /// Gets or sets the run time ticks.
    #[serde(rename = "RunTimeTicks", skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<i64>,
    /// Gets or sets the type of the media.
    #[serde(rename = "MediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Gets or sets the series.
    #[serde(rename = "Series", skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Gets or sets the album.
    #[serde(rename = "Album", skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    #[serde(rename = "AlbumId", skip_serializing_if = "Option::is_none")]
    pub album_id: Option<String>,
    /// Gets or sets the album artist.
    #[serde(rename = "AlbumArtist", skip_serializing_if = "Option::is_none")]
    pub album_artist: Option<String>,
    /// Gets or sets the artists.
    #[serde(rename = "Artists", skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<String>>,
    /// Gets or sets the song count.
    #[serde(rename = "SongCount", skip_serializing_if = "Option::is_none")]
    pub song_count: Option<i32>,
    /// Gets or sets the episode count.
    #[serde(rename = "EpisodeCount", skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<i32>,
    /// Gets or sets the channel identifier.
    #[serde(rename = "ChannelId", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// Gets or sets the name of the channel.
    #[serde(rename = "ChannelName", skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// Gets or sets the primary image aspect ratio.
    #[serde(rename = "PrimaryImageAspectRatio", skip_serializing_if = "Option::is_none")]
    pub primary_image_aspect_ratio: Option<f64>,
}

impl SearchHint {
    /// Class SearchHintResult.
    pub fn new() -> SearchHint {
        SearchHint {
            item_id: None,
            id: None,
            name: None,
            matched_term: None,
            index_number: None,
            production_year: None,
            parent_index_number: None,
            primary_image_tag: None,
            thumb_image_tag: None,
            thumb_image_item_id: None,
            backdrop_image_tag: None,
            backdrop_image_item_id: None,
            _type: None,
            is_folder: None,
            run_time_ticks: None,
            media_type: None,
            start_date: None,
            end_date: None,
            series: None,
            status: None,
            album: None,
            album_id: None,
            album_artist: None,
            artists: None,
            song_count: None,
            episode_count: None,
            channel_id: None,
            channel_name: None,
            primary_image_aspect_ratio: None,
        }
    }
}

