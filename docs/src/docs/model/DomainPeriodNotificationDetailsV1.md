# DomainPeriodNotificationDetailsV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author** | Option<**String**> | The raw intelligence item author username | [optional]
**content** | **String** | Highlighted content based on the rule that generated the notifications. Highlights are surrounded with a `<cs-highlight>` tag |
**created_date** | **String** | The date when the raw intelligence item was created |
**labels** | Option<**Vec<String>**> | The raw intelligence item labels. These contain hints around what is actually included in the item (malware, IPs, emails, etc). | [optional]
**language** | Option<**String**> | The raw intelligence item language | [optional]
**site** | Option<**String**> | The site where the raw intelligence item was found | [optional]
**title** | Option<**String**> | The raw intelligence item title | [optional]
**r#type** | **String** | The type of the raw intelligence item |
**updated_date** | **String** | The date when the raw intelligence item was updated |
**url** | Option<**String**> | The raw intelligence item URL | [optional]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
