# RegistrationPeriodIomEventV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **String** |  | 
**account_name** | **String** |  | 
**agent_id** | Option<**String**> |  | [optional]
**azure_tenant_id** | Option<**String**> |  | [optional]
**cid** | **String** |  | 
**cloud_labels** | Option<[**Vec<crate::models::ClassificationPeriodLabel>**](classification.Label.md)> |  | [optional]
**cloud_provider** | **String** |  | 
**cloud_scopes** | Option<[**Vec<crate::models::DomainPeriodCloudScope>**](domain.CloudScope.md)> |  | [optional]
**custom_policy_id** | Option<**i32**> |  | [optional]
**finding** | [**serde_json::Value**](.md) |  | 
**id** | **String** |  | 
**is_managed** | Option<**bool**> |  | [optional]
**policy_id** | Option<**i32**> |  | [optional]
**policy_statement** | **String** |  | 
**policy_type** | Option<**String**> |  | [optional]
**region** | **String** |  | 
**report_date_time** | **String** |  | 
**resource_attributes** | [**serde_json::Value**](.md) |  | 
**resource_create_time** | **String** |  | 
**resource_id** | **String** |  | 
**resource_id_type** | **String** |  | 
**resource_url** | **String** |  | 
**resource_uuid** | **String** |  | 
**scan_id** | Option<**String**> |  | [optional]
**scan_time** | **String** |  | 
**service** | **String** |  | 
**severity** | **String** |  | 
**status** | **String** |  | 
**tags** | **::std::collections::HashMap<String, String>** |  | 
**vm_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


