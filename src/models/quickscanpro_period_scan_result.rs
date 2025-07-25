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
pub struct QuickscanproPeriodScanResult {
    #[serde(
        rename = "beta_intelligence_context",
        skip_serializing_if = "Option::is_none"
    )]
    pub beta_intelligence_context: Option<serde_json::Value>,
    #[serde(rename = "file_artifacts")]
    pub file_artifacts: Vec<models::QuickscanproPeriodFileResult>,
    #[serde(rename = "mitre_attacks", skip_serializing_if = "Option::is_none")]
    pub mitre_attacks: Option<Vec<models::DomainPeriodMitreAttack>>,
    #[serde(rename = "url_artifacts", skip_serializing_if = "Option::is_none")]
    pub url_artifacts: Option<Vec<models::QuickscanproPeriodUrlResult>>,
    #[serde(rename = "verdict")]
    pub verdict: Verdict,
    #[serde(rename = "verdict_reason")]
    pub verdict_reason: String,
    #[serde(rename = "verdict_source", skip_serializing_if = "Option::is_none")]
    pub verdict_source: Option<Vec<String>>,
}

impl QuickscanproPeriodScanResult {
    pub fn new(
        file_artifacts: Vec<models::QuickscanproPeriodFileResult>,
        verdict: Verdict,
        verdict_reason: String,
    ) -> QuickscanproPeriodScanResult {
        QuickscanproPeriodScanResult {
            beta_intelligence_context: None,
            file_artifacts,
            mitre_attacks: None,
            url_artifacts: None,
            verdict,
            verdict_reason,
            verdict_source: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Verdict {
    #[serde(rename = "clean")]
    Clean,
    #[serde(rename = "likely_benign")]
    LikelyBenign,
    #[serde(rename = "suspicious")]
    Suspicious,
    #[serde(rename = "malicious")]
    Malicious,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for Verdict {
    fn default() -> Verdict {
        Self::Clean
    }
}
