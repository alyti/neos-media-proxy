/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SessionInfoFullNowPlayingItem : Class BaseItem.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SessionInfoFullNowPlayingItem {
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "IsHD", skip_serializing_if = "Option::is_none")]
    pub is_hd: Option<bool>,
    #[serde(rename = "IsShortcut", skip_serializing_if = "Option::is_none")]
    pub is_shortcut: Option<bool>,
    #[serde(rename = "ShortcutPath", skip_serializing_if = "Option::is_none")]
    pub shortcut_path: Option<String>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "ExtraIds", skip_serializing_if = "Option::is_none")]
    pub extra_ids: Option<Vec<String>>,
    #[serde(rename = "DateLastSaved", skip_serializing_if = "Option::is_none")]
    pub date_last_saved: Option<String>,
    /// Gets or sets the remote trailers.
    #[serde(rename = "RemoteTrailers", skip_serializing_if = "Option::is_none")]
    pub remote_trailers: Option<Vec<crate::models::MediaUrl>>,
    #[serde(rename = "SupportsExternalTransfer", skip_serializing_if = "Option::is_none")]
    pub supports_external_transfer: Option<bool>,
}

impl SessionInfoFullNowPlayingItem {
    /// Class BaseItem.
    pub fn new() -> SessionInfoFullNowPlayingItem {
        SessionInfoFullNowPlayingItem {
            size: None,
            container: None,
            is_hd: None,
            is_shortcut: None,
            shortcut_path: None,
            width: None,
            height: None,
            extra_ids: None,
            date_last_saved: None,
            remote_trailers: None,
            supports_external_transfer: None,
        }
    }
}


