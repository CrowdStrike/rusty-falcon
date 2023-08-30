/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainPeriodScanHostMetadata {
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "completed_on", skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<String>,
    #[serde(rename = "filecount", skip_serializing_if = "Option::is_none")]
    pub filecount: Option<Box<crate::models::DomainPeriodFileCount>>,
    #[serde(rename = "host_id", skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[serde(rename = "host_scan_id", skip_serializing_if = "Option::is_none")]
    pub host_scan_id: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "profile_id", skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(
        rename = "scan_control_reason",
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_control_reason: Option<String>,
    #[serde(rename = "scan_id", skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    #[serde(rename = "started_on", skip_serializing_if = "Option::is_none")]
    pub started_on: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DomainPeriodScanHostMetadata {
    pub fn new(id: String) -> DomainPeriodScanHostMetadata {
        DomainPeriodScanHostMetadata {
            cid: None,
            completed_on: None,
            filecount: None,
            host_id: None,
            host_scan_id: None,
            id,
            last_updated: None,
            profile_id: None,
            scan_control_reason: None,
            scan_id: None,
            severity: None,
            started_on: None,
            status: None,
        }
    }
}
