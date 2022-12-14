/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VersionInfo : Defines the MediaBrowser.Model.Updates.VersionInfo class.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Gets or sets the version.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Gets the version as a System.Version.
    #[serde(rename = "VersionNumber", skip_serializing_if = "Option::is_none")]
    pub version_number: Option<String>,
    /// Gets or sets the changelog for this version.
    #[serde(rename = "changelog", skip_serializing_if = "Option::is_none")]
    pub changelog: Option<String>,
    /// Gets or sets the ABI that this version was built against.
    #[serde(rename = "targetAbi", skip_serializing_if = "Option::is_none")]
    pub target_abi: Option<String>,
    /// Gets or sets the source URL.
    #[serde(rename = "sourceUrl", skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// Gets or sets a checksum for the binary.
    #[serde(rename = "checksum", skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// Gets or sets a timestamp of when the binary was built.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Gets or sets the repository name.
    #[serde(rename = "repositoryName", skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    /// Gets or sets the repository url.
    #[serde(rename = "repositoryUrl", skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
}

impl VersionInfo {
    /// Defines the MediaBrowser.Model.Updates.VersionInfo class.
    pub fn new() -> VersionInfo {
        VersionInfo {
            version: None,
            version_number: None,
            changelog: None,
            target_abi: None,
            source_url: None,
            checksum: None,
            timestamp: None,
            repository_name: None,
            repository_url: None,
        }
    }
}


