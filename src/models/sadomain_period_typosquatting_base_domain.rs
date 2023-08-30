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
pub struct SadomainPeriodTyposquattingBaseDomain {
    /// The date when the domain was registered
    #[serde(rename = "created_date", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// The ID of the domain
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the domain has a valid Whois record
    #[serde(rename = "is_registered")]
    pub is_registered: bool,
    /// The Punycode representation of the domain, i.e. starting with `xn--`
    #[serde(rename = "punycode_format")]
    pub punycode_format: String,
    /// The Unicode representation of the domain
    #[serde(rename = "unicode_format")]
    pub unicode_format: String,
    #[serde(rename = "whois", skip_serializing_if = "Option::is_none")]
    pub whois: Option<Box<crate::models::SadomainPeriodWhoisRecord>>,
}

impl SadomainPeriodTyposquattingBaseDomain {
    pub fn new(
        id: String,
        is_registered: bool,
        punycode_format: String,
        unicode_format: String,
    ) -> SadomainPeriodTyposquattingBaseDomain {
        SadomainPeriodTyposquattingBaseDomain {
            created_date: None,
            id,
            is_registered,
            punycode_format,
            unicode_format,
            whois: None,
        }
    }
}
