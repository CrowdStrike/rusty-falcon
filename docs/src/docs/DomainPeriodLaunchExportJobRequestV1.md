# DomainPeriodLaunchExportJobRequestV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity** | **String** | The entity type. This can be one of: [`notification-exposed-data-record`] |
**export_type** | **String** | The file type of the export. This can be one of: [`json`, `csv`] |
**filter** | **String** | FQL query to filter entities by. Possible filter properties depend on the entity type. |
**human_readable** | **bool** | If set to true (default), the field names in the exported file will resemble the table header in the UI (e.g. \"Hash type\"), otherwise the API level field names will be used (e.g. \"hash_type\") |
**sort** | Option<**String**> | Possible order by fields: created_timestamp, last_updated_timestamp. Ex: 'last_updated_timestamp|desc' | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
