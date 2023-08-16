# SensorUpdatePeriodSettingsRespV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**build** | **String** | The target build applied to devices in the policy | 
**scheduler** | [**crate::models::PolicyPeriodSensorUpdateScheduler**](policy.SensorUpdateScheduler.md) |  | 
**sensor_version** | **String** |  | 
**show_early_adopter_builds** | **bool** | If true, early adopter builds will be visible on the sensor update policy page | 
**stage** | **String** | The release stage this build is in | 
**uninstall_protection** | **String** | The uninstall protection setting to apply to devices in the policy | 
**variants** | [**Vec<crate::models::SensorUpdatePeriodBuildRespV1>**](sensor_update.BuildRespV1.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


