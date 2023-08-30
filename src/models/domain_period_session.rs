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
pub struct DomainPeriodSession {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "cloud_request_ids")]
    pub cloud_request_ids: Vec<String>,
    #[serde(rename = "commands")]
    pub commands: ::std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "commands_queued")]
    pub commands_queued: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "deleted_at")]
    pub deleted_at: String,
    #[serde(rename = "device_details")]
    pub device_details: Box<crate::models::DomainPeriodDevice>,
    #[serde(rename = "device_id")]
    pub device_id: String,
    #[serde(rename = "duration")]
    pub duration: f64,
    #[serde(rename = "hostname")]
    pub hostname: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "logs")]
    pub logs: Vec<crate::models::ModelPeriodSessionLog>,
    #[serde(rename = "offline_queued")]
    pub offline_queued: bool,
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "platform_id")]
    pub platform_id: i32,
    #[serde(rename = "platform_name")]
    pub platform_name: String,
    #[serde(rename = "pwd")]
    pub pwd: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "user_uuid")]
    pub user_uuid: String,
}

impl DomainPeriodSession {
    pub fn new(
        cid: String,
        cloud_request_ids: Vec<String>,
        commands: ::std::collections::HashMap<String, serde_json::Value>,
        commands_queued: bool,
        created_at: String,
        deleted_at: String,
        device_details: crate::models::DomainPeriodDevice,
        device_id: String,
        duration: f64,
        hostname: String,
        id: String,
        logs: Vec<crate::models::ModelPeriodSessionLog>,
        offline_queued: bool,
        origin: String,
        platform_id: i32,
        platform_name: String,
        pwd: String,
        updated_at: String,
        user_id: String,
        user_uuid: String,
    ) -> DomainPeriodSession {
        DomainPeriodSession {
            cid,
            cloud_request_ids,
            commands,
            commands_queued,
            created_at,
            deleted_at,
            device_details: Box::new(device_details),
            device_id,
            duration,
            hostname,
            id,
            logs,
            offline_queued,
            origin,
            platform_id,
            platform_name,
            pwd,
            updated_at,
            user_id,
            user_uuid,
        }
    }
}
