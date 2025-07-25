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
pub struct ExecutionsPeriodFlowExecutionResult {
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<Box<models::ExecutionsPeriodConditionResult>>,
    /// Timestamp of when the execution completed. Only present when status is an end state.
    #[serde(rename = "end_timestamp", skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<String>,
    /// When a node execution is in an error status this field is present and provides an error code that can be used to determine details why the failure occurred.
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// When a node execution is in an error status this field is present and provides a user friendly error message.
    #[serde(rename = "error_message", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Whether this node's result is mocked
    #[serde(rename = "mocked", skip_serializing_if = "Option::is_none")]
    pub mocked: Option<bool>,
    /// Unique id of the node as specified in the definition.
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// Timestamp of when the execution first started.
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: String,
    /// Current status of execution for the activity.
    #[serde(rename = "status")]
    pub status: String,
}

impl ExecutionsPeriodFlowExecutionResult {
    pub fn new(
        node_id: String,
        start_timestamp: String,
        status: String,
    ) -> ExecutionsPeriodFlowExecutionResult {
        ExecutionsPeriodFlowExecutionResult {
            condition: None,
            end_timestamp: None,
            error_code: None,
            error_message: None,
            mocked: None,
            node_id,
            start_timestamp,
            status,
        }
    }
}
