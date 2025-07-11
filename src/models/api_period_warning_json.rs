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
pub struct ApiPeriodWarningJson {
    /// Warnings are categorised to allow you to deal with a whole set of warnings the same way. Other values may be returned if cluster nodes are out of sync, using a newer or older version of LogScale.
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "classification")]
    pub classification: String,
    /// A stable message code that can be used to compare error types or look up error descriptions.
    #[serde(rename = "code")]
    pub code: String,
    /// A Human readable text describing the warning.
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiPeriodWarningJson {
    pub fn new(
        category: String,
        classification: String,
        code: String,
        message: String,
    ) -> ApiPeriodWarningJson {
        ApiPeriodWarningJson {
            category,
            classification,
            code,
            message,
        }
    }
}
