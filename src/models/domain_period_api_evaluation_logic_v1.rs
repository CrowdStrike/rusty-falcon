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
pub struct DomainPeriodApiEvaluationLogicV1 {
    #[serde(rename = "aid", skip_serializing_if = "Option::is_none")]
    pub aid: Option<String>,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(
        rename = "complex_check_operator",
        skip_serializing_if = "Option::is_none"
    )]
    pub complex_check_operator: Option<String>,
    #[serde(rename = "created_timestamp", skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "logic", skip_serializing_if = "Option::is_none")]
    pub logic: Option<Vec<crate::models::DomainPeriodApiEvaluationLogicItemV1>>,
    #[serde(rename = "updated_timestamp", skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
}

impl DomainPeriodApiEvaluationLogicV1 {
    pub fn new(id: String) -> DomainPeriodApiEvaluationLogicV1 {
        DomainPeriodApiEvaluationLogicV1 {
            aid: None,
            cid: None,
            complex_check_operator: None,
            created_timestamp: None,
            id,
            logic: None,
            updated_timestamp: None,
        }
    }
}
