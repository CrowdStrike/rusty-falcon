# MalqueryPeriodExternalHuntOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter_filetypes** | Option<**Vec<String>**> | Limit results to files of certain types such as EMAIL, PCAP, PDF, PE32. Full list can be found in the documentation | [optional]
**filter_meta** | Option<**Vec<String>**> | Specify a subset of metadata fields to return in the results. Possible values: sha256, md5, type, size, first_seen, label, family | [optional]
**limit** | Option<**i32**> | Maximum number of results to be returned | [optional]
**max_date** | Option<**String**> | Limit results to files first seen before this date. The format is YYYY/MM/DD | [optional]
**max_size** | Option<**String**> | Maximum file size. The value can be specified either in bytes or in multiples of KB/MB/GB. Examples: 128000, 1.3 KB, 8mb | [optional]
**min_date** | Option<**String**> | Limit results to files first seen after this date. The format is YYYY/MM/DD | [optional]
**min_size** | Option<**String**> | Minimum file size. The value can be specified either in bytes or in multiples of KB/MB/GB. Examples: 128000, 1.3 KB, 8mb | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


