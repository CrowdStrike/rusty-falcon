# DeviceControlPeriodSettingsReqV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**classes** | [**Vec<crate::models::DeviceControlPeriodUsbClassExceptionsReqV1>**](device_control.USBClassExceptionsReqV1.md) | Settings that apply to a USB Class |
**custom_notifications** | Option<[**crate::models::DeviceControlPeriodUsbCustomNotifications**](device_control.USBCustomNotifications.md)> |  | [optional]
**delete_exceptions** | **Vec<String>** | An array of exception IDs to delete from the policy |
**end_user_notification** | **String** | Does the end user receives a notification when the policy is violated |
**enforcement_mode** | **String** | How is this policy enforced |
**enhanced_file_metadata** | Option<**bool**> | A bool value that enables file metadata functionality on the sensor or not | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
