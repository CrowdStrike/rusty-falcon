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
pub struct StatusBreakdown5b560b5c25194b5d9a78bb8a4f8f4408 {
    #[serde(rename = "cant_run")]
    pub cant_run: i32,
    #[serde(rename = "dismissed")]
    pub dismissed: i32,
    #[serde(rename = "failed")]
    pub failed: i32,
    #[serde(rename = "passed")]
    pub passed: i32,
    #[serde(rename = "pending")]
    pub pending: i32,
    #[serde(rename = "stale")]
    pub stale: i32,
}

impl StatusBreakdown5b560b5c25194b5d9a78bb8a4f8f4408 {
    pub fn new(
        cant_run: i32,
        dismissed: i32,
        failed: i32,
        passed: i32,
        pending: i32,
        stale: i32,
    ) -> StatusBreakdown5b560b5c25194b5d9a78bb8a4f8f4408 {
        StatusBreakdown5b560b5c25194b5d9a78bb8a4f8f4408 {
            cant_run,
            dismissed,
            failed,
            passed,
            pending,
            stale,
        }
    }
}
