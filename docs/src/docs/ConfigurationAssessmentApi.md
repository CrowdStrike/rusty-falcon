# \ConfigurationAssessmentApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_combined_assessments_query**](ConfigurationAssessmentApi.md#get_combined_assessments_query) | **GET** /configuration-assessment/combined/assessments/v1 | Search for assessments in your environment by providing an FQL filter and paging details. Returns a set of HostFinding entities which match the filter criteria
[**get_rule_details**](ConfigurationAssessmentApi.md#get_rule_details) | **GET** /configuration-assessment/entities/rule-details/v1 | Get rules details for provided one or more rule IDs

## get_combined_assessments_query

> models::DomainPeriodApiCombinedFindingsResponseV1 get_combined_assessments_query(filter, after, limit, sort, facet)
Search for assessments in your environment by providing an FQL filter and paging details. Returns a set of HostFinding entities which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | Filter items using a query in Falcon Query Language (FQL). Wildcards * are unsupported.   Common filter options include:  <ul><li>created_timestamp:>'2019-11-25T22:36:12Z'</li><li>updated_timestamp:>'2019-11-25T22:36:12Z'</li><li>aid:'8e7656b27d8c49a34a1af416424d6231'</li></ul> | [required] |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of items to return in this response (default: 100, max: 5000). Use with the after parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort assessment by their properties. Common sort options include:  <ul><li>created_timestamp|desc</li><li>updated_timestamp|asc</li></ul> |  |
**facet** | Option<[**Vec<String>**](String.md)> | Select various details blocks to be returned for each assessment entity. Supported values:  <ul><li>host</li><li>finding.rule</li><li>finding.evaluation_logic</li></ul> |  |

### Return type

[**models::DomainPeriodApiCombinedFindingsResponseV1**](domain.APICombinedFindingsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rule_details

> models::DomainPeriodApiRuleDetailsResponseV1 get_rule_details(ids)
Get rules details for provided one or more rule IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more rules IDs (max: 400) | [required] |

### Return type

[**models::DomainPeriodApiRuleDetailsResponseV1**](domain.APIRuleDetailsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
