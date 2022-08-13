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
pub struct GetPersonRemoteSearchResultsRequest {
    #[serde(rename = "SearchInfo", skip_serializing_if = "Option::is_none")]
    pub search_info: Option<Box<crate::models::PersonLookupInfoRemoteSearchQuerySearchInfo>>,
    #[serde(rename = "ItemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// Gets or sets the provider name to search within if set.
    #[serde(rename = "SearchProviderName", skip_serializing_if = "Option::is_none")]
    pub search_provider_name: Option<String>,
    /// Gets or sets a value indicating whether disabled providers should be included.
    #[serde(rename = "IncludeDisabledProviders", skip_serializing_if = "Option::is_none")]
    pub include_disabled_providers: Option<bool>,
}

impl GetPersonRemoteSearchResultsRequest {
    pub fn new() -> GetPersonRemoteSearchResultsRequest {
        GetPersonRemoteSearchResultsRequest {
            search_info: None,
            item_id: None,
            search_provider_name: None,
            include_disabled_providers: None,
        }
    }
}


