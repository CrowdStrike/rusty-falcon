# DomainPeriodCreateActionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_format** | **String** | The level of detail in which the content will be delivered. It can be either 'standard' or 'enhanced' |
**frequency** | **String** | The time interval between the action's triggers. It can be one of the values: 'asap', 'daily' or 'weekly' |
**recipients** | **Vec<String>** | The address list who will be notified by this action. |
**trigger_matchless** | **bool** | Whether to periodically trigger the action based on the frequency, even when there are no new matches for the associated monitoring rule |
**r#type** | **String** | The action type. The only type currently supported is 'email' |

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
