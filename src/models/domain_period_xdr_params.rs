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
pub struct DomainPeriodXdrParams {
    #[serde(rename = "assign_to")]
    pub assign_to: String,
    #[serde(rename = "assign_to_uuid")]
    pub assign_to_uuid: String,
    #[serde(rename = "comment")]
    pub comment: String,
    #[serde(rename = "execution_offset")]
    pub execution_offset: String,
    #[serde(rename = "severity")]
    pub severity: i32,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "tactic")]
    pub tactic: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "technique")]
    pub technique: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl DomainPeriodXdrParams {
    pub fn new(
        assign_to: String,
        assign_to_uuid: String,
        comment: String,
        execution_offset: String,
        severity: i32,
        status: String,
        tactic: String,
        tags: Vec<String>,
        technique: String,
        r#type: String,
    ) -> DomainPeriodXdrParams {
        DomainPeriodXdrParams {
            assign_to,
            assign_to_uuid,
            comment,
            execution_offset,
            severity,
            status,
            tactic,
            tags,
            technique,
            r#type,
        }
    }
}