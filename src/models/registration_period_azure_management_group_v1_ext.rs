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
pub struct RegistrationPeriodAzureManagementGroupV1Ext {
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "DeletedAt")]
    pub deleted_at: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    /// Azure Management Group ID.
    #[serde(rename = "azure_management_group_id")]
    pub azure_management_group_id: String,
    /// Azure Management Group Name.
    #[serde(
        rename = "azure_management_group_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_management_group_name: Option<String>,
    /// Permissions status returned via API.
    #[serde(
        rename = "azure_permissions_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_permissions_status: Option<Vec<models::DomainPeriodPermission>>,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::StatemgmtPeriodCondition>>,
    #[serde(
        rename = "credentials_end_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub credentials_end_date: Option<String>,
    #[serde(rename = "credentials_type", skip_serializing_if = "Option::is_none")]
    pub credentials_type: Option<String>,
    /// Default Azure Subscription ID to provision shared IOA infrastructure.
    #[serde(
        rename = "default_subscription_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_subscription_id: Option<String>,
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "public_certificate", skip_serializing_if = "Option::is_none")]
    pub public_certificate: Option<String>,
    #[serde(
        rename = "public_certificate_raw",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_certificate_raw: Option<String>,
    #[serde(rename = "role_assignments", skip_serializing_if = "Option::is_none")]
    pub role_assignments: Option<Vec<models::AzurePeriodDbRoleAssignment>>,
    /// Account registration status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Azure Tenant ID to use.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    #[serde(rename = "years_valid", skip_serializing_if = "Option::is_none")]
    pub years_valid: Option<i32>,
}

impl RegistrationPeriodAzureManagementGroupV1Ext {
    pub fn new(
        created_at: String,
        deleted_at: String,
        id: i32,
        updated_at: String,
        azure_management_group_id: String,
        cid: String,
        tenant_id: String,
    ) -> RegistrationPeriodAzureManagementGroupV1Ext {
        RegistrationPeriodAzureManagementGroupV1Ext {
            created_at,
            deleted_at,
            id,
            updated_at,
            azure_management_group_id,
            azure_management_group_name: None,
            azure_permissions_status: None,
            cid,
            client_id: None,
            conditions: None,
            credentials_end_date: None,
            credentials_type: None,
            default_subscription_id: None,
            object_id: None,
            public_certificate: None,
            public_certificate_raw: None,
            role_assignments: None,
            status: None,
            tenant_id,
            years_valid: None,
        }
    }
}
