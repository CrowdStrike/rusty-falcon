/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2023-08-08T23:00:01Z
 *
 * Generated by: https://openapi-generator.tech
 */

/// PreventionPeriodSettingRespV1 : A prevention policy setting

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PreventionPeriodSettingRespV1 {
    /// The human readable description of the setting
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The id of the setting
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the setting
    #[serde(rename = "name")]
    pub name: String,
    /// The type of the setting which can be used as a hint when displaying in the UI
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The value for the setting. For a toggle this value will take the form {'enabled':true|false}. For an mlslider this will take the form {'detection':'DISABLED|CAUTIOUS|MODERATE|AGGRESSIVE|EXTRA_AGGRESSIVE','prevention':'DISABLED|CAUTIOUS|MODERATE|AGGRESSIVE|EXTRA_AGGRESSIVE'}
    #[serde(rename = "value")]
    pub value: serde_json::Value,
}

impl PreventionPeriodSettingRespV1 {
    /// A prevention policy setting
    pub fn new(
        id: String,
        name: String,
        r#type: RHashType,
        value: serde_json::Value,
    ) -> PreventionPeriodSettingRespV1 {
        PreventionPeriodSettingRespV1 {
            description: None,
            id,
            name,
            r#type,
            value,
        }
    }
}

/// The type of the setting which can be used as a hint when displaying in the UI
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "toggle")]
    Toggle,
    #[serde(rename = "mlslider")]
    Mlslider,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Toggle
    }
}