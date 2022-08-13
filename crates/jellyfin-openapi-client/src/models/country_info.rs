/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CountryInfo : Class CountryInfo.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CountryInfo {
    /// Gets or sets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the display name.
    #[serde(rename = "DisplayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Gets or sets the name of the two letter ISO region.
    #[serde(rename = "TwoLetterISORegionName", skip_serializing_if = "Option::is_none")]
    pub two_letter_iso_region_name: Option<String>,
    /// Gets or sets the name of the three letter ISO region.
    #[serde(rename = "ThreeLetterISORegionName", skip_serializing_if = "Option::is_none")]
    pub three_letter_iso_region_name: Option<String>,
}

impl CountryInfo {
    /// Class CountryInfo.
    pub fn new() -> CountryInfo {
        CountryInfo {
            name: None,
            display_name: None,
            two_letter_iso_region_name: None,
            three_letter_iso_region_name: None,
        }
    }
}

