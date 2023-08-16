# PreventionPeriodSettingReqV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The id of the setting to update | 
**value** | [**serde_json::Value**](.md) | The new value for the setting. For a toggle this value will take the form {'enabled':true|false}. For an mlslider this will take the form {'detection':'DISABLED|CAUTIOUS|MODERATE|AGGRESSIVE|EXTRA_AGGRESSIVE','prevention':'DISABLED|CAUTIOUS|MODERATE|AGGRESSIVE|EXTRA_AGGRESSIVE'} | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


