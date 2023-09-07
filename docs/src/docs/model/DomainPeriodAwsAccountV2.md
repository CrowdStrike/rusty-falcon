# DomainPeriodAwsAccountV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  |
**deleted_at** | **String** |  |
**id** | **i32** |  |
**updated_at** | **String** |  |
**account_id** | Option<**String**> | 12 digit AWS provided unique identifier for the account. | [optional]
**account_name** | Option<**String**> | AWS account name | [optional]
**account_type** | Option<**String**> |  | [optional]
**active_regions** | Option<**Vec<String>**> |  | [optional]
**aws_cloudtrail_bucket_name** | Option<**String**> | AWS CloudTrail bucket name to store logs. | [optional]
**aws_cloudtrail_region** | Option<**String**> | AWS CloudTrail region. | [optional]
**aws_eventbus_arn** | Option<**String**> | AWS Eventbus ARN. | [optional]
**aws_permissions_status** | [**Vec<crate::models::DomainPeriodPermission>**](domain.Permission.md) | Permissions status returned via API. |
**behavior_assessment_enabled** | Option<**bool**> |  | [optional]
**cid** | Option<**String**> |  | [optional]
**cloud_scopes** | Option<[**Vec<crate::models::DomainPeriodCloudScope>**](domain.CloudScope.md)> |  | [optional]
**cloudformation_url** | Option<**String**> |  | [optional]
**conditions** | Option<[**Vec<crate::models::DomainPeriodCondition>**](domain.Condition.md)> |  | [optional]
**cspm_enabled** | Option<**bool**> |  | [optional]
**d4c** | Option<[**crate::models::DomainPeriodAwsd4CAccountV1**](domain.AWSD4CAccountV1.md)> |  | [optional]
**d4c_migrated** | Option<**bool**> |  | [optional]
**environment** | Option<**String**> |  | [optional]
**eventbus_name** | Option<**String**> |  | [optional]
**external_id** | Option<**String**> | ID assigned for use with cross account IAM role access. | [optional]
**iam_role_arn** | Option<**String**> | The full arn of the IAM role created in this account to control access. | [optional]
**intermediate_role_arn** | Option<**String**> |  | [optional]
**is_custom_rolename** | **bool** |  |
**is_master** | Option<**bool**> |  | [optional]
**organization_id** | Option<**String**> | Up to 34 character AWS provided unique identifier for the organization. | [optional]
**remediation_cloudformation_url** | Option<**String**> |  | [optional]
**remediation_region** | Option<**String**> |  | [optional]
**remediation_tou_accepted** | Option<**String**> |  | [optional]
**root_account_id** | Option<**String**> | 12 digit AWS provided unique identifier for the root account (of the organization this account belongs to). | [optional]
**root_iam_role** | Option<**bool**> |  | [optional]
**secondary_role_arn** | Option<**String**> |  | [optional]
**sensor_management_enabled** | **bool** |  |
**settings** | Option<[**serde_json::Value**](.md)> |  | [optional]
**status** | Option<**String**> | Account registration status. | [optional]
**use_existing_cloudtrail** | Option<**bool**> |  | [optional]
**valid** | Option<**bool**> |  | [optional]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
