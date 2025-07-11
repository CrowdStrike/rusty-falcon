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
pub struct DomainPeriodExternalAssetCertificate {
    #[serde(rename = "ciphers", skip_serializing_if = "Option::is_none")]
    pub ciphers: Option<Vec<serde_json::Value>>,
    #[serde(rename = "fingerprint_sha256", skip_serializing_if = "Option::is_none")]
    pub fingerprint_sha256: Option<String>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<Box<models::DomainPeriodExternalAssetCertificateEntityIdentifiers>>,
    #[serde(rename = "pubkey_bits", skip_serializing_if = "Option::is_none")]
    pub pubkey_bits: Option<i32>,
    #[serde(rename = "pubkey_type", skip_serializing_if = "Option::is_none")]
    pub pubkey_type: Option<String>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(
        rename = "signature_algorithm",
        skip_serializing_if = "Option::is_none"
    )]
    pub signature_algorithm: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<Box<models::DomainPeriodExternalAssetCertificateEntityIdentifiers>>,
    #[serde(rename = "valid_from", skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(rename = "valid_to", skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
}

impl DomainPeriodExternalAssetCertificate {
    pub fn new() -> DomainPeriodExternalAssetCertificate {
        DomainPeriodExternalAssetCertificate {
            ciphers: None,
            fingerprint_sha256: None,
            issuer: None,
            pubkey_bits: None,
            pubkey_type: None,
            serial: None,
            signature_algorithm: None,
            subject: None,
            valid_from: None,
            valid_to: None,
        }
    }
}
