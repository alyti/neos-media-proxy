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
pub enum CodecType {
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "VideoAudio")]
    VideoAudio,
    #[serde(rename = "Audio")]
    Audio,

}

impl ToString for CodecType {
    fn to_string(&self) -> String {
        match self {
            Self::Video => String::from("Video"),
            Self::VideoAudio => String::from("VideoAudio"),
            Self::Audio => String::from("Audio"),
        }
    }
}

impl Default for CodecType {
    fn default() -> CodecType {
        Self::Video
    }
}



