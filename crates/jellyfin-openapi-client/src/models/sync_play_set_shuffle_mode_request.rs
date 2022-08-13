/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SyncPlaySetShuffleModeRequest : Class SetShuffleModeRequestDto.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SyncPlaySetShuffleModeRequest {
    /// Enum GroupShuffleMode.
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Box<crate::models::GroupShuffleMode>>,
}

impl SyncPlaySetShuffleModeRequest {
    /// Class SetShuffleModeRequestDto.
    pub fn new() -> SyncPlaySetShuffleModeRequest {
        SyncPlaySetShuffleModeRequest {
            mode: None,
        }
    }
}

