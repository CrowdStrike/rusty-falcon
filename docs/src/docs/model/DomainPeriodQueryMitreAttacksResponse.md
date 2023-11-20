# DomainPeriodQueryMitreAttacksResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | [**Vec<crate::models::MsaspecPeriodError>**](msaspec.Error.md) | Array of API Errors |
**meta** | [**crate::models::MsaspecPeriodMetaInfo**](msaspec.MetaInfo.md) |  |
**resources** | **Vec<String>** | Actor's MITRE attack (Tactic and Technique) ids, represents a concatenation of actors slug, tactic id and technique id (optional) concatenated by underscore, example: fancy-bear_TA0011_T1071' |

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
