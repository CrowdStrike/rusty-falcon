# RegistrationPeriodAzureAccountV1Ext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  | 
**deleted_at** | **String** |  | 
**id** | **i32** |  | 
**updated_at** | **String** |  | 
**account_type** | Option<**String**> |  | [optional]
**azure_permissions_status** | [**Vec<crate::models::DomainPeriodPermission>**](domain.Permission.md) | Permissions status returned via API. | 
**cid** | **String** |  | 
**client_id** | Option<**String**> |  | [optional]
**cloud_scopes** | Option<[**Vec<crate::models::DomainPeriodCloudScope>**](domain.CloudScope.md)> |  | [optional]
**conditions** | Option<[**Vec<crate::models::DomainPeriodCondition>**](domain.Condition.md)> |  | [optional]
**credentials_end_date** | Option<**String**> |  | [optional]
**credentials_type** | Option<**String**> |  | [optional]
**default_subscription_id** | Option<**String**> | Default Azure Subscription ID to provision shared IOA infrastructure. | [optional]
**environment** | Option<**String**> |  | [optional]
**object_id** | Option<**String**> |  | [optional]
**public_certificate** | Option<**String**> |  | [optional]
**public_certificate_raw** | Option<**String**> |  | [optional]
**role_assignments** | Option<[**Vec<crate::models::DomainPeriodAzureRoleAssignment>**](domain.AzureRoleAssignment.md)> |  | [optional]
**show_modal** | **bool** | Whether to show modal on the UI instructing existing D4C Azure customer to reregister subscriptions for CSPM. | 
**status** | Option<**String**> | Account registration status. | [optional]
**subscription_id** | Option<**String**> | Azure Subscription ID. | [optional]
**subscription_name** | Option<**String**> | Azure Subscription Name. | [optional]
**tenant_id** | Option<**String**> | Azure Tenant ID to use. | [optional]
**years_valid** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


