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
pub struct SadomainPeriodTyposquattingComponent {
    #[serde(rename = "base_domain")]
    pub base_domain: Box<models::SadomainPeriodTyposquattingBaseDomain>,
    /// The ID of the infrastructure component
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "parent_domain")]
    pub parent_domain: Box<models::SadomainPeriodTyposquattingParentDomain>,
    /// The Punycode representation of the infrastructure component, i.e. starting with `xn--`
    #[serde(rename = "punycode_format")]
    pub punycode_format: String,
    #[serde(rename = "subdomain", skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Box<models::SadomainPeriodTyposquattingSubdomain>>,
    #[serde(
        rename = "submit_for_blocking_info",
        skip_serializing_if = "Option::is_none"
    )]
    pub submit_for_blocking_info: Option<Box<models::SadomainPeriodSubmissionInformation>>,
    #[serde(
        rename = "submit_for_takedown_info",
        skip_serializing_if = "Option::is_none"
    )]
    pub submit_for_takedown_info: Option<Box<models::SadomainPeriodSubmissionInformation>>,
    /// The Unicode representation of the infrastructure component
    #[serde(rename = "unicode_format")]
    pub unicode_format: String,
}

impl SadomainPeriodTyposquattingComponent {
    pub fn new(
        base_domain: models::SadomainPeriodTyposquattingBaseDomain,
        id: String,
        parent_domain: models::SadomainPeriodTyposquattingParentDomain,
        punycode_format: String,
        unicode_format: String,
    ) -> SadomainPeriodTyposquattingComponent {
        SadomainPeriodTyposquattingComponent {
            base_domain: Box::new(base_domain),
            id,
            parent_domain: Box::new(parent_domain),
            punycode_format,
            subdomain: None,
            submit_for_blocking_info: None,
            submit_for_takedown_info: None,
            unicode_format,
        }
    }
}
