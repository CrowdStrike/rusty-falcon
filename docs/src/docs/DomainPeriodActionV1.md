# DomainPeriodActionV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cid** | **String** | The ID of the customer who created the action |
**content_format** | **String** | The level of detail in which the content will be delivered. It can be either 'standard' or 'enhanced' |
**created_timestamp** | **String** | The date when the action was created |
**frequency** | **String** | The time interval between the action's triggers. It can be one of the values: 'asap', 'daily' or 'weekly' |
**id** | **String** | The ID of the action |
**recipients** | **Vec<String>** | The address list who will be notified by this action. |
**rule_id** | **String** | The ID of the rule on which this action is attached |
**status** | **String** | The action status. It can be either 'enabled' or 'muted'. |
**trigger_matchless** | **bool** | Whether to periodically trigger the action based on the frequency, even when there are no new matches for the associated monitoring rule |
**r#type** | **String** | The action type. The only type currently supported is 'email' |
**updated_timestamp** | **String** | The date when the action was updated |
**user_uuid** | **String** | The UUID of the user who created the action |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
