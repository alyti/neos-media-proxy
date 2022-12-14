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
pub struct HttpHeaderInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "Match", skip_serializing_if = "Option::is_none")]
    pub _match: Option<Box<crate::models::HeaderMatchType>>,
}

impl HttpHeaderInfo {
    pub fn new() -> HttpHeaderInfo {
        HttpHeaderInfo {
            name: None,
            value: None,
            _match: None,
        }
    }
}


