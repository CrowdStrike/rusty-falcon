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

/// struct for typed errors of method [`aggregate_external_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateExternalAssetsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`blob_download_external_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlobDownloadExternalAssetsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`blob_preview_external_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlobPreviewExternalAssetsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`combined_ecosystem_subsidiaries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CombinedEcosystemSubsidiariesError {
    Status400(models::DomainPeriodFemEcosystemSubsidiariesResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodFemEcosystemSubsidiariesResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_external_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteExternalAssetsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ecosystem_subsidiaries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEcosystemSubsidiariesError {
    Status400(models::DomainPeriodFemEcosystemSubsidiariesResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodFemEcosystemSubsidiariesResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_external_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExternalAssetsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`patch_external_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchExternalAssetsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_external_assets_inventory_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostExternalAssetsInventoryV1Error {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`query_ecosystem_subsidiaries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryEcosystemSubsidiariesError {
    Status400(models::DomainPeriodFemEcosystemSubsidiariesResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodFemEcosystemSubsidiariesResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`query_external_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryExternalAssetsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`query_external_assets_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryExternalAssetsV2Error {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// Returns external assets aggregates as specified via JSON in request body.
pub async fn aggregate_external_assets(
    configuration: &configuration::Configuration,
    body: Vec<models::MsaPeriodAggregateQueryRequest>,
) -> Result<models::MsaPeriodAggregatesResponse, Error<AggregateExternalAssetsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!(
        "{}/fem/aggregates/external-assets/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MsaPeriodAggregatesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MsaPeriodAggregatesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AggregateExternalAssetsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Download the entire contents of the blob.
pub async fn blob_download_external_assets(
    configuration: &configuration::Configuration,
    asset_id: &str,
    hash: &str,
) -> Result<Vec<i32>, Error<BlobDownloadExternalAssetsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_asset_id = asset_id;
    let p_hash = hash;

    let uri_str = format!("{}/fem/entities/blobs-download/v1", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("assetId", &p_asset_id.to_string())]);
    req_builder = req_builder.query(&[("hash", &p_hash.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;i32&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;i32&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BlobDownloadExternalAssetsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Download a preview of the blob.
pub async fn blob_preview_external_assets(
    configuration: &configuration::Configuration,
    asset_id: &str,
    hash: &str,
) -> Result<models::DomainPeriodExternalAssetsBlobApiTypeV1, Error<BlobPreviewExternalAssetsError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_asset_id = asset_id;
    let p_hash = hash;

    let uri_str = format!("{}/fem/entities/blobs-preview/v1", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("assetId", &p_asset_id.to_string())]);
    req_builder = req_builder.query(&[("hash", &p_hash.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodExternalAssetsBlobApiTypeV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodExternalAssetsBlobApiTypeV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BlobPreviewExternalAssetsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn combined_ecosystem_subsidiaries(
    configuration: &configuration::Configuration,
    offset: Option<i32>,
    limit: Option<i32>,
    filter: Option<&str>,
    sort: Option<&str>,
    version_id: Option<&str>,
) -> Result<
    models::DomainPeriodFemEcosystemSubsidiariesEntitiesResponse,
    Error<CombinedEcosystemSubsidiariesError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_filter = filter;
    let p_sort = sort;
    let p_version_id = version_id;

    let uri_str = format!(
        "{}/fem/combined/ecosystem-subsidiaries/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_version_id {
        req_builder = req_builder.query(&[("version_id", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodFemEcosystemSubsidiariesEntitiesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodFemEcosystemSubsidiariesEntitiesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CombinedEcosystemSubsidiariesError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn delete_external_assets(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
    body: models::DomainPeriodExternalAssetApiDeleteRequestV1,
) -> Result<models::MsaspecPeriodQueryResponse, Error<DeleteExternalAssetsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;
    let p_body = body;

    let uri_str = format!(
        "{}/fem/entities/external-assets/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

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
    req_builder = req_builder.json(&p_body);

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
        let entity: Option<DeleteExternalAssetsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_ecosystem_subsidiaries(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
    version_id: Option<&str>,
) -> Result<
    models::DomainPeriodFemEcosystemSubsidiariesEntitiesResponse,
    Error<GetEcosystemSubsidiariesError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;
    let p_version_id = version_id;

    let uri_str = format!(
        "{}/fem/entities/ecosystem-subsidiaries/v1",
        configuration.base_path
    );
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
    if let Some(ref param_value) = p_version_id {
        req_builder = req_builder.query(&[("version_id", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodFemEcosystemSubsidiariesEntitiesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodFemEcosystemSubsidiariesEntitiesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEcosystemSubsidiariesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_external_assets(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
) -> Result<models::DomainPeriodExternalAssetsApiTypeV1, Error<GetExternalAssetsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!(
        "{}/fem/entities/external-assets/v1",
        configuration.base_path
    );
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodExternalAssetsApiTypeV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodExternalAssetsApiTypeV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetExternalAssetsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn patch_external_assets(
    configuration: &configuration::Configuration,
    body: models::DomainPeriodExternalAssetApiPatchRequestV1,
) -> Result<models::DomainPeriodExternalAssetsApiTypeV1, Error<PatchExternalAssetsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!(
        "{}/fem/entities/external-assets/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodExternalAssetsApiTypeV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodExternalAssetsApiTypeV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PatchExternalAssetsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn post_external_assets_inventory_v1(
    configuration: &configuration::Configuration,
    body: models::InventoryapiPeriodUserExternalAssetCreateRequestV1,
) -> Result<
    models::InventoryapiPeriodUserExternalAssetCreateResponseV1,
    Error<PostExternalAssetsInventoryV1Error>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!(
        "{}/fem/entities/external-asset-inventory/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::InventoryapiPeriodUserExternalAssetCreateResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::InventoryapiPeriodUserExternalAssetCreateResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostExternalAssetsInventoryV1Error> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn query_ecosystem_subsidiaries(
    configuration: &configuration::Configuration,
    offset: Option<i32>,
    limit: Option<i32>,
    filter: Option<&str>,
    sort: Option<&str>,
    version_id: Option<&str>,
) -> Result<
    models::DomainPeriodFemEcosystemSubsidiariesQueryResponse,
    Error<QueryEcosystemSubsidiariesError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_filter = filter;
    let p_sort = sort;
    let p_version_id = version_id;

    let uri_str = format!(
        "{}/fem/queries/ecosystem-subsidiaries/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_version_id {
        req_builder = req_builder.query(&[("version_id", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodFemEcosystemSubsidiariesQueryResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodFemEcosystemSubsidiariesQueryResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<QueryEcosystemSubsidiariesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn query_external_assets(
    configuration: &configuration::Configuration,
    offset: Option<&str>,
    limit: Option<i32>,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<models::MsaspecPeriodQueryResponse, Error<QueryExternalAssetsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_sort = sort;
    let p_filter = filter;

    let uri_str = format!("{}/fem/queries/external-assets/v1", configuration.base_path);
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
        let entity: Option<QueryExternalAssetsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn query_external_assets_v2(
    configuration: &configuration::Configuration,
    after: Option<&str>,
    limit: Option<i32>,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<models::DomainPeriodDiscoverApiResponse, Error<QueryExternalAssetsV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_after = after;
    let p_limit = limit;
    let p_sort = sort;
    let p_filter = filter;

    let uri_str = format!("{}/fem/queries/external-assets/v2", configuration.base_path);
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
        let entity: Option<QueryExternalAssetsV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
