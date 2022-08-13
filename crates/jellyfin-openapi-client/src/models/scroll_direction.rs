/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScrollDirection : An enum representing the axis that should be scrolled.

/// An enum representing the axis that should be scrolled.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScrollDirection {
    #[serde(rename = "Horizontal")]
    Horizontal,
    #[serde(rename = "Vertical")]
    Vertical,

}

impl ToString for ScrollDirection {
    fn to_string(&self) -> String {
        match self {
            Self::Horizontal => String::from("Horizontal"),
            Self::Vertical => String::from("Vertical"),
        }
    }
}

impl Default for ScrollDirection {
    fn default() -> ScrollDirection {
        Self::Horizontal
    }
}



