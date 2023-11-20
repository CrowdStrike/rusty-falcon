# DomainPeriodAwsConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloudtrail_bucket_owner_id** | Option<**String**> | The 12 digit AWS account which is hosting the centralized S3 bucket containing cloudtrail logs for all accounts. | [optional]
**created_timestamp** | Option<**String**> | Timestamp of when the settings were first provisioned within CrowdStrike's system.' | [optional]
**last_modified_timestamp** | Option<**String**> | Timestamp of when the settings were last modified. | [optional]
**static_external_id** | Option<**String**> | By setting this value, all subsequent accounts that are provisioned will default to using this value as the external ID. | [optional]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
