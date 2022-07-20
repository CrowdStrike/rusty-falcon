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
pub struct DomainAwsAccountV2 {
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "DeletedAt")]
    pub deleted_at: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    /// 12 digit AWS provided unique identifier for the account.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// AWS CloudTrail bucket name to store logs.
    #[serde(rename = "aws_cloudtrail_bucket_name", skip_serializing_if = "Option::is_none")]
    pub aws_cloudtrail_bucket_name: Option<String>,
    /// AWS CloudTrail region.
    #[serde(rename = "aws_cloudtrail_region", skip_serializing_if = "Option::is_none")]
    pub aws_cloudtrail_region: Option<String>,
    /// Permissions status returned via API.
    #[serde(rename = "aws_permissions_status")]
    pub aws_permissions_status: Vec<crate::models::DomainPermission>,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "cloudformation_url", skip_serializing_if = "Option::is_none")]
    pub cloudformation_url: Option<String>,
    #[serde(rename = "eventbus_name", skip_serializing_if = "Option::is_none")]
    pub eventbus_name: Option<String>,
    /// ID assigned for use with cross account IAM role access.
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The full arn of the IAM role created in this account to control access.
    #[serde(rename = "iam_role_arn", skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "intermediate_role_arn", skip_serializing_if = "Option::is_none")]
    pub intermediate_role_arn: Option<String>,
    #[serde(rename = "is_master")]
    pub is_master: bool,
    /// Up to 34 character AWS provided unique identifier for the organization.
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Account registration status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DomainAwsAccountV2 {
    pub fn new(created_at: String, deleted_at: String, id: i32, updated_at: String, aws_permissions_status: Vec<crate::models::DomainPermission>, is_master: bool) -> DomainAwsAccountV2 {
        DomainAwsAccountV2 {
            created_at,
            deleted_at,
            id,
            updated_at,
            account_id: None,
            aws_cloudtrail_bucket_name: None,
            aws_cloudtrail_region: None,
            aws_permissions_status,
            cid: None,
            cloudformation_url: None,
            eventbus_name: None,
            external_id: None,
            iam_role_arn: None,
            intermediate_role_arn: None,
            is_master,
            organization_id: None,
            status: None,
        }
    }
}
