# PreventionPeriodSettingRespV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The human readable description of the setting | [optional]
**id** | **String** | The id of the setting | 
**name** | **String** | The name of the setting | 
**r#type** | **String** | The type of the setting which can be used as a hint when displaying in the UI | 
**value** | [**serde_json::Value**](.md) | The value for the setting. For a toggle this value will take the form {'enabled':true|false}. For an mlslider this will take the form {'detection':'DISABLED|CAUTIOUS|MODERATE|AGGRESSIVE|EXTRA_AGGRESSIVE','prevention':'DISABLED|CAUTIOUS|MODERATE|AGGRESSIVE|EXTRA_AGGRESSIVE'} | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


