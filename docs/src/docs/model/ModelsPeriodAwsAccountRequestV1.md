# ModelsPeriodAwsAccountRequestV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloudtrail_bucket_owner_id** | Option<**String**> | The 12 digit AWS account which is hosting the S3 bucket containing cloudtrail logs for this account. If this field is set, it takes precedence of the settings level field. | [optional]
**cloudtrail_bucket_region** | Option<**String**> | Region where the S3 bucket containing cloudtrail logs resides. | [optional]
**external_id** | Option<**String**> | ID assigned for use with cross account IAM role access. | [optional]
**iam_role_arn** | Option<**String**> | The full arn of the IAM role created in this account to control access. | [optional]
**id** | Option<**String**> | 12 digit AWS provided unique identifier for the account. | [optional]
**rate_limit_reqs** | Option<**i32**> | Rate limiting setting to control the maximum number of requests that can be made within the rate_limit_time threshold. | [optional]
**rate_limit_time** | Option<**i64**> | Rate limiting setting to control the number of seconds for which rate_limit_reqs applies. | [optional]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
