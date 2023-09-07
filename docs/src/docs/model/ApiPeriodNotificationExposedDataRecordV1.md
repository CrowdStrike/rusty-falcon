# ApiPeriodNotificationExposedDataRecordV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author** | **String** | The individual or group who exposed the data |
**author_id** | Option<**String**> | The ID of the author within Recon | [optional]
**cid** | **String** | The customer ID |
**company** | Option<**String**> | The company of the user | [optional]
**created_date** | **String** | The date when this entity was created in Recon |
**credential_status** | Option<**String**> | The status set after deduplication. Possible values: 'newly_detected', 'previously_reported', 'other' | [optional]
**credentials_domain** | Option<**String**> | The domain where the credentials are valid | [optional]
**credentials_ip** | Option<**String**> | The IP where the credentials are valid | [optional]
**credentials_url** | Option<**String**> | The URL where the credentials are valid | [optional]
**display_name** | Option<**String**> | The nickname of the user on the impacted site | [optional]
**domain** | Option<**String**> | The domain of the email linked to the impacted site | [optional]
**email** | Option<**String**> | The email linked to the impacted site | [optional]
**event_date** | **String** | The approximate date when the event occurred |
**exposure_date** | **String** | The date when the exposed data was posted online |
**file** | Option<[**crate::models::ApiPeriodExposedDataFileDetailsV1**](api.ExposedDataFileDetailsV1.md)> |  | [optional]
**financial** | Option<[**crate::models::ApiPeriodExposedDataRecordFinancialV1**](api.ExposedDataRecordFinancialV1.md)> |  | [optional]
**full_name** | Option<**String**> | The full name of the user on the impacted site | [optional]
**hash_type** | Option<**String**> | The algorithm used to hash the password | [optional]
**id** | **String** | The ID of this entity |
**job_position** | Option<**String**> | The users job at the company | [optional]
**location** | Option<[**crate::models::ApiPeriodExposedDataRecordLocationV1**](api.ExposedDataRecordLocationV1.md)> |  | [optional]
**login_id** | Option<**String**> |  | [optional]
**notification_id** | **String** | The ID of the parent notification associated with this entity |
**password** | Option<**String**> | The password used for login | [optional]
**password_hash** | Option<**String**> | The password hash | [optional]
**password_salt** | Option<**String**> | The password salt | [optional]
**phone_number** | Option<**String**> | The phone number of the user on the impacted site | [optional]
**raw_intel_id** | **String** |  |
**rule** | [**crate::models::ApiPeriodRuleDetailsV1**](api.RuleDetailsV1.md) |  |
**site** | **String** | The source where this entity was found |
**site_id** | Option<**String**> | The ID of the site within Recon | [optional]
**social** | Option<[**crate::models::ApiPeriodExposedDataRecordSocialV1**](api.ExposedDataRecordSocialV1.md)> |  | [optional]
**source_category** | **String** | The category of the source where this entity was found |
**user_id** | Option<**String**> | The ID of the user on the impacted site | [optional]
**user_ip** | Option<**String**> | The IP of the user on the impacted site | [optional]
**user_uuid** | **String** |  |

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
