# SadomainPeriodRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**breach_monitoring_enabled** | **bool** | Whether to monitor for breach data. Available only for `Company Domains` and `Email addresses` rule topics. When enabled, ownership of the monitored domains or emails is required |
**cid** | **String** |  |
**created_timestamp** | **String** | The creation time for a given rule |
**filter** | **String** | The FQL filter contained in a rule and used for searching. Parentheses may be added automatically for clarity |
**id** | **String** | The ID of a given rule |
**name** | **String** | The name of a given rule |
**ownership_assets** | Option<[**crate::models::SadomainPeriodCustomerAssets**](sadomain.CustomerAssets.md)> |  | [optional]
**permissions** | **String** | The permissions of a given rule |
**priority** | **String** | The priority of a given rule |
**status** | **String** | The status of a given rule |
**status_message** | Option<**String**> | The detailed status message of a given rule | [optional]
**substring_matching_enabled** | **bool** | Whether to monitor for substring matches. Only available for the `Typosquatting` rule topic |
**topic** | **String** | The topic of a given rule |
**updated_timestamp** | **String** | The last updated time for a given rule |
**user_id** | Option<**String**> | The user ID of the user that created a given rule | [optional]
**user_name** | Option<**String**> | The user name of the user that created a given rule | [optional]
**user_uuid** | **String** | The UUID of the user that created a given rule |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
