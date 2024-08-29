# \IdentityEntitiesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sensor_aggregates**](IdentityEntitiesApi.md#get_sensor_aggregates) | **POST** /identity-protection/aggregates/devices/GET/v1 | Get sensor aggregates as specified via json in request body.
[**get_sensor_details**](IdentityEntitiesApi.md#get_sensor_details) | **POST** /identity-protection/entities/devices/GET/v1 | Get details on one or more sensors by providing device IDs in a POST body. Supports up to a maximum of 5000 IDs.
[**query_sensors_by_filter**](IdentityEntitiesApi.md#query_sensors_by_filter) | **GET** /identity-protection/queries/devices/v1 | Search for sensors in your environment by hostname, IP, and other criteria.

## get_sensor_aggregates

> models::MsaPeriodAggregatesResponse get_sensor_aggregates(body)
Get sensor aggregates as specified via json in request body.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodAggregateQueryRequest**](MsaPeriodAggregateQueryRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sensor_details

> models::ApiPeriodSensorDetailsResponseSwagger get_sensor_details(body)
Get details on one or more sensors by providing device IDs in a POST body. Supports up to a maximum of 5000 IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::ApiPeriodSensorDetailsResponseSwagger**](api.SensorDetailsResponseSwagger.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_sensors_by_filter

> models::MsaspecPeriodQueryResponse query_sensors_by_filter(offset, limit, sort, filter)
Search for sensors in your environment by hostname, IP, and other criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-200] |  |
**sort** | Option<**String**> | The property to sort by (e.g. status.desc or hostname.asc) |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
