# DomainPeriodMatchedBreachSummaryV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**community_name** | Option<**String**> | Community/colloquial exposed data event name. | [optional]
**confidence_level** | Option<**String**> | The level of confidence regarding data veridicality. Options for likely authentic, confirmed authentic (default: unverified). | [optional]
**credentials_domains** | Option<**Vec<String>**> |  | [optional]
**credentials_ips** | Option<**Vec<String>**> |  | [optional]
**description** | **String** | The description of the breach | 
**event_date** | Option<**String**> | The date the exposed data event occurred. | [optional]
**event_id** | Option<**String**> | CrowdStrike-generated unique exposed data event identifier. | [optional]
**exposure_date** | Option<**String**> | The date when the data was leaked online | [optional]
**fields** | **Vec<String>** | The set of fields which were breached: 'email', 'password', 'login_id', 'phone', etc. | 
**files** | Option<[**Vec<crate::models::DomainPeriodFileDetailsV1>**](domain.FileDetailsV1.md)> | Metadata regarding the file(s) where exposed data records where found. | [optional]
**idp_send_date** | Option<**String**> |  | [optional]
**idp_send_status** | Option<**String**> |  | [optional]
**name** | **String** | The name of the breach | 
**obtained_by** | Option<**String**> | Exposed Data Event Threat Actor/Group: Moniker(s) or real name(s) of the individual/group who unveiled confidential data. | [optional]
**url** | Option<**String**> | Where the leak was found. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


