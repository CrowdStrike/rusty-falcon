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
pub struct RegistrationPeriodAwsAccountExtV2 {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(
        rename = "behavior_assessment_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub behavior_assessment_enabled: Option<bool>,
    #[serde(rename = "cloudtrail_region")]
    pub cloudtrail_region: String,
    #[serde(rename = "deployment_method", skip_serializing_if = "Option::is_none")]
    pub deployment_method: Option<String>,
    #[serde(rename = "dspm_enabled", skip_serializing_if = "Option::is_none")]
    pub dspm_enabled: Option<bool>,
    #[serde(rename = "dspm_role", skip_serializing_if = "Option::is_none")]
    pub dspm_role: Option<String>,
    #[serde(rename = "falcon_client_id", skip_serializing_if = "Option::is_none")]
    pub falcon_client_id: Option<String>,
    #[serde(rename = "iam_role_arn")]
    pub iam_role_arn: String,
    #[serde(rename = "is_master", skip_serializing_if = "Option::is_none")]
    pub is_master: Option<bool>,
    #[serde(rename = "organization_id")]
    pub organization_id: String,
    #[serde(rename = "root_stack_id", skip_serializing_if = "Option::is_none")]
    pub root_stack_id: Option<String>,
    #[serde(
        rename = "sensor_management_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub sensor_management_enabled: Option<bool>,
    #[serde(rename = "target_ous", skip_serializing_if = "Option::is_none")]
    pub target_ous: Option<Vec<String>>,
    #[serde(
        rename = "use_existing_cloudtrail",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_existing_cloudtrail: Option<bool>,
}

impl RegistrationPeriodAwsAccountExtV2 {
    pub fn new(
        account_id: String,
        cloudtrail_region: String,
        iam_role_arn: String,
        organization_id: String,
    ) -> RegistrationPeriodAwsAccountExtV2 {
        RegistrationPeriodAwsAccountExtV2 {
            account_id,
            account_type: None,
            behavior_assessment_enabled: None,
            cloudtrail_region,
            deployment_method: None,
            dspm_enabled: None,
            dspm_role: None,
            falcon_client_id: None,
            iam_role_arn,
            is_master: None,
            organization_id,
            root_stack_id: None,
            sensor_management_enabled: None,
            target_ous: None,
            use_existing_cloudtrail: None,
        }
    }
}
