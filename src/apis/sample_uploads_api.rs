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

/// struct for typed errors of method [`archive_delete_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchiveDeleteV1Error {
    Status400(),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`archive_get_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchiveGetV1Error {
    Status400(models::ClientPeriodArchiveCreateResponseV1),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::ClientPeriodArchiveCreateResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`archive_list_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchiveListV1Error {
    Status400(models::ClientPeriodArchiveListFilesResponseV1),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::ClientPeriodArchiveListFilesResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`archive_upload_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchiveUploadV1Error {
    Status400(models::ClientPeriodArchiveCreateResponseV1),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::ClientPeriodArchiveCreateResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`archive_upload_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchiveUploadV2Error {
    Status400(models::ClientPeriodArchiveCreateResponseV1),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::ClientPeriodArchiveCreateResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_sample_v3`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSampleV3Error {
    Status400(models::MsaPeriodQueryResponse),
    Status403(models::MsaPeriodQueryResponse),
    Status404(models::MsaPeriodQueryResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodQueryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`extraction_create_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtractionCreateV1Error {
    Status400(models::ClientPeriodExtractionCreateResponseV1),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::ClientPeriodExtractionCreateResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`extraction_get_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtractionGetV1Error {
    Status400(models::ClientPeriodExtractionCreateResponseV1),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::ClientPeriodExtractionCreateResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`extraction_list_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtractionListV1Error {
    Status400(models::ClientPeriodExtractionListFilesResponseV1),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::ClientPeriodExtractionListFilesResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sample_v3`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSampleV3Error {
    Status400(models::MsaPeriodReplyMetaOnly),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_sample_v3`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadSampleV3Error {
    Status400(models::ClientPeriodSampleMetadataResponseV2),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::ClientPeriodSampleMetadataResponseV2),
    UnknownValue(serde_json::Value),
}

pub async fn archive_delete_v1(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<(), Error<ArchiveDeleteV1Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!("{}/archives/entities/archives/v1", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    req_builder = req_builder.query(&[("id", &p_id.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ArchiveDeleteV1Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn archive_get_v1(
    configuration: &configuration::Configuration,
    id: &str,
    include_files: Option<bool>,
) -> Result<models::ClientPeriodArchiveCreateResponseV1, Error<ArchiveGetV1Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_include_files = include_files;

    let uri_str = format!("{}/archives/entities/archives/v1", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("id", &p_id.to_string())]);
    if let Some(ref param_value) = p_include_files {
        req_builder = req_builder.query(&[("include_files", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ClientPeriodArchiveCreateResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ClientPeriodArchiveCreateResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ArchiveGetV1Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn archive_list_v1(
    configuration: &configuration::Configuration,
    id: &str,
    limit: Option<i32>,
    offset: Option<&str>,
) -> Result<models::ClientPeriodArchiveListFilesResponseV1, Error<ArchiveListV1Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_limit = limit;
    let p_offset = offset;

    let uri_str = format!(
        "{}/archives/entities/archive-files/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("id", &p_id.to_string())]);
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ClientPeriodArchiveListFilesResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ClientPeriodArchiveListFilesResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ArchiveListV1Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn archive_upload_v1(
    configuration: &configuration::Configuration,
    name: &str,
    body: Vec<i32>,
    password: Option<&str>,
    is_confidential: Option<bool>,
    comment: Option<&str>,
) -> Result<models::ClientPeriodArchiveCreateResponseV1, Error<ArchiveUploadV1Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_body = body;
    let p_password = password;
    let p_is_confidential = is_confidential;
    let p_comment = comment;

    let uri_str = format!("{}/archives/entities/archives/v1", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    req_builder = req_builder.query(&[("name", &p_name.to_string())]);
    if let Some(ref param_value) = p_password {
        req_builder = req_builder.query(&[("password", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_is_confidential {
        req_builder = req_builder.query(&[("is_confidential", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_comment {
        req_builder = req_builder.query(&[("comment", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ClientPeriodArchiveCreateResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ClientPeriodArchiveCreateResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ArchiveUploadV1Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn archive_upload_v2(
    configuration: &configuration::Configuration,
    file: std::path::PathBuf,
    name: &str,
    password: Option<&str>,
    is_confidential: Option<bool>,
    comment: Option<&str>,
) -> Result<models::ClientPeriodArchiveCreateResponseV1, Error<ArchiveUploadV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let _p_file = file;
    let p_name = name;
    let p_password = password;
    let p_is_confidential = is_confidential;
    let p_comment = comment;

    let uri_str = format!("{}/archives/entities/archives/v2", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut multipart_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'file' parameter
    if let Some(param_value) = p_password {
        multipart_form = multipart_form.text("password", param_value.to_string());
    }
    multipart_form = multipart_form.text("name", p_name.to_string());
    if let Some(param_value) = p_is_confidential {
        multipart_form = multipart_form.text("is_confidential", param_value.to_string());
    }
    if let Some(param_value) = p_comment {
        multipart_form = multipart_form.text("comment", param_value.to_string());
    }
    req_builder = req_builder.multipart(multipart_form);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ClientPeriodArchiveCreateResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ClientPeriodArchiveCreateResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ArchiveUploadV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn delete_sample_v3(
    configuration: &configuration::Configuration,
    ids: &str,
) -> Result<models::MsaPeriodQueryResponse, Error<DeleteSampleV3Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!("{}/samples/entities/samples/v3", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    req_builder = req_builder.query(&[("ids", &p_ids.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MsaPeriodQueryResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MsaPeriodQueryResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteSampleV3Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn extraction_create_v1(
    configuration: &configuration::Configuration,
    body: models::ClientPeriodExtractionCreateRequestV1,
) -> Result<models::ClientPeriodExtractionCreateResponseV1, Error<ExtractionCreateV1Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!(
        "{}/archives/entities/extractions/v1",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ClientPeriodExtractionCreateResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ClientPeriodExtractionCreateResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ExtractionCreateV1Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn extraction_get_v1(
    configuration: &configuration::Configuration,
    id: &str,
    include_files: Option<bool>,
) -> Result<models::ClientPeriodExtractionCreateResponseV1, Error<ExtractionGetV1Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_include_files = include_files;

    let uri_str = format!(
        "{}/archives/entities/extractions/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("id", &p_id.to_string())]);
    if let Some(ref param_value) = p_include_files {
        req_builder = req_builder.query(&[("include_files", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ClientPeriodExtractionCreateResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ClientPeriodExtractionCreateResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ExtractionGetV1Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn extraction_list_v1(
    configuration: &configuration::Configuration,
    id: &str,
    limit: Option<i32>,
    offset: Option<&str>,
) -> Result<models::ClientPeriodExtractionListFilesResponseV1, Error<ExtractionListV1Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_limit = limit;
    let p_offset = offset;

    let uri_str = format!(
        "{}/archives/entities/extraction-files/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("id", &p_id.to_string())]);
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ClientPeriodExtractionListFilesResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ClientPeriodExtractionListFilesResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ExtractionListV1Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_sample_v3(
    configuration: &configuration::Configuration,
    ids: &str,
    password_protected: Option<bool>,
) -> Result<String, Error<GetSampleV3Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;
    let p_password_protected = password_protected;

    let uri_str = format!("{}/samples/entities/samples/v3", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("ids", &p_ids.to_string())]);
    if let Some(ref param_value) = p_password_protected {
        req_builder = req_builder.query(&[("password_protected", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSampleV3Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn upload_sample_v3(
    configuration: &configuration::Configuration,
    sample: std::path::PathBuf,
    file_name: &str,
    comment: Option<&str>,
    is_confidential: Option<bool>,
) -> Result<models::ClientPeriodSampleMetadataResponseV2, Error<UploadSampleV3Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let _p_sample = sample;
    let p_file_name = file_name;
    let p_comment = comment;
    let p_is_confidential = is_confidential;

    let uri_str = format!("{}/samples/entities/samples/v3", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut multipart_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'sample' parameter
    multipart_form = multipart_form.text("file_name", p_file_name.to_string());
    if let Some(param_value) = p_comment {
        multipart_form = multipart_form.text("comment", param_value.to_string());
    }
    if let Some(param_value) = p_is_confidential {
        multipart_form = multipart_form.text("is_confidential", param_value.to_string());
    }
    req_builder = req_builder.multipart(multipart_form);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ClientPeriodSampleMetadataResponseV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ClientPeriodSampleMetadataResponseV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UploadSampleV3Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
