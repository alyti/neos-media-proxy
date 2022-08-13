/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ValidatePathRequest : Validate path object.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValidatePathRequest {
    /// Gets or sets a value indicating whether validate if path is writable.
    #[serde(rename = "ValidateWritable", skip_serializing_if = "Option::is_none")]
    pub validate_writable: Option<bool>,
    /// Gets or sets the path.
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Gets or sets is path file.
    #[serde(rename = "IsFile", skip_serializing_if = "Option::is_none")]
    pub is_file: Option<bool>,
}

impl ValidatePathRequest {
    /// Validate path object.
    pub fn new() -> ValidatePathRequest {
        ValidatePathRequest {
            validate_writable: None,
            path: None,
            is_file: None,
        }
    }
}

