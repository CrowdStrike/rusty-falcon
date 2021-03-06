/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainMatchedBreachSummaryV1 {
    /// The description of the breach
    #[serde(rename = "description")]
    pub description: String,
    /// The set of fields which were breached: 'email', 'password', 'login_id', 'phone', etc.
    #[serde(rename = "fields")]
    pub fields: Vec<String>,
    /// The name of the breach
    #[serde(rename = "name")]
    pub name: String,
}

impl DomainMatchedBreachSummaryV1 {
    pub fn new(description: String, fields: Vec<String>, name: String) -> DomainMatchedBreachSummaryV1 {
        DomainMatchedBreachSummaryV1 { description, fields, name }
    }
}
