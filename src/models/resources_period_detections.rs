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
pub struct ResourcesPeriodDetections {
    #[serde(rename = "compliant", skip_serializing_if = "Option::is_none")]
    pub compliant: Option<Box<models::ResourcesPeriodCompliance>>,
    #[serde(rename = "highest_severity", skip_serializing_if = "Option::is_none")]
    pub highest_severity: Option<String>,
    #[serde(rename = "ioa_counts")]
    pub ioa_counts: i32,
    #[serde(
        rename = "ioa_counts_by_severity",
        skip_serializing_if = "Option::is_none"
    )]
    pub ioa_counts_by_severity: Option<Box<models::ResourcesPeriodDetectionCount>>,
    #[serde(rename = "iom_counts")]
    pub iom_counts: i32,
    #[serde(
        rename = "iom_counts_by_severity",
        skip_serializing_if = "Option::is_none"
    )]
    pub iom_counts_by_severity: Option<Box<models::ResourcesPeriodDetectionCount>>,
    #[serde(rename = "non_compliant", skip_serializing_if = "Option::is_none")]
    pub non_compliant: Option<Box<models::ResourcesPeriodCompliance>>,
    #[serde(rename = "resource_url")]
    pub resource_url: String,
    #[serde(rename = "severities", skip_serializing_if = "Option::is_none")]
    pub severities: Option<Vec<String>>,
}

impl ResourcesPeriodDetections {
    pub fn new(
        ioa_counts: i32,
        iom_counts: i32,
        resource_url: String,
    ) -> ResourcesPeriodDetections {
        ResourcesPeriodDetections {
            compliant: None,
            highest_severity: None,
            ioa_counts,
            ioa_counts_by_severity: None,
            iom_counts,
            iom_counts_by_severity: None,
            non_compliant: None,
            resource_url,
            severities: None,
        }
    }
}
