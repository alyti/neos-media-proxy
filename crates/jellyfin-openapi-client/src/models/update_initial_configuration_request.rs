/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateInitialConfigurationRequest : The startup configuration DTO.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateInitialConfigurationRequest {
    /// Gets or sets UI language culture.
    #[serde(rename = "UICulture", skip_serializing_if = "Option::is_none")]
    pub ui_culture: Option<String>,
    /// Gets or sets the metadata country code.
    #[serde(rename = "MetadataCountryCode", skip_serializing_if = "Option::is_none")]
    pub metadata_country_code: Option<String>,
    /// Gets or sets the preferred language for the metadata.
    #[serde(rename = "PreferredMetadataLanguage", skip_serializing_if = "Option::is_none")]
    pub preferred_metadata_language: Option<String>,
}

impl UpdateInitialConfigurationRequest {
    /// The startup configuration DTO.
    pub fn new() -> UpdateInitialConfigurationRequest {
        UpdateInitialConfigurationRequest {
            ui_culture: None,
            metadata_country_code: None,
            preferred_metadata_language: None,
        }
    }
}


