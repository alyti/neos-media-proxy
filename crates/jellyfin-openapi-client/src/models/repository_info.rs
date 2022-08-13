/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RepositoryInfo : Class RepositoryInfo.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RepositoryInfo {
    /// Gets or sets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the URL.
    #[serde(rename = "Url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Gets or sets a value indicating whether the repository is enabled.
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl RepositoryInfo {
    /// Class RepositoryInfo.
    pub fn new() -> RepositoryInfo {
        RepositoryInfo {
            name: None,
            url: None,
            enabled: None,
        }
    }
}


