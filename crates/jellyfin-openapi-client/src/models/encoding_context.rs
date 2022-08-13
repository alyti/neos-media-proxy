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
pub enum EncodingContext {
    #[serde(rename = "Streaming")]
    Streaming,
    #[serde(rename = "Static")]
    _Static,

}

impl ToString for EncodingContext {
    fn to_string(&self) -> String {
        match self {
            Self::Streaming => String::from("Streaming"),
            Self::_Static => String::from("Static"),
        }
    }
}

impl Default for EncodingContext {
    fn default() -> EncodingContext {
        Self::Streaming
    }
}



