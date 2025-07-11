/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2PeriodForLoop {
    #[serde(rename = "cel_condition", skip_serializing_if = "Option::is_none")]
    pub cel_condition: Option<String>,
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "condition_display", skip_serializing_if = "Option::is_none")]
    pub condition_display: Option<Vec<String>>,
    #[serde(rename = "continue_on_partial_execution")]
    pub continue_on_partial_execution: bool,
    #[serde(rename = "input")]
    pub input: String,
    #[serde(
        rename = "max_execution_seconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_execution_seconds: Option<i32>,
    #[serde(
        rename = "max_iteration_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_iteration_count: Option<i32>,
    #[serde(rename = "sequential", skip_serializing_if = "Option::is_none")]
    pub sequential: Option<bool>,
}

impl V2PeriodForLoop {
    pub fn new(continue_on_partial_execution: bool, input: String) -> V2PeriodForLoop {
        V2PeriodForLoop {
            cel_condition: None,
            condition: None,
            condition_display: None,
            continue_on_partial_execution,
            input,
            max_execution_seconds: None,
            max_iteration_count: None,
            sequential: None,
        }
    }
}
