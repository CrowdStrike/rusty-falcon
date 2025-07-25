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
pub struct DefinitionsPeriodDefinitionExt {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<std::collections::HashMap<String, models::V2PeriodActivity>>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<std::collections::HashMap<String, models::V2PeriodCondition>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates whether the workflow is enabled and active or not.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Unique identifier for the trigger.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Timestamp of when this version of the workflow was created.
    #[serde(rename = "last_modified_timestamp")]
    pub last_modified_timestamp: String,
    #[serde(rename = "loops", skip_serializing_if = "Option::is_none")]
    pub loops: Option<std::collections::HashMap<String, models::V2PeriodLoop>>,
    #[serde(rename = "multi_instance", skip_serializing_if = "Option::is_none")]
    pub multi_instance: Option<bool>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nodeRegistry")]
    pub node_registry: std::collections::HashMap<String, String>,
    #[serde(rename = "output_fields", skip_serializing_if = "Option::is_none")]
    pub output_fields: Option<Vec<String>>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<models::V2PeriodParameters>>,
    #[serde(rename = "parent")]
    pub parent: Box<models::V2PeriodModel>,
    #[serde(
        rename = "provision_on_install",
        skip_serializing_if = "Option::is_none"
    )]
    pub provision_on_install: Option<bool>,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "trigger")]
    pub trigger: Box<models::V2PeriodTrigger>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "uniqNodeSeen")]
    pub uniq_node_seen: std::collections::HashMap<String, bool>,
    #[serde(rename = "use_cases", skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<Vec<String>>,
    #[serde(rename = "vendors", skip_serializing_if = "Option::is_none")]
    pub vendors: Option<Vec<String>>,
    /// Version of the workflow. A given definition ID can have many versions. Each time an update is applied a new version is generated.
    #[serde(rename = "version")]
    pub version: i32,
}

impl DefinitionsPeriodDefinitionExt {
    pub fn new(
        enabled: bool,
        id: String,
        last_modified_timestamp: String,
        name: String,
        node_registry: std::collections::HashMap<String, String>,
        parent: models::V2PeriodModel,
        trigger: models::V2PeriodTrigger,
        uniq_node_seen: std::collections::HashMap<String, bool>,
        version: i32,
    ) -> DefinitionsPeriodDefinitionExt {
        DefinitionsPeriodDefinitionExt {
            actions: None,
            conditions: None,
            description: None,
            enabled,
            id,
            labels: None,
            last_modified_timestamp,
            loops: None,
            multi_instance: None,
            name,
            node_registry,
            output_fields: None,
            parameters: None,
            parent: Box::new(parent),
            provision_on_install: None,
            summary: None,
            trigger: Box::new(trigger),
            r#type: None,
            uniq_node_seen,
            use_cases: None,
            vendors: None,
            version,
        }
    }
}
