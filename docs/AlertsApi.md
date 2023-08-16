# \AlertsApi

All URIs are relative to *https://api.crowdstrike.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_queries_alerts_v1**](AlertsApi.md#get_queries_alerts_v1) | **GET** /alerts/queries/alerts/v1 | retrieves all Alerts ids that match a given query
[**patch_entities_alerts_v2**](AlertsApi.md#patch_entities_alerts_v2) | **PATCH** /alerts/entities/alerts/v2 | Perform actions on detections identified by detection ID(s) in request. Each action has a name and a description which describes what the action does. If a request adds and removes tag in a single request, the order of processing would be to remove tags before adding new ones in.  
[**post_aggregates_alerts_v1**](AlertsApi.md#post_aggregates_alerts_v1) | **POST** /alerts/aggregates/alerts/v1 | retrieves aggregates for Alerts across all CIDs
[**post_entities_alerts_v1**](AlertsApi.md#post_entities_alerts_v1) | **POST** /alerts/entities/alerts/v1 | retrieves all Alerts given their ids



## get_queries_alerts_v1

> crate::models::MsaspecPeriodQueryResponse get_queries_alerts_v1(offset, limit, sort, filter, q)
retrieves all Alerts ids that match a given query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The first detection to return, where `0` is the latest detection. Use with the `offset` parameter to manage pagination of results. |  |
**limit** | Option<**i32**> | The maximum number of detections to return in this response (default: 100; max: 10000). Use with the `offset` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort detections in either `asc` (ascending) or `desc` (descending) order. For example: `status|asc` or `status|desc`. |  |
**filter** | Option<**String**> | Filter detections using a query in Falcon Query Language (FQL). An asterisk wildcard `*` includes all results.   The full list of valid filter options is extensive. Review it in our [documentation inside the Falcon console](https://falcon.crowdstrike.com/documentation/45/falcon-query-language-fql). |  |
**q** | Option<**String**> | Search all detection metadata for the provided string |  |

### Return type

[**crate::models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_entities_alerts_v2

> crate::models::MsaspecPeriodResponseFields patch_entities_alerts_v2(body)
Perform actions on detections identified by detection ID(s) in request. Each action has a name and a description which describes what the action does. If a request adds and removes tag in a single request, the order of processing would be to remove tags before adding new ones in.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DetectsapiPeriodPatchEntitiesAlertsV2Request**](DetectsapiPeriodPatchEntitiesAlertsV2Request.md) | request body takes a list of action parameter request that is applied against all \"ids\" provided | [required] |

### Return type

[**crate::models::MsaspecPeriodResponseFields**](msaspec.ResponseFields.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_aggregates_alerts_v1

> crate::models::ApiPeriodAggregatesResponse post_aggregates_alerts_v1(body)
retrieves aggregates for Alerts across all CIDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<crate::models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) | request body takes a list of aggregation query requests | [required] |

### Return type

[**crate::models::ApiPeriodAggregatesResponse**](api.aggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_entities_alerts_v1

> crate::models::DetectsapiPeriodPostEntitiesAlertsV1Response post_entities_alerts_v1(body)
retrieves all Alerts given their ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DetectsapiPeriodPostEntitiesAlertsV1Request**](DetectsapiPeriodPostEntitiesAlertsV1Request.md) |  | [required] |

### Return type

[**crate::models::DetectsapiPeriodPostEntitiesAlertsV1Response**](detectsapi.PostEntitiesAlertsV1Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

