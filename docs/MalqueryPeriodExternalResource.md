# MalqueryPeriodExternalResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**family** | Option<**String**> | Sample family | [optional]
**filesize** | Option<**i32**> | Sample size | [optional]
**filetype** | Option<**String**> | Sample file type | [optional]
**first_seen** | Option<**String**> | Date when it was first seen | [optional]
**ignore_reason** | Option<**String**> | Reason why the resource is ignored | [optional]
**label** | Option<**String**> | Sample label | [optional]
**label_confidence** | Option<**String**> | Resource label confidence | [optional]
**md5** | Option<**String**> | Sample MD5 | [optional]
**pattern** | Option<**String**> | Search pattern | [optional]
**pattern_type** | Option<**String**> | Search pattern type | [optional]
**samples** | [**Vec<crate::models::MalqueryPeriodSampleMetadata>**](malquery.SampleMetadata.md) | List of sample metadata | 
**sha1** | Option<**String**> | Sample SHA1 | [optional]
**sha256** | Option<**String**> | Sample SHA256 | [optional]
**tags** | Option<**Vec<String>**> | List of resource tags | [optional]
**yara_rule** | Option<**String**> | Search YARA rule | [optional]
**yara_rules** | Option<**Vec<String>**> | List of YARA rules for related files | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


