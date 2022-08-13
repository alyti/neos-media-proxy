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
pub struct AllThemeMediaResult {
    #[serde(rename = "ThemeVideosResult", skip_serializing_if = "Option::is_none")]
    pub theme_videos_result: Option<Box<crate::models::AllThemeMediaResultThemeVideosResult>>,
    #[serde(rename = "ThemeSongsResult", skip_serializing_if = "Option::is_none")]
    pub theme_songs_result: Option<Box<crate::models::AllThemeMediaResultThemeVideosResult>>,
    #[serde(rename = "SoundtrackSongsResult", skip_serializing_if = "Option::is_none")]
    pub soundtrack_songs_result: Option<Box<crate::models::AllThemeMediaResultThemeVideosResult>>,
}

impl AllThemeMediaResult {
    pub fn new() -> AllThemeMediaResult {
        AllThemeMediaResult {
            theme_videos_result: None,
            theme_songs_result: None,
            soundtrack_songs_result: None,
        }
    }
}

