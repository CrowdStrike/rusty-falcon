# DomainPeriodEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | **String** | The raw body of the event |
**body_is_truncated** | **bool** | By default, event bodies are truncated to 64kb and bodyIsTruncated is set to True. For event bodies larger than 64kb, call the /events-full-body endpoint with the respective eventId |
**body_link** | Option<**String**> | Link to the event, can be missing | [optional]
**botnet_config_source** | Option<[**crate::models::DomainPeriodBotnetConfigSource**](domain.BotnetConfigSource.md)> |  | [optional]
**created_date** | **String** | The date the event was created (in UTC format) |
**ddos_attack_source** | Option<[**crate::models::DomainPeriodDdosAttackSource**](domain.DDOSAttackSource.md)> |  | [optional]
**event_type** | **String** | The type of event. One of `TweetEvent`, `CodePasteEvent`, `BotnetConfigEvent`, `DdosAttackEvent` |
**fingerprint** | **String** | The event's fingerprint |
**id** | **String** | The unique event ID |
**matched_rules** | Option<[**Vec<crate::models::DomainPeriodMatchedRule>**](domain.MatchedRule.md)> | List of objects with rules that matched the event | [optional]
**pastebin_text_source** | Option<[**crate::models::DomainPeriodPastebinTextSource**](domain.PastebinTextSource.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | A list of tags summarizing event content | [optional]
**tweet_source** | Option<[**crate::models::DomainPeriodTweetSource**](domain.TweetSource.md)> |  | [optional]
**updated_date** | **String** | The date the event was last updated (in UTC format) |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
