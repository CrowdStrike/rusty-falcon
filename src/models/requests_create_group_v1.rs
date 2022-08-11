/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

/// RequestsCreateGroupV1 : A specific device group to create

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestsCreateGroupV1 {
    /// The FQL assignment rule for the group. This may only be specified if the groups type is 'dynamic'
    #[serde(rename = "assignment_rule", skip_serializing_if = "Option::is_none")]
    pub assignment_rule: Option<String>,
    /// The description of the group
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The type of device group to create
    #[serde(rename = "group_type")]
    pub group_type: GroupType,
    /// The name of the group
    #[serde(rename = "name")]
    pub name: String,
}

impl RequestsCreateGroupV1 {
    /// A specific device group to create
    pub fn new(group_type: GroupType, name: String) -> RequestsCreateGroupV1 {
        RequestsCreateGroupV1 {
            assignment_rule: None,
            description: None,
            group_type,
            name,
        }
    }
}

/// The type of device group to create
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupType {
    #[serde(rename = "static")]
    _Static,
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "staticByID")]
    StaticByID,
}

impl Default for GroupType {
    fn default() -> GroupType {
        Self::_Static
    }
}
