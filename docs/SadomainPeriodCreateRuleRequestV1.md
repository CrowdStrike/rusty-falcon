# SadomainPeriodCreateRuleRequestV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**breach_monitoring_enabled** | **bool** | Whether to monitor for breach data. Available only for `Company Domains` and `Email addresses` rule topics. When enabled, ownership of the monitored domains or emails is required | 
**filter** | **String** | The FQL filter to be used for searching | 
**name** | **String** | The name of a given rule | 
**permissions** | **String** | The permissions for a given rule which specifies the rule's access by other users. Possible values: [`public`, `private`] | 
**priority** | **String** | The priority for a given rule. Possible values: [`low`, `medium`, `high`] | 
**substring_matching_enabled** | **bool** | Whether to monitor for substring matches. Only available for the `Typosquatting` topic. | 
**topic** | **String** | The topic of a given rule. Possible values: [`SA_BRAND_PRODUCT`, `SA_VIP`, `SA_THIRD_PARTY`, `SA_IP`, `SA_CVE`, `SA_BIN`, `SA_DOMAIN`, `SA_EMAIL`, `SA_ALIAS`, `SA_AUTHOR`, `SA_CUSTOM`, `SA_TYPOSQUATTING`] | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


