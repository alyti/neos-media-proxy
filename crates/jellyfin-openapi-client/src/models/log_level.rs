/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "Trace")]
    Trace,
    #[serde(rename = "Debug")]
    Debug,
    #[serde(rename = "Information")]
    Information,
    #[serde(rename = "Warning")]
    Warning,
    #[serde(rename = "Error")]
    Error,
    #[serde(rename = "Critical")]
    Critical,
    #[serde(rename = "None")]
    None,

}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Trace => String::from("Trace"),
            Self::Debug => String::from("Debug"),
            Self::Information => String::from("Information"),
            Self::Warning => String::from("Warning"),
            Self::Error => String::from("Error"),
            Self::Critical => String::from("Critical"),
            Self::None => String::from("None"),
        }
    }
}

impl Default for LogLevel {
    fn default() -> LogLevel {
        Self::Trace
    }
}




