/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SyncPlayCreateGroupRequest : Class NewGroupRequestDto.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SyncPlayCreateGroupRequest {
    /// Gets or sets the group name.
    #[serde(rename = "GroupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl SyncPlayCreateGroupRequest {
    /// Class NewGroupRequestDto.
    pub fn new() -> SyncPlayCreateGroupRequest {
        SyncPlayCreateGroupRequest {
            group_name: None,
        }
    }
}


