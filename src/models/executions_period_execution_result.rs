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
pub struct ExecutionsPeriodExecutionResult {
    /// Details for the result of each activity node.
    #[serde(rename = "activities")]
    pub activities: Vec<models::ExecutionsPeriodActivityExecutionResult>,
    /// Populated when the execution origin is from a sub model.
    #[serde(rename = "ancestor_executions")]
    pub ancestor_executions: Vec<models::ExecutionsPeriodAncestorExecution>,
    /// Unique id of the workflow the execution is associated with.
    #[serde(rename = "definition_id")]
    pub definition_id: String,
    /// Version of the definition that executed.
    #[serde(rename = "definition_version")]
    pub definition_version: i32,
    /// Timestamp of when the execution completed. Only present when status is an end state.
    #[serde(rename = "end_timestamp", skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<String>,
    /// Unique id generated for the execution.
    #[serde(rename = "execution_id")]
    pub execution_id: String,
    /// Details for the results of each loop in the workflow definition.
    #[serde(rename = "loops")]
    pub loops: Vec<models::ExecutionsPeriodLoopResult>,
    /// A boolean value indicating whether the failed workflow execution is retryable
    #[serde(rename = "retryable")]
    pub retryable: bool,
    /// Timestamp of when the execution first started.
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: String,
    /// Overall status for the execution.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "trigger")]
    pub trigger: Box<models::ExecutionsPeriodTriggerResult>,
}

impl ExecutionsPeriodExecutionResult {
    pub fn new(
        activities: Vec<models::ExecutionsPeriodActivityExecutionResult>,
        ancestor_executions: Vec<models::ExecutionsPeriodAncestorExecution>,
        definition_id: String,
        definition_version: i32,
        execution_id: String,
        loops: Vec<models::ExecutionsPeriodLoopResult>,
        retryable: bool,
        start_timestamp: String,
        status: String,
        trigger: models::ExecutionsPeriodTriggerResult,
    ) -> ExecutionsPeriodExecutionResult {
        ExecutionsPeriodExecutionResult {
            activities,
            ancestor_executions,
            definition_id,
            definition_version,
            end_timestamp: None,
            execution_id,
            loops,
            retryable,
            start_timestamp,
            status,
            trigger: Box::new(trigger),
        }
    }
}