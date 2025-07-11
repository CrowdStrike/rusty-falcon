/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;

/// struct for typed errors of method [`cloud_registration_azure_create_registration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudRegistrationAzureCreateRegistrationError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status409(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_registration_azure_delete_registration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudRegistrationAzureDeleteRegistrationError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_registration_azure_download_script`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudRegistrationAzureDownloadScriptError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_registration_azure_get_registration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudRegistrationAzureGetRegistrationError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status404(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cloud_registration_azure_update_registration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloudRegistrationAzureUpdateRegistrationError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status404(models::MsaspecPeriodResponseFields),
    Status409(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`download_azure_script`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadAzureScriptError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

pub async fn cloud_registration_azure_create_registration(
    configuration: &configuration::Configuration,
    body: models::AzurePeriodAzureRegistrationCreateRequestExtV1,
) -> Result<
    models::AzurePeriodAzureRegistrationResponseExtV1,
    Error<CloudRegistrationAzureCreateRegistrationError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/cloud-security-registration-azure/entities/registrations/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudRegistrationAzureCreateRegistrationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cloud_registration_azure_delete_registration(
    configuration: &configuration::Configuration,
    tenant_ids: Vec<String>,
) -> Result<
    models::AzurePeriodDeleteRegistrationResponseExtV1,
    Error<CloudRegistrationAzureDeleteRegistrationError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/cloud-security-registration-azure/entities/registrations/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = match "multi" {
        "multi" => local_var_req_builder.query(
            &tenant_ids
                .into_iter()
                .map(|p| ("tenant_ids".to_owned(), p.to_string()))
                .collect::<Vec<(std::string::String, std::string::String)>>(),
        ),
        _ => local_var_req_builder.query(&[(
            "tenant_ids",
            &tenant_ids
                .into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        )]),
    };
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudRegistrationAzureDeleteRegistrationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cloud_registration_azure_download_script(
    configuration: &configuration::Configuration,
    body: models::AzurePeriodAzureDownloadScriptRequestV1,
) -> Result<
    models::AzurePeriodAzureProvisionGetAccountScriptResponseV1,
    Error<CloudRegistrationAzureDownloadScriptError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/cloud-security-registration-azure/entities/scripts/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudRegistrationAzureDownloadScriptError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cloud_registration_azure_get_registration(
    configuration: &configuration::Configuration,
    tenant_id: &str,
) -> Result<
    models::AzurePeriodAzureRegistrationResponseExtV1,
    Error<CloudRegistrationAzureGetRegistrationError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/cloud-security-registration-azure/entities/registrations/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("tenant_id", &tenant_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudRegistrationAzureGetRegistrationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cloud_registration_azure_update_registration(
    configuration: &configuration::Configuration,
    body: models::AzurePeriodAzureRegistrationUpdateRequestExtV1,
) -> Result<
    models::AzurePeriodAzureRegistrationResponseExtV1,
    Error<CloudRegistrationAzureUpdateRegistrationError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/cloud-security-registration-azure/entities/registrations/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloudRegistrationAzureUpdateRegistrationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn download_azure_script(
    configuration: &configuration::Configuration,
    tenant_id: &str,
) -> Result<(), Error<DownloadAzureScriptError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/cloud-security-registration-azure/entities/scripts/v1",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("tenant_id", &tenant_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DownloadAzureScriptError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
