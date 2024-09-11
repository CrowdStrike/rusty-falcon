# \SpotlightEvaluationLogicApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**combined_query_evaluation_logic**](SpotlightEvaluationLogicApi.md#combined_query_evaluation_logic) | **GET** /spotlight/combined/evaluation-logic/v1 | Search for evaluation logic in your environment by providing a FQL filter and paging details. Returns a set of evaluation logic entities which match the filter criteria.
[**get_evaluation_logic**](SpotlightEvaluationLogicApi.md#get_evaluation_logic) | **GET** /spotlight/entities/evaluation-logic/v1 | Get details on evaluation logic items by providing one or more IDs.
[**query_evaluation_logic**](SpotlightEvaluationLogicApi.md#query_evaluation_logic) | **GET** /spotlight/queries/evaluation-logic/v1 | Search for evaluation logic in your environment by providing a FQL filter and paging details. Returns a set of evaluation logic IDs which match the filter criteria.

## combined_query_evaluation_logic

> models::DomainPeriodSpapiEvaluationLogicCombinedResponseV1 combined_query_evaluation_logic(filter, after, limit, sort)
Search for evaluation logic in your environment by providing a FQL filter and paging details. Returns a set of evaluation logic entities which match the filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | FQL query specifying the filter parameters. | [required] |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | Maximum number of entities to return. |  |
**sort** | Option<**String**> | Sort evaluation logic by their properties. |  |

### Return type

[**models::DomainPeriodSpapiEvaluationLogicCombinedResponseV1**](domain.SPAPIEvaluationLogicCombinedResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_evaluation_logic

> models::DomainPeriodSpapiEvaluationLogicEntitiesResponseV1 get_evaluation_logic(ids)
Get details on evaluation logic items by providing one or more IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more evaluation logic IDs. | [required] |

### Return type

[**models::DomainPeriodSpapiEvaluationLogicEntitiesResponseV1**](domain.SPAPIEvaluationLogicEntitiesResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_evaluation_logic

> models::DomainPeriodSpapiQueryResponse query_evaluation_logic(filter, after, limit, sort)
Search for evaluation logic in your environment by providing a FQL filter and paging details. Returns a set of evaluation logic IDs which match the filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | FQL query specifying the filter parameters. | [required] |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | Maximum number of entities to return. |  |
**sort** | Option<**String**> | Sort evaluation logic by their properties. |  |

### Return type

[**models::DomainPeriodSpapiQueryResponse**](domain.SPAPIQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
