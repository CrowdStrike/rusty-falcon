/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiPeriodIndicatorCreateReqV1 {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "applied_globally")]
    pub applied_globally: bool,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(rename = "host_groups", skip_serializing_if = "Option::is_none")]
    pub host_groups: Option<Vec<String>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::ApiPeriodMetadataReqV1>>,
    #[serde(rename = "mobile_action", skip_serializing_if = "Option::is_none")]
    pub mobile_action: Option<String>,
    #[serde(rename = "platforms", skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiPeriodIndicatorCreateReqV1 {
    pub fn new(applied_globally: bool) -> ApiPeriodIndicatorCreateReqV1 {
        ApiPeriodIndicatorCreateReqV1 {
            action: None,
            applied_globally,
            description: None,
            expiration: None,
            host_groups: None,
            metadata: None,
            mobile_action: None,
            platforms: None,
            severity: None,
            source: None,
            tags: None,
            r#type: None,
            value: None,
        }
    }
}
