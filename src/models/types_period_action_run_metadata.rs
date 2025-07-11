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
pub struct TypesPeriodActionRunMetadata {
    #[serde(rename = "collected_objects", skip_serializing_if = "Option::is_none")]
    pub collected_objects: Option<i32>,
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Box<models::TypesPeriodTimestamp>>,
    #[serde(
        rename = "integration_task_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub integration_task_id: Option<i64>,
    #[serde(
        rename = "integration_task_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub integration_task_name: Option<String>,
    #[serde(
        rename = "integration_task_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub integration_task_type: Option<Box<models::TypesPeriodIntegrationTaskType>>,
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Box<models::TypesPeriodTimestamp>>,
    #[serde(rename = "total_objects", skip_serializing_if = "Option::is_none")]
    pub total_objects: Option<i32>,
}

impl TypesPeriodActionRunMetadata {
    pub fn new() -> TypesPeriodActionRunMetadata {
        TypesPeriodActionRunMetadata {
            collected_objects: None,
            end_time: None,
            integration_task_id: None,
            integration_task_name: None,
            integration_task_type: None,
            start_time: None,
            total_objects: None,
        }
    }
}
