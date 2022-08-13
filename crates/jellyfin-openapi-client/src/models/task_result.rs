/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TaskResult : Class TaskExecutionInfo.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaskResult {
    /// Gets or sets the start time UTC.
    #[serde(rename = "StartTimeUtc", skip_serializing_if = "Option::is_none")]
    pub start_time_utc: Option<String>,
    /// Gets or sets the end time UTC.
    #[serde(rename = "EndTimeUtc", skip_serializing_if = "Option::is_none")]
    pub end_time_utc: Option<String>,
    /// Gets or sets the status.
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::TaskCompletionStatus>>,
    /// Gets or sets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the key.
    #[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Gets or sets the id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Gets or sets the error message.
    #[serde(rename = "ErrorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Gets or sets the long error message.
    #[serde(rename = "LongErrorMessage", skip_serializing_if = "Option::is_none")]
    pub long_error_message: Option<String>,
}

impl TaskResult {
    /// Class TaskExecutionInfo.
    pub fn new() -> TaskResult {
        TaskResult {
            start_time_utc: None,
            end_time_utc: None,
            status: None,
            name: None,
            key: None,
            id: None,
            error_message: None,
            long_error_message: None,
        }
    }
}

