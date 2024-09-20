# \ConfigurationAssessmentEvaluationLogicApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_evaluation_logic_mixin0**](ConfigurationAssessmentEvaluationLogicApi.md#get_evaluation_logic_mixin0) | **GET** /configuration-assessment/entities/evaluation-logic/v1 | Get details on evaluation logic items by providing one or more finding IDs.

## get_evaluation_logic_mixin0

> models::DomainPeriodApiEvaluationLogicEntitiesResponseV1 get_evaluation_logic_mixin0(ids)
Get details on evaluation logic items by providing one or more finding IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more evaluation logic finding IDs. | [required] |

### Return type

[**models::DomainPeriodApiEvaluationLogicEntitiesResponseV1**](domain.APIEvaluationLogicEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
