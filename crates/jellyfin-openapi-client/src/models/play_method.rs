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
pub enum PlayMethod {
    #[serde(rename = "Transcode")]
    Transcode,
    #[serde(rename = "DirectStream")]
    DirectStream,
    #[serde(rename = "DirectPlay")]
    DirectPlay,

}

impl ToString for PlayMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Transcode => String::from("Transcode"),
            Self::DirectStream => String::from("DirectStream"),
            Self::DirectPlay => String::from("DirectPlay"),
        }
    }
}

impl Default for PlayMethod {
    fn default() -> PlayMethod {
        Self::Transcode
    }
}




