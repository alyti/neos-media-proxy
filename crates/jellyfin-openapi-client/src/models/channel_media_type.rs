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
pub enum ChannelMediaType {
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Photo")]
    Photo,

}

impl ToString for ChannelMediaType {
    fn to_string(&self) -> String {
        match self {
            Self::Audio => String::from("Audio"),
            Self::Video => String::from("Video"),
            Self::Photo => String::from("Photo"),
        }
    }
}

impl Default for ChannelMediaType {
    fn default() -> ChannelMediaType {
        Self::Audio
    }
}




