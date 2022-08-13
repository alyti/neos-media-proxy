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
pub struct SessionInfoCapabilities {
    #[serde(rename = "PlayableMediaTypes", skip_serializing_if = "Option::is_none")]
    pub playable_media_types: Option<Vec<String>>,
    #[serde(rename = "SupportedCommands", skip_serializing_if = "Option::is_none")]
    pub supported_commands: Option<Vec<crate::models::GeneralCommandType>>,
    #[serde(rename = "SupportsMediaControl", skip_serializing_if = "Option::is_none")]
    pub supports_media_control: Option<bool>,
    #[serde(rename = "SupportsContentUploading", skip_serializing_if = "Option::is_none")]
    pub supports_content_uploading: Option<bool>,
    #[serde(rename = "MessageCallbackUrl", skip_serializing_if = "Option::is_none")]
    pub message_callback_url: Option<String>,
    #[serde(rename = "SupportsPersistentIdentifier", skip_serializing_if = "Option::is_none")]
    pub supports_persistent_identifier: Option<bool>,
    #[serde(rename = "SupportsSync", skip_serializing_if = "Option::is_none")]
    pub supports_sync: Option<bool>,
    #[serde(rename = "DeviceProfile", skip_serializing_if = "Option::is_none")]
    pub device_profile: Option<Box<crate::models::ClientCapabilitiesDeviceProfile>>,
    #[serde(rename = "AppStoreUrl", skip_serializing_if = "Option::is_none")]
    pub app_store_url: Option<String>,
    #[serde(rename = "IconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

impl SessionInfoCapabilities {
    pub fn new() -> SessionInfoCapabilities {
        SessionInfoCapabilities {
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


