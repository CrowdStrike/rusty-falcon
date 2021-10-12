/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2021-10-05T19:33:53Z
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MalqueryRateLimitsMeta {
    /// Days left until the limits are refreshed
    #[serde(rename = "days_left")]
    pub days_left: i32,
    /// How many downloads were executed in the last month
    #[serde(rename = "download_count")]
    pub download_count: i32,
    /// Download counts per user
    #[serde(rename = "download_counts", skip_serializing_if = "Option::is_none")]
    pub download_counts: Option<Vec<crate::models::MalqueryUserRequestCount>>,
    /// Total download limit per month
    #[serde(rename = "download_limit")]
    pub download_limit: i32,
    /// How many hunts were executed in the last month
    #[serde(rename = "hunt_count")]
    pub hunt_count: i32,
    /// Hunt counts per user
    #[serde(rename = "hunt_counts", skip_serializing_if = "Option::is_none")]
    pub hunt_counts: Option<Vec<crate::models::MalqueryUserRequestCount>>,
    /// Total hunt limit per month
    #[serde(rename = "hunt_limit")]
    pub hunt_limit: i32,
    /// How many monitors were created in the last month
    #[serde(rename = "monitor_count")]
    pub monitor_count: i32,
    /// Total monitor limit per month
    #[serde(rename = "monitor_limit")]
    pub monitor_limit: i32,
    /// Time when the limits are refreshed. ISO 8601 format
    #[serde(rename = "refresh_time")]
    pub refresh_time: String,
}

impl MalqueryRateLimitsMeta {
    pub fn new(days_left: i32, download_count: i32, download_limit: i32, hunt_count: i32, hunt_limit: i32, monitor_count: i32, monitor_limit: i32, refresh_time: String) -> MalqueryRateLimitsMeta {
        MalqueryRateLimitsMeta {
            days_left,
            download_count,
            download_counts: None,
            download_limit,
            hunt_count,
            hunt_counts: None,
            hunt_limit,
            monitor_count,
            monitor_limit,
            refresh_time,
        }
    }
}


