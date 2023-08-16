# DomainPeriodApiDetectionDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**adversary_ids** | Option<**Vec<i32>**> |  | [optional]
**assigned_to_name** | Option<**String**> |  | [optional]
**assigned_to_uid** | Option<**String**> |  | [optional]
**behaviors** | Option<[**Vec<crate::models::DetectsPeriodBehavior>**](detects.Behavior.md)> |  | [optional]
**behaviors_processed** | **Vec<String>** |  | 
**cid** | **String** |  | 
**created_timestamp** | **String** |  | 
**date_updated** | Option<**String**> |  | [optional]
**detection_id** | **String** |  | 
**device** | [**crate::models::DetectsPeriodDeviceDetailIndexed**](detects.DeviceDetailIndexed.md) |  | 
**email_sent** | **bool** |  | 
**first_behavior** | **String** |  | 
**hostinfo** | [**crate::models::DetectsPeriodHostInfo**](detects.HostInfo.md) |  | 
**last_behavior** | **String** |  | 
**max_confidence** | **i32** |  | 
**max_severity** | **i32** |  | 
**max_severity_displayname** | **String** |  | 
**overwatch_notes** | Option<**String**> |  | [optional]
**quarantined_files** | Option<[**Vec<crate::models::DetectsPeriodQuarantinedFile>**](detects.QuarantinedFile.md)> |  | [optional]
**seconds_to_resolved** | **i64** |  | 
**seconds_to_triaged** | **i64** |  | 
**show_in_ui** | **bool** |  | 
**status** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


