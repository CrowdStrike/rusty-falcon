# ModelsPeriodAwsAccountV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_health** | Option<[**crate::models::ModelsPeriodAwsAccountAccessHealth**](models.awsAccountAccessHealth.md)> |  | [optional]
**alias** | Option<**String**> | Alias/Name associated with the account. This is only updated once the account is in a registered state. | [optional]
**cid** | Option<**String**> |  | [optional]
**cloudformation_stack_id** | Option<**String**> | Unique identifier for the cloudformation stack id used for provisioning. | [optional]
**cloudformation_url** | Option<**String**> | URL of the CloudFormation template to execute. This is returned when mode is to set 'cloudformation' when provisioning. | [optional]
**cloudtrail_bucket_owner_id** | Option<**String**> | The 12 digit AWS account which is hosting the S3 bucket containing cloudtrail logs for this account. If this field is set, it takes precedence of the settings level field. | [optional]
**cloudtrail_bucket_region** | Option<**String**> | Region where the S3 bucket containing cloudtrail logs resides. This is only set if using cloudformation to provision and create the trail. | [optional]
**created_timestamp** | Option<**String**> | Timestamp of when the account was first provisioned within CrowdStrike's system.' | [optional]
**external_id** | Option<**String**> | ID assigned for use with cross account IAM role access. | [optional]
**iam_role_arn** | Option<**String**> | The full arn of the IAM role created in this account to control access. | [optional]
**id** | Option<**String**> | 12 digit AWS provided unique identifier for the account. | [optional]
**last_modified_timestamp** | Option<**String**> | Timestamp of when the account was last modified. | [optional]
**last_scanned_timestamp** | Option<**String**> | Timestamp of when the account was scanned. | [optional]
**policy_version** | Option<**String**> | Current version of permissions associated with IAM role and granted access. | [optional]
**provisioning_state** | Option<**String**> | Provisioning state of the account. Values can be; initiated, registered, unregistered. | [optional]
**rate_limit_reqs** | Option<**i32**> | Rate limiting setting to control the maximum number of requests that can be made within the rate_limit_time duration. | [optional]
**rate_limit_time** | Option<**i64**> | Rate limiting setting to control the number of seconds for which rate_limit_reqs applies. | [optional]
**template_version** | Option<**String**> | Current version of cloudformation template used to manage access. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


