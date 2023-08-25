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
pub struct DetectsPeriodBehavior {
    #[serde(rename = "alleged_filetype")]
    pub alleged_filetype: String,
    #[serde(rename = "behavior_id")]
    pub behavior_id: String,
    #[serde(rename = "cmdline")]
    pub cmdline: String,
    #[serde(rename = "confidence")]
    pub confidence: i32,
    #[serde(rename = "container_id", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "control_graph_id")]
    pub control_graph_id: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "device_id")]
    pub device_id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "filepath")]
    pub filepath: String,
    #[serde(rename = "ioc_description")]
    pub ioc_description: String,
    #[serde(rename = "ioc_source")]
    pub ioc_source: String,
    #[serde(rename = "ioc_type")]
    pub ioc_type: String,
    #[serde(rename = "ioc_value")]
    pub ioc_value: String,
    #[serde(rename = "md5")]
    pub md5: String,
    #[serde(rename = "objective")]
    pub objective: String,
    #[serde(rename = "parent_details")]
    pub parent_details: Box<crate::models::DetectsPeriodParentDetails>,
    #[serde(rename = "pattern_disposition")]
    pub pattern_disposition: i32,
    #[serde(rename = "pattern_disposition_details")]
    pub pattern_disposition_details: Box<crate::models::PatterndispositionPeriodPatternDisposition>,
    #[serde(rename = "rule_instance_id", skip_serializing_if = "Option::is_none")]
    pub rule_instance_id: Option<String>,
    #[serde(
        rename = "rule_instance_version",
        skip_serializing_if = "Option::is_none"
    )]
    pub rule_instance_version: Option<i32>,
    #[serde(rename = "scenario")]
    pub scenario: String,
    #[serde(rename = "severity")]
    pub severity: i32,
    #[serde(rename = "sha256")]
    pub sha256: String,
    #[serde(rename = "tactic")]
    pub tactic: String,
    #[serde(rename = "tactic_id")]
    pub tactic_id: String,
    #[serde(rename = "technique")]
    pub technique: String,
    #[serde(rename = "technique_id")]
    pub technique_id: String,
    #[serde(
        rename = "template_instance_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub template_instance_id: Option<String>,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "triggering_process_graph_id")]
    pub triggering_process_graph_id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "user_name")]
    pub user_name: String,
}

impl DetectsPeriodBehavior {
    pub fn new(
        alleged_filetype: String,
        behavior_id: String,
        cmdline: String,
        confidence: i32,
        control_graph_id: String,
        description: String,
        device_id: String,
        display_name: String,
        filename: String,
        filepath: String,
        ioc_description: String,
        ioc_source: String,
        ioc_type: String,
        ioc_value: String,
        md5: String,
        objective: String,
        parent_details: crate::models::DetectsPeriodParentDetails,
        pattern_disposition: i32,
        pattern_disposition_details: crate::models::PatterndispositionPeriodPatternDisposition,
        scenario: String,
        severity: i32,
        sha256: String,
        tactic: String,
        tactic_id: String,
        technique: String,
        technique_id: String,
        timestamp: String,
        triggering_process_graph_id: String,
        user_id: String,
        user_name: String,
    ) -> DetectsPeriodBehavior {
        DetectsPeriodBehavior {
            alleged_filetype,
            behavior_id,
            cmdline,
            confidence,
            container_id: None,
            control_graph_id,
            description,
            device_id,
            display_name,
            filename,
            filepath,
            ioc_description,
            ioc_source,
            ioc_type,
            ioc_value,
            md5,
            objective,
            parent_details: Box::new(parent_details),
            pattern_disposition,
            pattern_disposition_details: Box::new(pattern_disposition_details),
            rule_instance_id: None,
            rule_instance_version: None,
            scenario,
            severity,
            sha256,
            tactic,
            tactic_id,
            technique,
            technique_id,
            template_instance_id: None,
            timestamp,
            triggering_process_graph_id,
            user_id,
            user_name,
        }
    }
}