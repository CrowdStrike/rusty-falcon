/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::de::Error as _;

/// struct for typed errors of method [`get_iot_hosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIotHostsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`query_iot_hosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryIotHostsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`query_iot_hosts_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryIotHostsV2Error {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

pub async fn get_iot_hosts(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
) -> Result<models::DomainPeriodDiscoverApiioTHostEntitiesResponse, Error<GetIotHostsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!("{}/discover/entities/iot-hosts/v1", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = match "multi" {
        "multi" => req_builder.query(
            &p_ids
                .into_iter()
                .map(|p| ("ids".to_owned(), p.to_string()))
                .collect::<Vec<(std::string::String, std::string::String)>>(),
        ),
        _ => req_builder.query(&[(
            "ids",
            &p_ids
                .into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        )]),
    };
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodDiscoverApiioTHostEntitiesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodDiscoverApiioTHostEntitiesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetIotHostsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn query_iot_hosts(
    configuration: &configuration::Configuration,
    offset: Option<i32>,
    limit: Option<i32>,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<models::MsaspecPeriodQueryResponse, Error<QueryIotHostsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_sort = sort;
    let p_filter = filter;

    let uri_str = format!("{}/discover/queries/iot-hosts/v1", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MsaspecPeriodQueryResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MsaspecPeriodQueryResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<QueryIotHostsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn query_iot_hosts_v2(
    configuration: &configuration::Configuration,
    after: Option<&str>,
    limit: Option<i32>,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<models::DomainPeriodDiscoverApiResponse, Error<QueryIotHostsV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_after = after;
    let p_limit = limit;
    let p_sort = sort;
    let p_filter = filter;

    let uri_str = format!("{}/discover/queries/iot-hosts/v2", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodDiscoverApiResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodDiscoverApiResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<QueryIotHostsV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
