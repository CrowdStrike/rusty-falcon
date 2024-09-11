# \ZeroTrustAssessmentApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_assessment_v1**](ZeroTrustAssessmentApi.md#get_assessment_v1) | **GET** /zero-trust-assessment/entities/assessments/v1 | Get Zero Trust Assessment data for one or more hosts by providing agent IDs (AID) and a customer ID (CID).
[**get_assessments_by_score_v1**](ZeroTrustAssessmentApi.md#get_assessments_by_score_v1) | **GET** /zero-trust-assessment/queries/assessments/v1 | Get Zero Trust Assessment data for one or more hosts by providing a customer ID (CID) and a range of scores.
[**get_audit_v1**](ZeroTrustAssessmentApi.md#get_audit_v1) | **GET** /zero-trust-assessment/entities/audit/v1 | Get the Zero Trust Assessment audit report for one customer ID (CID).

## get_assessment_v1

> models::DomainPeriodAssessmentsResponse get_assessment_v1(ids)
Get Zero Trust Assessment data for one or more hosts by providing agent IDs (AID) and a customer ID (CID).

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more agent IDs, which you can find in the data.zta file, or the Falcon console. | [required] |

### Return type

[**models::DomainPeriodAssessmentsResponse**](domain.AssessmentsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_assessments_by_score_v1

> models::DomainPeriodAssessmentsByScoreResponse get_assessments_by_score_v1(filter, limit, after, sort)
Get Zero Trust Assessment data for one or more hosts by providing a customer ID (CID) and a range of scores.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | FQL query specifying the filter score. | [required] |
**limit** | Option<**i32**> | The number of scores to return in this response (min: 1, max: 1000, default: 100). Use with the `after` parameter to manage pagination of results. |  |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**sort** | Option<**String**> | Sort accounts by their properties. A single sort field is allowed. Defaults to ascending. Supported sort option include:  <ul><li>score|desc</li><li>score|asc</li></ul> |  |[default to score]

### Return type

[**models::DomainPeriodAssessmentsByScoreResponse**](domain.AssessmentsByScoreResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_audit_v1

> models::DomainPeriodAuditResponse get_audit_v1()
Get the Zero Trust Assessment audit report for one customer ID (CID).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DomainPeriodAuditResponse**](domain.AuditResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
