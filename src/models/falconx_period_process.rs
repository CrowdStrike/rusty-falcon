/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */
use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FalconxPeriodProcess {
    #[serde(rename = "amsi_calls", skip_serializing_if = "Option::is_none")]
    pub amsi_calls: Option<Vec<models::FalconxPeriodAmsiCall>>,
    #[serde(rename = "command_line", skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
    #[serde(rename = "file_accesses", skip_serializing_if = "Option::is_none")]
    pub file_accesses: Option<Vec<models::FalconxPeriodFileAccess>>,
    #[serde(rename = "handles", skip_serializing_if = "Option::is_none")]
    pub handles: Option<Vec<models::FalconxPeriodHandle>>,
    #[serde(rename = "icon_artifact_id", skip_serializing_if = "Option::is_none")]
    pub icon_artifact_id: Option<String>,
    #[serde(rename = "modules", skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<models::FalconxPeriodModule>>,
    #[serde(rename = "mutants", skip_serializing_if = "Option::is_none")]
    pub mutants: Option<Vec<String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "normalized_path", skip_serializing_if = "Option::is_none")]
    pub normalized_path: Option<String>,
    #[serde(rename = "parent_uid", skip_serializing_if = "Option::is_none")]
    pub parent_uid: Option<String>,
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    #[serde(rename = "process_flags", skip_serializing_if = "Option::is_none")]
    pub process_flags: Option<Vec<models::FalconxPeriodProcessFlag>>,
    #[serde(rename = "registry", skip_serializing_if = "Option::is_none")]
    pub registry: Option<Vec<models::FalconxPeriodRegistry>>,
    #[serde(rename = "script_calls", skip_serializing_if = "Option::is_none")]
    pub script_calls: Option<Vec<models::FalconxPeriodScriptCall>>,
    #[serde(rename = "sha256", skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(rename = "streams", skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<models::FalconxPeriodStream>>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl FalconxPeriodProcess {
    pub fn new() -> FalconxPeriodProcess {
        FalconxPeriodProcess {
            amsi_calls: None,
            command_line: None,
            file_accesses: None,
            handles: None,
            icon_artifact_id: None,
            modules: None,
            mutants: None,
            name: None,
            normalized_path: None,
            parent_uid: None,
            pid: None,
            process_flags: None,
            registry: None,
            script_calls: None,
            sha256: None,
            streams: None,
            uid: None,
        }
    }
}
