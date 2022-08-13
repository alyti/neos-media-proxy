/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateDeviceOptionsRequest : A dto representing custom options for a device.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateDeviceOptionsRequest {
    /// Gets or sets the id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Gets or sets the device id.
    #[serde(rename = "DeviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// Gets or sets the custom name.
    #[serde(rename = "CustomName", skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
}

impl UpdateDeviceOptionsRequest {
    /// A dto representing custom options for a device.
    pub fn new() -> UpdateDeviceOptionsRequest {
        UpdateDeviceOptionsRequest {
            id: None,
            device_id: None,
            custom_name: None,
        }
    }
}


