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


/// struct for typed errors of method [`get_person`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPersonError {
    Status404(crate::models::ProblemDetails),
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_persons`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPersonsError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}


pub async fn get_person(configuration: &configuration::Configuration, name: &str, user_id: Option<&str>) -> Result<crate::models::BaseItemDto, Error<GetPersonError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/Persons/{name}", local_var_configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetPersonError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_persons(configuration: &configuration::Configuration, limit: Option<i32>, search_term: Option<&str>, fields: Option<Vec<crate::models::ItemFields>>, filters: Option<Vec<crate::models::ItemFilter>>, is_favorite: Option<bool>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<Vec<crate::models::ImageType>>, exclude_person_types: Option<Vec<String>>, person_types: Option<Vec<String>>, appears_in_item_id: Option<&str>, user_id: Option<&str>, enable_images: Option<bool>) -> Result<crate::models::BaseItemDtoQueryResult, Error<GetPersonsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/Persons", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search_term {
        local_var_req_builder = local_var_req_builder.query(&[("searchTerm", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("fields".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("fields", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
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
    if let Some(ref local_var_str) = exclude_person_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("excludePersonTypes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("excludePersonTypes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = person_types {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("personTypes".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("personTypes", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = appears_in_item_id {
        local_var_req_builder = local_var_req_builder.query(&[("appearsInItemId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_id {
        local_var_req_builder = local_var_req_builder.query(&[("userId", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetPersonsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

