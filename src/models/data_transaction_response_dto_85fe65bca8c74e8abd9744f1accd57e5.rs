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
pub struct DataTransactionResponseDto85fe65bca8c74e8abd9744f1accd57e5 {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "account_integration_id")]
    pub account_integration_id: String,
    #[serde(rename = "last_update", deserialize_with = "Option::deserialize")]
    pub last_update: Option<String>,
    #[serde(rename = "sources")]
    pub sources: Vec<models::Source187c0280ad3e4902becd3170cc483b36>,
    #[serde(rename = "status")]
    pub status: Status,
}

impl DataTransactionResponseDto85fe65bca8c74e8abd9744f1accd57e5 {
    pub fn new(
        account_id: String,
        account_integration_id: String,
        last_update: Option<String>,
        sources: Vec<models::Source187c0280ad3e4902becd3170cc483b36>,
        status: Status,
    ) -> DataTransactionResponseDto85fe65bca8c74e8abd9744f1accd57e5 {
        DataTransactionResponseDto85fe65bca8c74e8abd9744f1accd57e5 {
            account_id,
            account_integration_id,
            last_update,
            sources,
            status,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "pending")]
    Pending,
}

impl Default for Status {
    fn default() -> Status {
        Self::Cancelled
    }
}
