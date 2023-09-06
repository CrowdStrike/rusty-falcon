# DomainPeriodBreachedItemV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company** | Option<**String**> |  | [optional]
**credential_status** | Option<**String**> | The status set after deduplication. Possible values: 'newly_detected', 'previously_reported', 'other' | [optional]
**credentials_domain** | Option<**String**> |  | [optional]
**credentials_ip** | Option<**String**> |  | [optional]
**credentials_url** | Option<**String**> |  | [optional]
**display_name** | Option<**String**> |  | [optional]
**domain** | **String** | The domain associated with the breached account. |
**email** | **String** | The email of the breached account. |
**financial** | Option<[**crate::models::DomainPeriodExposedDataRecordFinancialV1**](domain.ExposedDataRecordFinancialV1.md)> |  | [optional]
**hash_type** | **String** | The original hashing algorithm applied to the breached password. Possible values: 'plain', 'unknown', 'base64', 'md5', 'sha1', 'bcrypt', etc. The value 'plain' means that the password was originally found as plaintext. |
**job_position** | Option<**String**> |  | [optional]
**location** | Option<[**crate::models::DomainPeriodExposedDataRecordLocationV1**](domain.ExposedDataRecordLocationV1.md)> |  | [optional]
**login_id** | **String** | The username of the breached account. |
**name** | **String** | The name of the person associated with the breached account. |
**password** | **String** | The breached password. Passwords are returned as salted hashes, generated using the SHA256 algorithm and the CID as the salt. |
**password_hash** | Option<**String**> |  | [optional]
**password_salt** | Option<**String**> |  | [optional]
**phone** | **String** | The phone number of the person associated with the breached account. |
**social** | Option<[**crate::models::DomainPeriodExposedDataRecordSocialV1**](domain.ExposedDataRecordSocialV1.md)> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**user_ip** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
