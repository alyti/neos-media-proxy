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
pub struct TimerEventInfo {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ProgramId", skip_serializing_if = "Option::is_none")]
    pub program_id: Option<String>,
}

impl TimerEventInfo {
    pub fn new() -> TimerEventInfo {
        TimerEventInfo {
            id: None,
            program_id: None,
        }
    }
}


