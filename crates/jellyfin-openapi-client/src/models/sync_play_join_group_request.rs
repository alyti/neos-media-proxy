/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SyncPlayJoinGroupRequest : Class JoinGroupRequestDto.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SyncPlayJoinGroupRequest {
    /// Gets or sets the group identifier.
    #[serde(rename = "GroupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl SyncPlayJoinGroupRequest {
    /// Class JoinGroupRequestDto.
    pub fn new() -> SyncPlayJoinGroupRequest {
        SyncPlayJoinGroupRequest {
            group_id: None,
        }
    }
}

