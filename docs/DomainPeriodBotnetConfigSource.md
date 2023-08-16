# DomainPeriodBotnetConfigSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actions** | [**Vec<crate::models::DomainPeriodKeyValuePair>**](domain.KeyValuePair.md) | Populated for action botnets. List of action key value pairs, defined as the action and its type | 
**body** | **String** | Populated for spam botnets. The body of the spam template. If this contains replaceable variables, keep the variable names in the template as they appear in the raw config | 
**botnet** | **String** | The botnet's name | 
**config_type** | **String** | The type of botnet: `action`, `inject`, or `spam` | 
**injects** | [**Vec<crate::models::DomainPeriodBotnetInject>**](domain.BotnetInject.md) | Populated for inject botnets. List of inject targets and their data | 
**sub_botnet** | **String** | Optional sub-botnet differentiator | 
**subject** | **String** | Populated for spam botnets. The subject of the spam template. If this contains replaceable variables, keep the variable names in the template as they appear in the raw config | 
**variables** | [**Vec<crate::models::DomainPeriodKeyValuePair>**](domain.KeyValuePair.md) | Populated for spam botnets. List of key value pairs for any replaceable variables in the spam template | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


