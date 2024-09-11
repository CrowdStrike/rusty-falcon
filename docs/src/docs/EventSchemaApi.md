# \EventSchemaApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fdrschema_period_combined_period_event_period_get**](EventSchemaApi.md#fdrschema_period_combined_period_event_period_get) | **GET** /fdr/combined/schema-members/v1 | Fetch combined schema
[**fdrschema_period_entities_period_event_period_get**](EventSchemaApi.md#fdrschema_period_entities_period_event_period_get) | **GET** /fdr/entities/schema-events/v1 | Fetch event schema by ID
[**fdrschema_period_queries_period_event_period_get**](EventSchemaApi.md#fdrschema_period_queries_period_event_period_get) | **GET** /fdr/queries/schema-events/v1 | Get list of event IDs given a particular query.

## fdrschema_period_combined_period_event_period_get

> models::SchemaPeriodSensorEventResponseV1 fdrschema_period_combined_period_event_period_get()
Fetch combined schema

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SchemaPeriodSensorEventResponseV1**](schema.SensorEventResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fdrschema_period_entities_period_event_period_get

> models::SchemaPeriodSensorEventResponseV1 fdrschema_period_entities_period_event_period_get(ids)
Fetch event schema by ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Specify feed IDs to fetch |  |

### Return type

[**models::SchemaPeriodSensorEventResponseV1**](schema.SensorEventResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fdrschema_period_queries_period_event_period_get

> models::MsaspecPeriodQueryResponse fdrschema_period_queries_period_event_period_get(limit, offset, filter, sort)
Get list of event IDs given a particular query.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Limit of the data |  |
**offset** | Option<**i32**> | Offset into the data |  |
**filter** | Option<**String**> | FQL filter of the data |  |
**sort** | Option<**String**> | Sort the data |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
