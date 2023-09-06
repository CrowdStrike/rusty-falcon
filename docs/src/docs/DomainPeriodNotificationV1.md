# DomainPeriodNotificationV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assigned_to_uid** | Option<**String**> | The email of the user who is assigned to this notification | [optional]
**assigned_to_username** | Option<**String**> | The name of the user who is assigned to this notification | [optional]
**assigned_to_uuid** | Option<**String**> | The unique ID of the user who is assigned to this notification | [optional]
**breach_summary** | Option<[**crate::models::DomainPeriodMatchedBreachSummaryV1**](domain.MatchedBreachSummaryV1.md)> |  | [optional]
**cid** | **String** |  |
**created_date** | **String** | The date when the notification was generated |
**highlights** | Option<**Vec<String>**> | Highlighted content based on the rule that generated the notifications. Highlights are surrounded with a `<cs-highlight>` tag | [optional]
**id** | **String** | The ID of the notification |
**item_author** | Option<**String**> | The author who posted the intelligence item | [optional]
**item_author_id** | Option<**String**> | The ID of the author who posted the intelligence item | [optional]
**item_date** | **String** | Timestamp when the item is considered to have been created |
**item_id** | **String** | ID of the item which matched the rule |
**item_site** | Option<**String**> | The site where the intelligence item was found | [optional]
**item_site_id** | Option<**String**> | The ID of the site where the intelligence item was found | [optional]
**item_type** | **String** | Type of the item which matched the rule: `post`, `reply`, `botnet_config`, `breach`, etc. |
**logs** | Option<[**Vec<crate::models::SadomainPeriodNotificationLog>**](sadomain.NotificationLog.md)> |  | [optional]
**raw_intel_id** | **String** | ID of the raw intel item that matched the rule |
**rule_id** | **String** | The ID of the rule that generated this notification |
**rule_name** | **String** | The name of the rule that generated this notification |
**rule_priority** | **String** | The priority of the rule that generated this notification |
**rule_topic** | **String** | The topic of the rule that generated this notification |
**source_category** | Option<**String**> | Category of the source that generated the notification | [optional]
**status** | **String** | The notification status. This can be one of: `new`, `in-progress`, `closed-false-positive`, `closed-true-positive`. |
**typosquatting** | Option<[**crate::models::SadomainPeriodTyposquattingComponent**](sadomain.TyposquattingComponent.md)> |  | [optional]
**updated_date** | **String** | The date when the notification was updated |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
