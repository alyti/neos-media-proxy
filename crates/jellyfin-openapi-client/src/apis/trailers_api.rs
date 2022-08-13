/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.3
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_trailers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTrailersError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}


pub async fn get_trailers(configuration: &configuration::Configuration, user_id: Option<&str>, max_official_rating: Option<&str>, has_theme_song: Option<bool>, has_theme_video: Option<bool>, has_subtitles: Option<bool>, has_special_feature: Option<bool>, has_trailer: Option<bool>, adjacent_to: Option<&str>, parent_index_number: Option<i32>, has_parental_rating: Option<bool>, is_hd: Option<bool>, is4_k: Option<bool>, location_types: Option<Vec<crate::models::LocationType>>, exclude_location_types: Option<Vec<crate::models::LocationType>>, is_missing: Option<bool>, is_unaired: Option<bool>, min_community_rating: Option<f64>, min_critic_rating: Option<f64>, min_premiere_date: Option<String>, min_date_last_saved: Option<String>, min_date_last_saved_for_user: Option<String>, max_premiere_date: Option<String>, has_overview: Option<bool>, has_imdb_id: Option<bool>, has_tmdb_id: Option<bool>, has_tvdb_id: Option<bool>, is_movie: Option<bool>, is_series: Option<bool>, is_news: Option<bool>, is_kids: Option<bool>, is_sports: Option<bool>, exclude_item_ids: Option<Vec<String>>, start_index: Option<i32>, limit: Option<i32>, recursive: Option<bool>, search_term: Option<&str>, sort_order: Option<Vec<crate::models::SortOrder>>, parent_id: Option<&str>, fields: Option<Vec<crate::models::ItemFields>>, exclude_item_types: Option<Vec<crate::models::BaseItemKind>>, filters: Option<Vec<crate::models::ItemFilter>>, is_favorite: Option<bool>, media_types: Option<Vec<String>>, image_types: Option<Vec<crate::models::ImageType>>, sort_by: Option<Vec<String>>, is_played: Option<bool>, genres: Option<Vec<String>>, official_ratings: Option<Vec<String>>, tags: Option<Vec<String>>, years: Option<Vec<i32>>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<Vec<crate::models::ImageType>>, person: Option<&str>, person_ids: Option<Vec<String>>, person_types: Option<Vec<String>>, studios: Option<Vec<String>>, artists: Option<Vec<String>>, exclude_artist_ids: Option<Vec<String>>, artist_ids: Option<Vec<String>>, album_artist_ids: Option<Vec<String>>, contributing_artist_ids: Option<Vec<String>>, albums: Option<Vec<String>>, album_ids: Option<Vec<String>>, ids: Option<Vec<String>>, video_types: Option<Vec<crate::models::VideoType>>, min_official_rating: Option<&str>, is_locked: Option<bool>, is_place_holder: Option<bool>, has_official_rating: Option<bool>, collapse_box_set_items: Option<bool>, min_width: Option<i32>, min_height: Option<i32>, max_width: Option<i32>, max_height: Option<i32>, is3_d: Option<bool>, series_status: Option<Vec<crate::models::SeriesStatus>>, name_starts_with_or_greater: Option<&str>, name_starts_with: Option<&str>, name_less_than: Option<&str>, studio_ids: Option<Vec<String>>, genre_ids: Option<Vec<String>>, enable_total_record_count: Option<bool>, enable_images: Option<bool>) -> Result<crate::models::BaseItemDtoQueryResult, Error<GetTrailersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/Trailers", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_official_rating {
        local_var_req_builder = local_var_req_builder.query(&[("maxOfficialRating", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_theme_song {
        local_var_req_builder = local_var_req_builder.query(&[("hasThemeSong", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_theme_video {
        local_var_req_builder = local_var_req_builder.query(&[("hasThemeVideo", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_subtitles {
        local_var_req_builder = local_var_req_builder.query(&[("hasSubtitles", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_special_feature {
        local_var_req_builder = local_var_req_builder.query(&[("hasSpecialFeature", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_trailer {
        local_var_req_builder = local_var_req_builder.query(&[("hasTrailer", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = adjacent_to {
        local_var_req_builder = local_var_req_builder.query(&[("adjacentTo", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = parent_index_number {
        local_var_req_builder = local_var_req_builder.query(&[("parentIndexNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_parental_rating {
        local_var_req_builder = local_var_req_builder.query(&[("hasParentalRating", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_hd {
        local_var_req_builder = local_var_req_builder.query(&[("isHd", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is4_k {
        local_var_req_builder = local_var_req_builder.query(&[("is4K", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = location_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("locationTypes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("locationTypes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = exclude_location_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("excludeLocationTypes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("excludeLocationTypes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = is_missing {
        local_var_req_builder = local_var_req_builder.query(&[("isMissing", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_unaired {
        local_var_req_builder = local_var_req_builder.query(&[("isUnaired", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_community_rating {
        local_var_req_builder = local_var_req_builder.query(&[("minCommunityRating", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_critic_rating {
        local_var_req_builder = local_var_req_builder.query(&[("minCriticRating", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_premiere_date {
        local_var_req_builder = local_var_req_builder.query(&[("minPremiereDate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_date_last_saved {
        local_var_req_builder = local_var_req_builder.query(&[("minDateLastSaved", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_date_last_saved_for_user {
        local_var_req_builder = local_var_req_builder.query(&[("minDateLastSavedForUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_premiere_date {
        local_var_req_builder = local_var_req_builder.query(&[("maxPremiereDate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_overview {
        local_var_req_builder = local_var_req_builder.query(&[("hasOverview", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_imdb_id {
        local_var_req_builder = local_var_req_builder.query(&[("hasImdbId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_tmdb_id {
        local_var_req_builder = local_var_req_builder.query(&[("hasTmdbId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_tvdb_id {
        local_var_req_builder = local_var_req_builder.query(&[("hasTvdbId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_movie {
        local_var_req_builder = local_var_req_builder.query(&[("isMovie", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_series {
        local_var_req_builder = local_var_req_builder.query(&[("isSeries", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_news {
        local_var_req_builder = local_var_req_builder.query(&[("isNews", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_kids {
        local_var_req_builder = local_var_req_builder.query(&[("isKids", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_sports {
        local_var_req_builder = local_var_req_builder.query(&[("isSports", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_item_ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("excludeItemIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("excludeItemIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = start_index {
        local_var_req_builder = local_var_req_builder.query(&[("startIndex", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recursive {
        local_var_req_builder = local_var_req_builder.query(&[("recursive", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search_term {
        local_var_req_builder = local_var_req_builder.query(&[("searchTerm", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_order {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("sortOrder".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("sortOrder", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = parent_id {
        local_var_req_builder = local_var_req_builder.query(&[("parentId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = exclude_item_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("excludeItemTypes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("excludeItemTypes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = filters {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("filters".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("filters", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = is_favorite {
        local_var_req_builder = local_var_req_builder.query(&[("isFavorite", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = media_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("mediaTypes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("mediaTypes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = image_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("imageTypes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("imageTypes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = sort_by {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("sortBy".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("sortBy", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = is_played {
        local_var_req_builder = local_var_req_builder.query(&[("isPlayed", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = genres {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("genres".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("genres", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = official_ratings {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("officialRatings".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("officialRatings", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = tags {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("tags".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("tags", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = years {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("years".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("years", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = enable_user_data {
        local_var_req_builder = local_var_req_builder.query(&[("enableUserData", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = image_type_limit {
        local_var_req_builder = local_var_req_builder.query(&[("imageTypeLimit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_image_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("enableImageTypes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("enableImageTypes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = person {
        local_var_req_builder = local_var_req_builder.query(&[("person", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = person_ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("personIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("personIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = person_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("personTypes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("personTypes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = studios {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("studios".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("studios", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = artists {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("artists".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("artists", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = exclude_artist_ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("excludeArtistIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("excludeArtistIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = artist_ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("artistIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("artistIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = album_artist_ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("albumArtistIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("albumArtistIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = contributing_artist_ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("contributingArtistIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("contributingArtistIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = albums {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("albums".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("albums", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = album_ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("albumIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("albumIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("ids".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("ids", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = video_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("videoTypes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("videoTypes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = min_official_rating {
        local_var_req_builder = local_var_req_builder.query(&[("minOfficialRating", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_locked {
        local_var_req_builder = local_var_req_builder.query(&[("isLocked", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_place_holder {
        local_var_req_builder = local_var_req_builder.query(&[("isPlaceHolder", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_official_rating {
        local_var_req_builder = local_var_req_builder.query(&[("hasOfficialRating", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = collapse_box_set_items {
        local_var_req_builder = local_var_req_builder.query(&[("collapseBoxSetItems", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_width {
        local_var_req_builder = local_var_req_builder.query(&[("minWidth", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_height {
        local_var_req_builder = local_var_req_builder.query(&[("minHeight", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_width {
        local_var_req_builder = local_var_req_builder.query(&[("maxWidth", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_height {
        local_var_req_builder = local_var_req_builder.query(&[("maxHeight", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is3_d {
        local_var_req_builder = local_var_req_builder.query(&[("is3D", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = series_status {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("seriesStatus".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("seriesStatus", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = name_starts_with_or_greater {
        local_var_req_builder = local_var_req_builder.query(&[("nameStartsWithOrGreater", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name_starts_with {
        local_var_req_builder = local_var_req_builder.query(&[("nameStartsWith", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name_less_than {
        local_var_req_builder = local_var_req_builder.query(&[("nameLessThan", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = studio_ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("studioIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("studioIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = genre_ids {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("genreIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("genreIds", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = enable_total_record_count {
        local_var_req_builder = local_var_req_builder.query(&[("enableTotalRecordCount", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = enable_images {
        local_var_req_builder = local_var_req_builder.query(&[("enableImages", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTrailersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

