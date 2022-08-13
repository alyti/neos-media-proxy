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
pub struct ListingsProviderInfo {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "Password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "ListingsId", skip_serializing_if = "Option::is_none")]
    pub listings_id: Option<String>,
    #[serde(rename = "ZipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    #[serde(rename = "Country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "EnabledTuners", skip_serializing_if = "Option::is_none")]
    pub enabled_tuners: Option<Vec<String>>,
    #[serde(rename = "EnableAllTuners", skip_serializing_if = "Option::is_none")]
    pub enable_all_tuners: Option<bool>,
    #[serde(rename = "NewsCategories", skip_serializing_if = "Option::is_none")]
    pub news_categories: Option<Vec<String>>,
    #[serde(rename = "SportsCategories", skip_serializing_if = "Option::is_none")]
    pub sports_categories: Option<Vec<String>>,
    #[serde(rename = "KidsCategories", skip_serializing_if = "Option::is_none")]
    pub kids_categories: Option<Vec<String>>,
    #[serde(rename = "MovieCategories", skip_serializing_if = "Option::is_none")]
    pub movie_categories: Option<Vec<String>>,
    #[serde(rename = "ChannelMappings", skip_serializing_if = "Option::is_none")]
    pub channel_mappings: Option<Vec<crate::models::NameValuePair>>,
    #[serde(rename = "MoviePrefix", skip_serializing_if = "Option::is_none")]
    pub movie_prefix: Option<String>,
    #[serde(rename = "PreferredLanguage", skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(rename = "UserAgent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

impl ListingsProviderInfo {
    pub fn new() -> ListingsProviderInfo {
        ListingsProviderInfo {
            id: None,
            _type: None,
            username: None,
            password: None,
            listings_id: None,
            zip_code: None,
            country: None,
            path: None,
            enabled_tuners: None,
            enable_all_tuners: None,
            news_categories: None,
            sports_categories: None,
            kids_categories: None,
            movie_categories: None,
            channel_mappings: None,
            movie_prefix: None,
            preferred_language: None,
            user_agent: None,
        }
    }
}


