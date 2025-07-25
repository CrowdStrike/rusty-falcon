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
pub struct AlertB01d538f7601448fa7e8338bcd3a68c6 {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "affected_diff")]
    pub affected_diff: Vec<String>,
    #[serde(rename = "alert_type")]
    pub alert_type: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "integration")]
    pub integration: std::collections::HashMap<String, String>,
    #[serde(rename = "is_archived")]
    pub is_archived: bool,
    #[serde(rename = "new_affected_count")]
    pub new_affected_count: i32,
    #[serde(rename = "security_check_api_link")]
    pub security_check_api_link: String,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "source_id")]
    pub source_id: String,
    #[serde(rename = "threat_api_link")]
    pub threat_api_link: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "user_who_archived")]
    pub user_who_archived: String,
}

impl AlertB01d538f7601448fa7e8338bcd3a68c6 {
    pub fn new(
        account_id: String,
        affected_diff: Vec<String>,
        alert_type: String,
        description: String,
        id: String,
        integration: std::collections::HashMap<String, String>,
        is_archived: bool,
        new_affected_count: i32,
        security_check_api_link: String,
        source: String,
        source_id: String,
        threat_api_link: String,
        timestamp: String,
        user_who_archived: String,
    ) -> AlertB01d538f7601448fa7e8338bcd3a68c6 {
        AlertB01d538f7601448fa7e8338bcd3a68c6 {
            account_id,
            affected_diff,
            alert_type,
            description,
            id,
            integration,
            is_archived,
            new_affected_count,
            security_check_api_link,
            source,
            source_id,
            threat_api_link,
            timestamp,
            user_who_archived,
        }
    }
}
