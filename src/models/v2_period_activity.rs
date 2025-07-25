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
pub struct V2PeriodActivity {
    #[serde(rename = "class", skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(
        rename = "inline_configuration",
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_configuration: Option<Box<models::V2PeriodInlineConfig>>,
    /// Optional user provided name for the activity, if not specified a default of the name for that activity will be used.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<Vec<String>>,
    #[serde(rename = "properties")]
    pub properties: serde_json::Value,
    #[serde(rename = "version_constraint", skip_serializing_if = "Option::is_none")]
    pub version_constraint: Option<String>,
}

impl V2PeriodActivity {
    pub fn new(id: String, properties: serde_json::Value) -> V2PeriodActivity {
        V2PeriodActivity {
            class: None,
            id,
            inline_configuration: None,
            name: None,
            next: None,
            properties,
            version_constraint: None,
        }
    }
}
