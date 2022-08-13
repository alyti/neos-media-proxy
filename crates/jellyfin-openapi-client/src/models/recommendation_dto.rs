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
pub struct RecommendationDto {
    #[serde(rename = "Items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::BaseItemDto>>,
    #[serde(rename = "RecommendationType", skip_serializing_if = "Option::is_none")]
    pub recommendation_type: Option<Box<crate::models::RecommendationType>>,
    #[serde(rename = "BaselineItemName", skip_serializing_if = "Option::is_none")]
    pub baseline_item_name: Option<String>,
    #[serde(rename = "CategoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
}

impl RecommendationDto {
    pub fn new() -> RecommendationDto {
        RecommendationDto {
            items: None,
            recommendation_type: None,
            baseline_item_name: None,
            category_id: None,
        }
    }
}


