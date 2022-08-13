/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IsoType : Enum IsoType.

/// Enum IsoType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsoType {
    #[serde(rename = "Dvd")]
    Dvd,
    #[serde(rename = "BluRay")]
    BluRay,

}

impl ToString for IsoType {
    fn to_string(&self) -> String {
        match self {
            Self::Dvd => String::from("Dvd"),
            Self::BluRay => String::from("BluRay"),
        }
    }
}

impl Default for IsoType {
    fn default() -> IsoType {
        Self::Dvd
    }
}




