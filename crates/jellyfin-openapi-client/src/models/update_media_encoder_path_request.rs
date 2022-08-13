/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateMediaEncoderPathRequest : Media Encoder Path Dto.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateMediaEncoderPathRequest {
    /// Gets or sets media encoder path.
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Gets or sets media encoder path type.
    #[serde(rename = "PathType", skip_serializing_if = "Option::is_none")]
    pub path_type: Option<String>,
}

impl UpdateMediaEncoderPathRequest {
    /// Media Encoder Path Dto.
    pub fn new() -> UpdateMediaEncoderPathRequest {
        UpdateMediaEncoderPathRequest {
            path: None,
            path_type: None,
        }
    }
}

