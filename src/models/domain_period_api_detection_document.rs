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
pub struct DomainPeriodApiDetectionDocument {
    #[serde(rename = "adversary_ids", skip_serializing_if = "Option::is_none")]
    pub adversary_ids: Option<Vec<i32>>,
    #[serde(rename = "assigned_to_name", skip_serializing_if = "Option::is_none")]
    pub assigned_to_name: Option<String>,
    #[serde(rename = "assigned_to_uid", skip_serializing_if = "Option::is_none")]
    pub assigned_to_uid: Option<String>,
    #[serde(rename = "behaviors", skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<models::DetectsPeriodBehavior>>,
    #[serde(rename = "behaviors_processed")]
    pub behaviors_processed: Vec<String>,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "created_timestamp")]
    pub created_timestamp: String,
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "detection_id")]
    pub detection_id: String,
    #[serde(rename = "device")]
    pub device: Box<models::DetectsPeriodDeviceDetailIndexed>,
    #[serde(rename = "email_sent")]
    pub email_sent: bool,
    #[serde(rename = "first_behavior")]
    pub first_behavior: String,
    #[serde(rename = "hostinfo")]
    pub hostinfo: Box<models::DetectsPeriodHostInfo>,
    #[serde(rename = "last_behavior")]
    pub last_behavior: String,
    #[serde(rename = "max_confidence")]
    pub max_confidence: i32,
    #[serde(rename = "max_severity")]
    pub max_severity: i32,
    #[serde(rename = "max_severity_displayname")]
    pub max_severity_displayname: String,
    #[serde(rename = "overwatch_notes", skip_serializing_if = "Option::is_none")]
    pub overwatch_notes: Option<String>,
    #[serde(rename = "quarantined_files", skip_serializing_if = "Option::is_none")]
    pub quarantined_files: Option<Vec<models::DetectsPeriodQuarantinedFile>>,
    #[serde(rename = "seconds_to_resolved")]
    pub seconds_to_resolved: i64,
    #[serde(rename = "seconds_to_triaged")]
    pub seconds_to_triaged: i64,
    #[serde(rename = "show_in_ui")]
    pub show_in_ui: bool,
    #[serde(rename = "status")]
    pub status: String,
}

impl DomainPeriodApiDetectionDocument {
    pub fn new(
        behaviors_processed: Vec<String>,
        cid: String,
        created_timestamp: String,
        detection_id: String,
        device: models::DetectsPeriodDeviceDetailIndexed,
        email_sent: bool,
        first_behavior: String,
        hostinfo: models::DetectsPeriodHostInfo,
        last_behavior: String,
        max_confidence: i32,
        max_severity: i32,
        max_severity_displayname: String,
        seconds_to_resolved: i64,
        seconds_to_triaged: i64,
        show_in_ui: bool,
        status: String,
    ) -> DomainPeriodApiDetectionDocument {
        DomainPeriodApiDetectionDocument {
            adversary_ids: None,
            assigned_to_name: None,
            assigned_to_uid: None,
            behaviors: None,
            behaviors_processed,
            cid,
            created_timestamp,
            date_updated: None,
            detection_id,
            device: Box::new(device),
            email_sent,
            first_behavior,
            hostinfo: Box::new(hostinfo),
            last_behavior,
            max_confidence,
            max_severity,
            max_severity_displayname,
            overwatch_notes: None,
            quarantined_files: None,
            seconds_to_resolved,
            seconds_to_triaged,
            show_in_ui,
            status,
        }
    }
}
