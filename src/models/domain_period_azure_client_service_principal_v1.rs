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
pub struct DomainPeriodAzureClientServicePrincipalV1 {
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(
        rename = "behavior_assessment_override",
        skip_serializing_if = "Option::is_none"
    )]
    pub behavior_assessment_override: Option<bool>,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::DomainPeriodCondition>>,
    /// If the account has CSPM enabled.
    #[serde(rename = "cspm_enabled")]
    pub cspm_enabled: bool,
    #[serde(
        rename = "default_subscription_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_subscription_id: Option<String>,
    #[serde(
        rename = "encrypted_private_key",
        skip_serializing_if = "Option::is_none"
    )]
    pub encrypted_private_key: Option<String>,
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "public_certificate", skip_serializing_if = "Option::is_none")]
    pub public_certificate: Option<String>,
    #[serde(
        rename = "resource_permissions",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_permissions: Option<Vec<crate::models::DomainPeriodAzureResourcePermission>>,
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    #[serde(rename = "x5t", skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
}

impl DomainPeriodAzureClientServicePrincipalV1 {
    pub fn new(
        cid: String,
        cspm_enabled: bool,
        tenant_id: String,
    ) -> DomainPeriodAzureClientServicePrincipalV1 {
        DomainPeriodAzureClientServicePrincipalV1 {
            account_type: None,
            behavior_assessment_override: None,
            cid,
            client_id: None,
            conditions: None,
            cspm_enabled,
            default_subscription_id: None,
            encrypted_private_key: None,
            object_id: None,
            public_certificate: None,
            resource_permissions: None,
            tenant_id,
            x5t: None,
        }
    }
}
