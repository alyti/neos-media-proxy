/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateMediaPathRequest : Update library options dto.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateMediaPathRequest {
    /// Gets or sets the library name.
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PathInfo")]
    pub path_info: Box<crate::models::UpdateMediaPathRequestDtoPathInfo>,
}

impl UpdateMediaPathRequest {
    /// Update library options dto.
    pub fn new(name: String, path_info: crate::models::UpdateMediaPathRequestDtoPathInfo) -> UpdateMediaPathRequest {
        UpdateMediaPathRequest {
            name,
            path_info: Box::new(path_info),
        }
    }
}

