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
pub struct ScheduledexclusionsPeriodCreateRequest {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "processes", skip_serializing_if = "Option::is_none")]
    pub processes: Option<String>,
    #[serde(rename = "repeated", skip_serializing_if = "Option::is_none")]
    pub repeated: Option<Box<models::ScheduledexclusionsPeriodRepeated>>,
    #[serde(rename = "schedule_end", skip_serializing_if = "Option::is_none")]
    pub schedule_end: Option<String>,
    #[serde(rename = "schedule_start", skip_serializing_if = "Option::is_none")]
    pub schedule_start: Option<String>,
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<String>,
}

impl ScheduledexclusionsPeriodCreateRequest {
    pub fn new(name: String, timezone: String) -> ScheduledexclusionsPeriodCreateRequest {
        ScheduledexclusionsPeriodCreateRequest {
            description: None,
            name,
            policy_id: None,
            processes: None,
            repeated: None,
            schedule_end: None,
            schedule_start: None,
            timezone,
            users: None,
        }
    }
}
