/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2023-08-08T23:00:01Z
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EmpowerapiPeriodRemoteCommandPutFileV1 {
    #[serde(
        rename = "comments_for_audit_log",
        skip_serializing_if = "Option::is_none"
    )]
    pub comments_for_audit_log: Option<String>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "created_by_uuid", skip_serializing_if = "Option::is_none")]
    pub created_by_uuid: Option<String>,
    #[serde(rename = "created_timestamp", skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "file_type", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(rename = "modified_by_uuid", skip_serializing_if = "Option::is_none")]
    pub modified_by_uuid: Option<String>,
    #[serde(rename = "modified_timestamp", skip_serializing_if = "Option::is_none")]
    pub modified_timestamp: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "permission_type", skip_serializing_if = "Option::is_none")]
    pub permission_type: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Vec<String>>,
    #[serde(rename = "run_attempt_count")]
    pub run_attempt_count: i32,
    #[serde(rename = "run_success_count")]
    pub run_success_count: i32,
    #[serde(rename = "sha256", skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "write_access", skip_serializing_if = "Option::is_none")]
    pub write_access: Option<bool>,
}

impl EmpowerapiPeriodRemoteCommandPutFileV1 {
    pub fn new(
        run_attempt_count: i32,
        run_success_count: i32,
    ) -> EmpowerapiPeriodRemoteCommandPutFileV1 {
        EmpowerapiPeriodRemoteCommandPutFileV1 {
            comments_for_audit_log: None,
            content: None,
            created_by: None,
            created_by_uuid: None,
            created_timestamp: None,
            description: None,
            file_type: None,
            id: None,
            modified_by: None,
            modified_by_uuid: None,
            modified_timestamp: None,
            name: None,
            permission_type: None,
            platform: None,
            run_attempt_count,
            run_success_count,
            sha256: None,
            size: None,
            write_access: None,
        }
    }
}