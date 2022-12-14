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
pub struct SendMessageCommandRequest {
    #[serde(rename = "Header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "TimeoutMs", skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i64>,
}

impl SendMessageCommandRequest {
    pub fn new(text: String) -> SendMessageCommandRequest {
        SendMessageCommandRequest {
            header: None,
            text,
            timeout_ms: None,
        }
    }
}


