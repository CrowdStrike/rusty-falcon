# \DetectsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_aggregate_detects**](DetectsApi.md#get_aggregate_detects) | **POST** /detects/aggregates/detects/GET/v1 | Get detect aggregates as specified via json in request body.
[**get_detect_summaries**](DetectsApi.md#get_detect_summaries) | **POST** /detects/entities/summaries/GET/v1 | View information about detections
[**query_detects**](DetectsApi.md#query_detects) | **GET** /detects/queries/detects/v1 | Search for detection IDs that match a given query
[**update_detects_by_ids_v2**](DetectsApi.md#update_detects_by_ids_v2) | **PATCH** /detects/entities/detects/v2 | Modify the state, assignee, and visibility of detections

## get_aggregate_detects

> models::MsaPeriodAggregatesResponse get_aggregate_detects(body)
Get detect aggregates as specified via json in request body.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) | Query criteria and settings | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_detect_summaries

> models::DomainPeriodMsaDetectSummariesResponse get_detect_summaries(body)
View information about detections

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) | View key attributes of detections, including the associated host, [disposition](https://falcon.crowdstrike.com/documentation/86/detections-monitoring-apis#pattern-disposition-value-descriptions), objective/tactic/technique, adversary, and more. Specify one or more detection IDs (max 1000 per request). Find detection IDs with the `/detects/queries/detects/v1` endpoint, the Falcon console, or the Streaming API. | [required] |

### Return type

[**models::DomainPeriodMsaDetectSummariesResponse**](domain.MsaDetectSummariesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_detects

> models::MsaPeriodQueryResponse query_detects(offset, limit, sort, filter, q)
Search for detection IDs that match a given query

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The first detection to return, where `0` is the latest detection. Use with the `limit` parameter to manage pagination of results. |  |
**limit** | Option<**i32**> | The maximum number of detections to return in this response (default: 9999; max: 9999). Use with the `offset` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort detections using these options:  - `first_behavior`: Timestamp of the first behavior associated with this detection - `last_behavior`: Timestamp of the last behavior associated with this detection - `max_severity`: Highest severity of the behaviors associated with this detection - `max_confidence`: Highest confidence of the behaviors associated with this detection - `adversary_id`: ID of the adversary associated with this detection, if any - `device.hostname`: Hostname of the host where this detection was detected  Sort either `asc` (ascending) or `desc` (descending). For example: `last_behavior|asc` |  |
**filter** | Option<**String**> | Filter detections using a query in Falcon Query Language (FQL) An asterisk wildcard `*` includes all results.   Common filter options include:  - `status` - `device.device_id` - `max_severity`  The full list of valid filter options is extensive. Review it in our [documentation inside the Falcon console](https://falcon.crowdstrike.com/documentation/45/falcon-query-language-fql). |  |
**q** | Option<**String**> | Search all detection metadata for the provided string |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_detects_by_ids_v2

> models::MsaPeriodReplyMetaOnly update_detects_by_ids_v2(body)
Modify the state, assignee, and visibility of detections

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodDetectsEntitiesPatchRequest**](DomainPeriodDetectsEntitiesPatchRequest.md) | This endpoint modifies attributes (state and assignee) of detections.   This endpoint accepts a query formatted as a JSON array of key-value pairs. You can update one or more attributes one or more detections with a single request.  **`assigned_to_uuid` values**  A user ID, such as `1234567891234567891`  **`ids` values**  One or more detection IDs, which you can find with the `/detects/queries/detects/v1` endpoint, the Falcon console, or the Streaming API.  **`show_in_ui` values**  - `true`: This detection is displayed in Falcon - `false`: This detection is not displayed in Falcon. Most commonly used together with the `status` key's `false_positive` value.  **`status` values**  - `new` - `in_progress` - `true_positive` - `false_positive` - `closed` - `ignored`  **`comment` values** Optional comment to add to the detection. Comments are displayed with the detection in Falcon and usually used to provide context or notes for other Falcon users. A detection can have multiple comments over time. | [required] |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
