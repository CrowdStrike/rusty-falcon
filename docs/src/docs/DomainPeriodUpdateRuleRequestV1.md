# DomainPeriodUpdateRuleRequestV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**breach_monitoring_enabled** | **bool** | Whether to monitor for breach data. Available only for `Company Domains` and `Email addresses` rule topics. When enabled, ownership of the monitored domains or emails is required |
**filter** | **String** | The FQL filter to be used for searching |
**id** | **String** | The rule ID to be updated |
**name** | **String** | The name of a given rule |
**permissions** | **String** | The permissions for a given rule which specifies the rule's access by other users. Possible values: [`public`, `private`] |
**priority** | **String** | The priority for a given rule. Possible values: [`low`, `medium`, `high`] |
**substring_matching_enabled** | **bool** | Whether to monitor for substring matches. Only available for the `Typosquatting` topic. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
