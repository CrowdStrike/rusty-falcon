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
pub struct RequestsCreateDeviceControlPolicyV1 {
    /// If specified the settings of the DeviceControl policy identified by the id will be used
    #[serde(rename = "clone_id", skip_serializing_if = "Option::is_none")]
    pub clone_id: Option<String>,
    /// The description to use when creating the policy
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name to use when creating the policy
    #[serde(rename = "name")]
    pub name: String,
    /// The name of the platform
    #[serde(rename = "platform_name")]
    pub platform_name: PlatformName,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<crate::models::RequestsDeviceControlPolicySettingsV1>>,
}

impl RequestsCreateDeviceControlPolicyV1 {
    pub fn new(name: String, platform_name: PlatformName) -> RequestsCreateDeviceControlPolicyV1 {
        RequestsCreateDeviceControlPolicyV1 {
            clone_id: None,
            description: None,
            name,
            platform_name,
            settings: None,
        }
    }
}

/// The name of the platform
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlatformName {
    #[serde(rename = "Windows")]
    Windows,
    #[serde(rename = "Mac")]
    Mac,
    #[serde(rename = "Linux")]
    Linux,
}

