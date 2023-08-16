# SensorUpdatePeriodSettingsReqV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**build** | Option<**String**> | The target build to apply to the policy | [optional]
**scheduler** | [**crate::models::PolicyPeriodSensorUpdateScheduler**](policy.SensorUpdateScheduler.md) |  | 
**show_early_adopter_builds** | Option<**bool**> | If true, early adopter builds will be visible on the sensor update policy page | [optional]
**uninstall_protection** | Option<**String**> | The uninstall protection state to apply to the policy | [optional]
**variants** | [**Vec<crate::models::SensorUpdatePeriodBuildReqV1>**](sensor_update.BuildReqV1.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


