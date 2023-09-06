# ChangesPeriodChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action_timestamp** | **String** |  |
**action_type** | **String** | Possible values: UNKNOWN, CREATE, WRITE, DELETE, SET, RENAME. |
**aid** | **String** |  |
**attributes** | Option<[**Vec<crate::models::ChangesPeriodAttribute>**](changes.Attribute.md)> |  | [optional]
**cid** | **String** |  |
**command_line** | **String** |  |
**diff** | Option<[**crate::models::ChangesPeriodDiff**](changes.Diff.md)> |  | [optional]
**entity_path** | **String** |  |
**entity_path_new** | Option<**String**> |  | [optional]
**entity_type** | **String** | Possible values: UNKNOWN, FILE, DIR, REGKEY,  REGVAL. |
**grandparent_process_image_file_name** | Option<**String**> |  | [optional]
**host** | Option<[**crate::models::ChangesPeriodHost**](changes.Host.md)> |  | [optional]
**id** | **String** |  |
**ingestion_timestamp** | **String** |  |
**is_from_different_mount_namespace** | Option<**bool**> |  | [optional]
**is_suppressed** | **bool** |  |
**oci_container_id** | Option<**String**> |  | [optional]
**parent_process_image_file_name** | Option<**String**> |  | [optional]
**permissions** | Option<[**crate::models::ChangesPeriodPermissions**](changes.Permissions.md)> |  | [optional]
**permissions_lin** | Option<[**crate::models::ChangesPeriodPermissionsLin**](changes.PermissionsLin.md)> |  | [optional]
**platform_name** | **String** |  |
**policy** | Option<[**crate::models::ChangesPeriodPolicy**](changes.Policy.md)> |  | [optional]
**prevalence** | Option<[**crate::models::ChangesPeriodPrevalence**](changes.Prevalence.md)> |  | [optional]
**process_id** | **String** |  |
**process_image_file_name** | **String** |  |
**real_user_id** | Option<**String**> |  | [optional]
**severity** | **String** | Possible values: UNKNOWN, LOW, MEDIUM, HIGH, CRITICAL |
**tags** | Option<[**Vec<crate::models::ChangesPeriodTag>**](changes.Tag.md)> |  | [optional]
**user_id** | **String** |  |
**user_name** | **String** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
