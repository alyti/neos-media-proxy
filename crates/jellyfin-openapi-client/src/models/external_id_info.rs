/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ExternalIdInfo : Represents the external id information for serialization to the client.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExternalIdInfo {
    /// Gets or sets the display name of the external id provider (IE: IMDB, MusicBrainz, etc).
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the unique key for this id. This key should be unique across all providers.
    #[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Gets or sets the specific media type for this id. This is used to distinguish between the different  external id types for providers with multiple ids.  A null value indicates there is no specific media type associated with the external id, or this is the  default id for the external provider so there is no need to specify a type.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Box<crate::models::ExternalIdMediaType>>,
    /// Gets or sets the URL format string.
    #[serde(rename = "UrlFormatString", skip_serializing_if = "Option::is_none")]
    pub url_format_string: Option<String>,
}

impl ExternalIdInfo {
    /// Represents the external id information for serialization to the client.
    pub fn new() -> ExternalIdInfo {
        ExternalIdInfo {
            name: None,
            key: None,
            _type: None,
            url_format_string: None,
        }
    }
}


