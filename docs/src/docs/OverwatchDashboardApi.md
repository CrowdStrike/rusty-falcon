# \OverwatchDashboardApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aggregates_detections_global_counts**](OverwatchDashboardApi.md#aggregates_detections_global_counts) | **GET** /overwatch-dashboards/aggregates/detections-global-counts/v1 | Get the total number of detections pushed across all customers
[**aggregates_events**](OverwatchDashboardApi.md#aggregates_events) | **POST** /overwatch-dashboards/aggregates/events/GET/v1 | Get aggregate OverWatch detection event info by providing an aggregate query
[**aggregates_events_collections**](OverwatchDashboardApi.md#aggregates_events_collections) | **POST** /overwatch-dashboards/aggregates/events-collections/GET/v1 | Get OverWatch detection event collection info by providing an aggregate query
[**aggregates_incidents_global_counts**](OverwatchDashboardApi.md#aggregates_incidents_global_counts) | **GET** /overwatch-dashboards/aggregates/incidents-global-counts/v1 | Get the total number of incidents pushed across all customers
[**aggregates_ow_events_global_counts**](OverwatchDashboardApi.md#aggregates_ow_events_global_counts) | **GET** /overwatch-dashboards/aggregates/ow-events-global-counts/v1 | Get the total number of OverWatch events across all customers

## aggregates_detections_global_counts

> models::MsaPeriodFacetsResponse aggregates_detections_global_counts(filter)
Get the total number of detections pushed across all customers

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | An FQL filter string | [required] |

### Return type

[**models::MsaPeriodFacetsResponse**](msa.FacetsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregates_events

> models::MsaPeriodAggregatesResponse aggregates_events(body)
Get aggregate OverWatch detection event info by providing an aggregate query

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregates_events_collections

> models::MsaPeriodAggregatesResponse aggregates_events_collections(body)
Get OverWatch detection event collection info by providing an aggregate query

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregates_incidents_global_counts

> models::MsaPeriodFacetsResponse aggregates_incidents_global_counts(filter)
Get the total number of incidents pushed across all customers

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | An FQL filter string | [required] |

### Return type

[**models::MsaPeriodFacetsResponse**](msa.FacetsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregates_ow_events_global_counts

> models::MsaPeriodFacetsResponse aggregates_ow_events_global_counts(filter)
Get the total number of OverWatch events across all customers

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | An FQL filter string | [required] |

### Return type

[**models::MsaPeriodFacetsResponse**](msa.FacetsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
