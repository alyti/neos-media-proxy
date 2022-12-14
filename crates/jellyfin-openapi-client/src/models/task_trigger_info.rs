/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TaskTriggerInfo : Class TaskTriggerInfo.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaskTriggerInfo {
    /// Gets or sets the type.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Gets or sets the time of day.
    #[serde(rename = "TimeOfDayTicks", skip_serializing_if = "Option::is_none")]
    pub time_of_day_ticks: Option<i64>,
    /// Gets or sets the interval.
    #[serde(rename = "IntervalTicks", skip_serializing_if = "Option::is_none")]
    pub interval_ticks: Option<i64>,
    /// Gets or sets the day of week.
    #[serde(rename = "DayOfWeek", skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<Box<crate::models::DayOfWeek>>,
    /// Gets or sets the maximum runtime ticks.
    #[serde(rename = "MaxRuntimeTicks", skip_serializing_if = "Option::is_none")]
    pub max_runtime_ticks: Option<i64>,
}

impl TaskTriggerInfo {
    /// Class TaskTriggerInfo.
    pub fn new() -> TaskTriggerInfo {
        TaskTriggerInfo {
            _type: None,
            time_of_day_ticks: None,
            interval_ticks: None,
            day_of_week: None,
            max_runtime_ticks: None,
        }
    }
}


