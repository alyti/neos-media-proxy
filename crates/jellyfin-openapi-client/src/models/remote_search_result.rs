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
pub struct RemoteSearchResult {
    /// Gets or sets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the provider ids.
    #[serde(rename = "ProviderIds", skip_serializing_if = "Option::is_none")]
    pub provider_ids: Option<::std::collections::HashMap<String, String>>,
    /// Gets or sets the year.
    #[serde(rename = "ProductionYear", skip_serializing_if = "Option::is_none")]
    pub production_year: Option<i32>,
    #[serde(rename = "IndexNumber", skip_serializing_if = "Option::is_none")]
    pub index_number: Option<i32>,
    #[serde(rename = "IndexNumberEnd", skip_serializing_if = "Option::is_none")]
    pub index_number_end: Option<i32>,
    #[serde(rename = "ParentIndexNumber", skip_serializing_if = "Option::is_none")]
    pub parent_index_number: Option<i32>,
    #[serde(rename = "PremiereDate", skip_serializing_if = "Option::is_none")]
    pub premiere_date: Option<String>,
    #[serde(rename = "ImageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "SearchProviderName", skip_serializing_if = "Option::is_none")]
    pub search_provider_name: Option<String>,
    #[serde(rename = "Overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "AlbumArtist", skip_serializing_if = "Option::is_none")]
    pub album_artist: Option<Box<crate::models::RemoteSearchResultAlbumArtist>>,
    #[serde(rename = "Artists", skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<crate::models::RemoteSearchResult>>,
}

impl RemoteSearchResult {
    pub fn new() -> RemoteSearchResult {
        RemoteSearchResult {
            name: None,
            provider_ids: None,
            production_year: None,
            index_number: None,
            index_number_end: None,
            parent_index_number: None,
            premiere_date: None,
            image_url: None,
            search_provider_name: None,
            overview: None,
            album_artist: None,
            artists: None,
        }
    }
}


