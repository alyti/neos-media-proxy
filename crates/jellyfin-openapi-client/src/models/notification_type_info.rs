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
pub struct NotificationTypeInfo {
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "IsBasedOnUserEvent", skip_serializing_if = "Option::is_none")]
    pub is_based_on_user_event: Option<bool>,
}

impl NotificationTypeInfo {
    pub fn new() -> NotificationTypeInfo {
        NotificationTypeInfo {
            _type: None,
            name: None,
            enabled: None,
            category: None,
            is_based_on_user_event: None,
        }
    }
}

