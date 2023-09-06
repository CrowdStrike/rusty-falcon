# MalqueryPeriodRequestMetaInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**options** | Option<[**crate::models::MalqueryPeriodExternalHuntOptions**](malquery.ExternalHuntOptions.md)> |  | [optional]
**pagination** | Option<[**crate::models::MsaspecPeriodPaging**](msaspec.Paging.md)> |  | [optional]
**patterns** | Option<[**Vec<crate::models::MalqueryPeriodSearchParameter>**](malquery.SearchParameter.md)> | Patterns to search for | [optional]
**powered_by** | Option<**String**> |  | [optional]
**query_time** | Option<**f64**> | Elapsed time since the request started in seconds | [optional]
**reqid** | Option<**String**> | Request ID returned after creating a hunt or exact search | [optional]
**reqtype** | Option<**String**> | Request type. Possible values: hunt, search | [optional]
**sample** | Option<**String**> | Sample ID | [optional]
**stats** | Option<[**crate::models::MalqueryPeriodStats**](malquery.Stats.md)> |  | [optional]
**status** | Option<**String**> | Request status. Possible values: inprogress, failed, done | [optional]
**trace_id** | **String** |  |
**writes** | Option<[**crate::models::MsaspecPeriodWrites**](msaspec.Writes.md)> |  | [optional]
**yara_rule** | Option<**String**> | YARA rule to be monitored | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
