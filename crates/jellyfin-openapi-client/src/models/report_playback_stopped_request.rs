/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReportPlaybackStoppedRequest : Class PlaybackStopInfo.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReportPlaybackStoppedRequest {
    #[serde(rename = "Item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<crate::models::PlaybackProgressInfoItem>>,
    /// Gets or sets the item identifier.
    #[serde(rename = "ItemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// Gets or sets the session id.
    #[serde(rename = "SessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Gets or sets the media version identifier.
    #[serde(rename = "MediaSourceId", skip_serializing_if = "Option::is_none")]
    pub media_source_id: Option<String>,
    /// Gets or sets the position ticks.
    #[serde(rename = "PositionTicks", skip_serializing_if = "Option::is_none")]
    pub position_ticks: Option<i64>,
    /// Gets or sets the live stream identifier.
    #[serde(rename = "LiveStreamId", skip_serializing_if = "Option::is_none")]
    pub live_stream_id: Option<String>,
    /// Gets or sets the play session identifier.
    #[serde(rename = "PlaySessionId", skip_serializing_if = "Option::is_none")]
    pub play_session_id: Option<String>,
    /// Gets or sets a value indicating whether this MediaBrowser.Model.Session.PlaybackStopInfo is failed.
    #[serde(rename = "Failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(rename = "NextMediaType", skip_serializing_if = "Option::is_none")]
    pub next_media_type: Option<String>,
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option::is_none")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "NowPlayingQueue", skip_serializing_if = "Option::is_none")]
    pub now_playing_queue: Option<Vec<crate::models::QueueItem>>,
}

impl ReportPlaybackStoppedRequest {
    /// Class PlaybackStopInfo.
    pub fn new() -> ReportPlaybackStoppedRequest {
        ReportPlaybackStoppedRequest {
            item: None,
            item_id: None,
            session_id: None,
            media_source_id: None,
            position_ticks: None,
            live_stream_id: None,
            play_session_id: None,
            failed: None,
            next_media_type: None,
            playlist_item_id: None,
            now_playing_queue: None,
        }
    }
}


