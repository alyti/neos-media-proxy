/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostFullCapabilitiesRequest : Client capabilities dto.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostFullCapabilitiesRequest {
    /// Gets or sets the list of playable media types.
    #[serde(rename = "PlayableMediaTypes", skip_serializing_if = "Option::is_none")]
    pub playable_media_types: Option<Vec<String>>,
    /// Gets or sets the list of supported commands.
    #[serde(rename = "SupportedCommands", skip_serializing_if = "Option::is_none")]
    pub supported_commands: Option<Vec<crate::models::GeneralCommandType>>,
    /// Gets or sets a value indicating whether session supports media control.
    #[serde(rename = "SupportsMediaControl", skip_serializing_if = "Option::is_none")]
    pub supports_media_control: Option<bool>,
    /// Gets or sets a value indicating whether session supports content uploading.
    #[serde(rename = "SupportsContentUploading", skip_serializing_if = "Option::is_none")]
    pub supports_content_uploading: Option<bool>,
    /// Gets or sets the message callback url.
    #[serde(rename = "MessageCallbackUrl", skip_serializing_if = "Option::is_none")]
    pub message_callback_url: Option<String>,
    /// Gets or sets a value indicating whether session supports a persistent identifier.
    #[serde(rename = "SupportsPersistentIdentifier", skip_serializing_if = "Option::is_none")]
    pub supports_persistent_identifier: Option<bool>,
    /// Gets or sets a value indicating whether session supports sync.
    #[serde(rename = "SupportsSync", skip_serializing_if = "Option::is_none")]
    pub supports_sync: Option<bool>,
    #[serde(rename = "DeviceProfile", skip_serializing_if = "Option::is_none")]
    pub device_profile: Option<Box<crate::models::ClientCapabilitiesDeviceProfile>>,
    /// Gets or sets the app store url.
    #[serde(rename = "AppStoreUrl", skip_serializing_if = "Option::is_none")]
    pub app_store_url: Option<String>,
    /// Gets or sets the icon url.
    #[serde(rename = "IconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

impl PostFullCapabilitiesRequest {
    /// Client capabilities dto.
    pub fn new() -> PostFullCapabilitiesRequest {
        PostFullCapabilitiesRequest {
            playable_media_types: None,
            supported_commands: None,
            supports_media_control: None,
            supports_content_uploading: None,
            message_callback_url: None,
            supports_persistent_identifier: None,
            supports_sync: None,
            device_profile: None,
            app_store_url: None,
            icon_url: None,
        }
    }
}


