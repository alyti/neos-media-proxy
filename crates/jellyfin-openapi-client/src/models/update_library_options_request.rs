/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateLibraryOptionsRequest : Update library options dto.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateLibraryOptionsRequest {
    /// Gets or sets the library item id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LibraryOptions", skip_serializing_if = "Option::is_none")]
    pub library_options: Option<Box<crate::models::AddVirtualFolderDtoLibraryOptions>>,
}

impl UpdateLibraryOptionsRequest {
    /// Update library options dto.
    pub fn new() -> UpdateLibraryOptionsRequest {
        UpdateLibraryOptionsRequest {
            id: None,
            library_options: None,
        }
    }
}


