/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MovePlaylistItemRequestDto : Class MovePlaylistItemRequestDto.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MovePlaylistItemRequestDto {
    /// Gets or sets the playlist identifier of the item.
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option::is_none")]
    pub playlist_item_id: Option<String>,
    /// Gets or sets the new position.
    #[serde(rename = "NewIndex", skip_serializing_if = "Option::is_none")]
    pub new_index: Option<i32>,
}

impl MovePlaylistItemRequestDto {
    /// Class MovePlaylistItemRequestDto.
    pub fn new() -> MovePlaylistItemRequestDto {
        MovePlaylistItemRequestDto {
            playlist_item_id: None,
            new_index: None,
        }
    }
}


