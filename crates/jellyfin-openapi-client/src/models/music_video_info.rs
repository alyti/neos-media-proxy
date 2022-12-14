/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MusicVideoInfo {
    /// Gets or sets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the original title.
    #[serde(rename = "OriginalTitle", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,
    /// Gets or sets the path.
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Gets or sets the metadata language.
    #[serde(rename = "MetadataLanguage", skip_serializing_if = "Option::is_none")]
    pub metadata_language: Option<String>,
    /// Gets or sets the metadata country code.
    #[serde(rename = "MetadataCountryCode", skip_serializing_if = "Option::is_none")]
    pub metadata_country_code: Option<String>,
    /// Gets or sets the provider ids.
    #[serde(rename = "ProviderIds", skip_serializing_if = "Option::is_none")]
    pub provider_ids: Option<::std::collections::HashMap<String, String>>,
    /// Gets or sets the year.
    #[serde(rename = "Year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(rename = "IndexNumber", skip_serializing_if = "Option::is_none")]
    pub index_number: Option<i32>,
    #[serde(rename = "ParentIndexNumber", skip_serializing_if = "Option::is_none")]
    pub parent_index_number: Option<i32>,
    #[serde(rename = "PremiereDate", skip_serializing_if = "Option::is_none")]
    pub premiere_date: Option<String>,
    #[serde(rename = "IsAutomated", skip_serializing_if = "Option::is_none")]
    pub is_automated: Option<bool>,
    #[serde(rename = "Artists", skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<String>>,
}

impl MusicVideoInfo {
    pub fn new() -> MusicVideoInfo {
        MusicVideoInfo {
            name: None,
            original_title: None,
            path: None,
            metadata_language: None,
            metadata_country_code: None,
            provider_ids: None,
            year: None,
            index_number: None,
            parent_index_number: None,
            premiere_date: None,
            is_automated: None,
            artists: None,
        }
    }
}


