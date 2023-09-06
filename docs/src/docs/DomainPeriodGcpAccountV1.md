# DomainPeriodGcpAccountV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  |
**deleted_at** | **String** |  |
**id** | **i32** |  |
**updated_at** | **String** |  |
**cid** | **String** |  |
**cloud_scopes** | Option<[**Vec<crate::models::DomainPeriodCloudScope>**](domain.CloudScope.md)> |  | [optional]
**cspm_enabled** | **bool** |  |
**display_name** | Option<**String**> | GCP Display Name | [optional]
**environment** | Option<**String**> |  | [optional]
**folder_id** | Option<**String**> | GCP folder ID | [optional]
**folder_name** | Option<**String**> | GCP folder Name | [optional]
**gcp_permissions_status** | [**Vec<crate::models::DomainPeriodPermission>**](domain.Permission.md) | Permissions status returned via API. |
**organization_id** | Option<**String**> | GCP organization ID | [optional]
**organization_name** | Option<**String**> | GCP organization name | [optional]
**parent_id** | **String** | GCP Account ID for organization/folder/projects. |
**parent_type** | Option<**String**> | GCP Parent Type. | [optional]
**project_id** | Option<**String**> | GCP Project ID | [optional]
**service_account_client_email** | Option<**String**> |  | [optional]
**service_account_client_id** | Option<**String**> |  | [optional]
**service_account_id** | Option<**i32**> | GCP service account ID | [optional]
**service_account_private_key_id** | Option<**String**> |  | [optional]
**status** | Option<**String**> | Account registration status. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
