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
pub struct ItautomationPeriodScheduledTask {
    /// Username of the user who created the scheduled task. Example: john.doe@example.com
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Timestamp when the scheduled task was created. Example: 2023-05-01T10:00:00Z
    #[serde(rename = "created_time")]
    pub created_time: String,
    /// Whether to discover new hosts for the scheduled task. Example: true
    #[serde(rename = "discover_new_hosts", skip_serializing_if = "Option::is_none")]
    pub discover_new_hosts: Option<bool>,
    /// Whether to discover offline hosts for the scheduled task. Example: true
    #[serde(
        rename = "discover_offline_hosts",
        skip_serializing_if = "Option::is_none"
    )]
    pub discover_offline_hosts: Option<bool>,
    /// Whether to distribute the scheduled task. Example: true
    #[serde(rename = "distribute", skip_serializing_if = "Option::is_none")]
    pub distribute: Option<bool>,
    /// Additional arguments for the scheduled task
    #[serde(rename = "execution_args", skip_serializing_if = "Option::is_none")]
    pub execution_args: Option<std::collections::HashMap<String, String>>,
    ///  Duration for which the task stays active. Once expired, new and offline hosts won't be targeted. Example: 1m
    #[serde(
        rename = "expiration_interval",
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_interval: Option<String>,
    #[serde(rename = "guardrails", skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Box<models::FalconforitapiPeriodGuardrails>>,
    /// Unique identifier of the scheduled task. Example: st_123456789
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the scheduled task is active. Example: true
    #[serde(rename = "is_active")]
    pub is_active: bool,
    /// Whether this is a preset scheduled task. Example: true
    #[serde(rename = "is_preset")]
    pub is_preset: bool,
    /// Timestamp of the last execution of this scheduled task. Example: 2023-05-01T15:30:00Z
    #[serde(rename = "last_run", skip_serializing_if = "Option::is_none")]
    pub last_run: Option<String>,
    /// Username of the user who last modified the scheduled task. Example: jane.smith@example.com
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    /// Timestamp when the scheduled task was last modified. Example: 2023-05-02T14:30:00Z
    #[serde(rename = "modified_time", skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    #[serde(rename = "schedule")]
    pub schedule: Box<models::FalconforitapiPeriodSchedule>,
    /// Filter expression to select target hosts. Example: hostname:*prod*
    #[serde(rename = "target")]
    pub target: String,
    /// Unique identifier of the associated task. Example: f64b95555ef54ea682619ce880d267cc
    #[serde(rename = "task_id")]
    pub task_id: String,
    /// Name of the associated task. Example: Daily System Scan
    #[serde(rename = "task_name")]
    pub task_name: String,
    /// Type of the associated task. Example: scan
    #[serde(rename = "task_type")]
    pub task_type: String,
    /// Conditions that trigger remediation actions
    #[serde(rename = "trigger_condition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<Vec<models::FalconforitapiPeriodConditionGroup>>,
}

impl ItautomationPeriodScheduledTask {
    pub fn new(
        created_by: String,
        created_time: String,
        id: String,
        is_active: bool,
        is_preset: bool,
        schedule: models::FalconforitapiPeriodSchedule,
        target: String,
        task_id: String,
        task_name: String,
        task_type: String,
    ) -> ItautomationPeriodScheduledTask {
        ItautomationPeriodScheduledTask {
            created_by,
            created_time,
            discover_new_hosts: None,
            discover_offline_hosts: None,
            distribute: None,
            execution_args: None,
            expiration_interval: None,
            guardrails: None,
            id,
            is_active,
            is_preset,
            last_run: None,
            modified_by: None,
            modified_time: None,
            schedule: Box::new(schedule),
            target,
            task_id,
            task_name,
            task_type,
            trigger_condition: None,
        }
    }
}
