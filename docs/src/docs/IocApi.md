# \IocApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**action_period_get_period_v1**](IocApi.md#action_period_get_period_v1) | **GET** /iocs/entities/actions/v1 | Get Actions by ids.
[**action_period_query_period_v1**](IocApi.md#action_period_query_period_v1) | **GET** /iocs/queries/actions/v1 | Query Actions.
[**get_indicators_report**](IocApi.md#get_indicators_report) | **POST** /iocs/entities/indicators-reports/v1 | Launch an indicators report creation job
[**indicator_period_aggregate_period_v1**](IocApi.md#indicator_period_aggregate_period_v1) | **POST** /iocs/aggregates/indicators/v1 | Get Indicators aggregates as specified via json in the request body.
[**indicator_period_combined_period_v1**](IocApi.md#indicator_period_combined_period_v1) | **GET** /iocs/combined/indicator/v1 | Get Combined for Indicators.
[**indicator_period_create_period_v1**](IocApi.md#indicator_period_create_period_v1) | **POST** /iocs/entities/indicators/v1 | Create Indicators.
[**indicator_period_delete_period_v1**](IocApi.md#indicator_period_delete_period_v1) | **DELETE** /iocs/entities/indicators/v1 | Delete Indicators by ids.
[**indicator_period_get_period_device_count_period_v1**](IocApi.md#indicator_period_get_period_device_count_period_v1) | **GET** /iocs/aggregates/indicators/device-count/v1 | Get the number of devices the indicator has run on
[**indicator_period_get_period_devices_ran_on_period_v1**](IocApi.md#indicator_period_get_period_devices_ran_on_period_v1) | **GET** /iocs/queries/indicators/devices/v1 | Get the IDs of devices the indicator has run on
[**indicator_period_get_period_processes_ran_on_period_v1**](IocApi.md#indicator_period_get_period_processes_ran_on_period_v1) | **GET** /iocs/queries/indicators/processes/v1 | Get the number of processes the indicator has run on
[**indicator_period_get_period_v1**](IocApi.md#indicator_period_get_period_v1) | **GET** /iocs/entities/indicators/v1 | Get Indicators by ids.
[**indicator_period_search_period_v1**](IocApi.md#indicator_period_search_period_v1) | **GET** /iocs/queries/indicators/v1 | Search for Indicators.
[**indicator_period_update_period_v1**](IocApi.md#indicator_period_update_period_v1) | **PATCH** /iocs/entities/indicators/v1 | Update Indicators.
[**ioc_type_period_query_period_v1**](IocApi.md#ioc_type_period_query_period_v1) | **GET** /iocs/queries/ioc-types/v1 | Query IOC Types.
[**platform_period_query_period_v1**](IocApi.md#platform_period_query_period_v1) | **GET** /iocs/queries/platforms/v1 | Query Platforms.
[**severity_period_query_period_v1**](IocApi.md#severity_period_query_period_v1) | **GET** /iocs/queries/severities/v1 | Query Severities.

## action_period_get_period_v1

> models::ApiPeriodActionRespV1 action_period_get_period_v1(ids)
Get Actions by ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | The ids of the Actions to retrieve |  |

### Return type

[**models::ApiPeriodActionRespV1**](api.ActionRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## action_period_query_period_v1

> models::ApiPeriodIndicatorQueryRespV1 action_period_query_period_v1(offset, limit)
Query Actions.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::ApiPeriodIndicatorQueryRespV1**](api.IndicatorQueryRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_indicators_report

> models::MsaPeriodEntitiesResponse get_indicators_report(body)
Launch an indicators report creation job

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodIndicatorsReportRequest**](ApiPeriodIndicatorsReportRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodEntitiesResponse**](msa.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## indicator_period_aggregate_period_v1

> models::MsaPeriodAggregatesResponse indicator_period_aggregate_period_v1(body, filter, from_parent)
Get Indicators aggregates as specified via json in the request body.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodAggregateQueryRequest**](MsaPeriodAggregateQueryRequest.md) |  | [required] |
**filter** | Option<**String**> | The filter to narrow down the aggregation data |  |
**from_parent** | Option<**bool**> | The filter for returning either only indicators for the request customer or its MSSP parents |  |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## indicator_period_combined_period_v1

> models::ApiPeriodIndicatorRespV1 indicator_period_combined_period_v1(filter, offset, limit, sort, after, from_parent)
Get Combined for Indicators.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results. |  |
**offset** | Option<**i32**> | The offset to start retrieving records from. Offset and After params are mutually exclusive. If none provided then scrolling will be used by default. To access more than 10k iocs, use the 'after' parameter instead of 'offset'. |  |
**limit** | Option<**i32**> | The maximum records to return. |  |
**sort** | Option<**String**> | The sort expression that should be used to sort the results. |  |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an 'after' token. On subsequent requests, provide the 'after' token from the previous response to continue from that place in the results. To access more than 10k indicators, use the 'after' parameter instead of 'offset'. |  |
**from_parent** | Option<**bool**> | The filter for returning either only indicators for the request customer or its MSSP parents |  |

### Return type

[**models::ApiPeriodIndicatorRespV1**](api.IndicatorRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## indicator_period_create_period_v1

> models::ApiPeriodIndicatorRespV1 indicator_period_create_period_v1(body, retrodetects, ignore_warnings)
Create Indicators.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodIndicatorCreateReqsV1**](ApiPeriodIndicatorCreateReqsV1.md) |  | [required] |
**retrodetects** | Option<**bool**> | Whether to submit to retrodetects |  |
**ignore_warnings** | Option<**bool**> | Set to true to ignore warnings and add all IOCs |  |[default to false]

### Return type

[**models::ApiPeriodIndicatorRespV1**](api.IndicatorRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## indicator_period_delete_period_v1

> models::ApiPeriodIndicatorQueryRespV1 indicator_period_delete_period_v1(filter, ids, comment, from_parent)
Delete Indicators by ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The FQL expression to delete Indicators in bulk. If both 'filter' and 'ids' are provided, then filter takes precedence and ignores ids. |  |
**ids** | Option<[**Vec<String>**](String.md)> | The ids of the Indicators to delete. If both 'filter' and 'ids' are provided, then filter takes precedence and ignores ids |  |
**comment** | Option<**String**> | The comment why these indicators were deleted |  |
**from_parent** | Option<**bool**> | The filter for returning either only indicators for the request customer or its MSSP parents |  |

### Return type

[**models::ApiPeriodIndicatorQueryRespV1**](api.IndicatorQueryRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## indicator_period_get_period_device_count_period_v1

> models::ApiPeriodDeviceCountRespV1 indicator_period_get_period_device_count_period_v1(r#type, value)
Get the number of devices the indicator has run on

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  The type of the indicator. Valid types include:  sha256: A hex-encoded sha256 hash string. Length - min: 64, max: 64.  md5: A hex-encoded md5 hash string. Length - min 32, max: 32.  domain: A domain name. Length - min: 1, max: 200.  ipv4: An IPv4 address. Must be a valid IP address.  ipv6: An IPv6 address. Must be a valid IP address.  | [required] |
**value** | **String** | The string representation of the indicator | [required] |

### Return type

[**models::ApiPeriodDeviceCountRespV1**](api.DeviceCountRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## indicator_period_get_period_devices_ran_on_period_v1

> models::ApiPeriodDevicesRanOnRespV1 indicator_period_get_period_devices_ran_on_period_v1(r#type, value, limit, offset)
Get the IDs of devices the indicator has run on

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  The type of the indicator. Valid types include:  sha256: A hex-encoded sha256 hash string. Length - min: 64, max: 64.  md5: A hex-encoded md5 hash string. Length - min 32, max: 32.  domain: A domain name. Length - min: 1, max: 200.  ipv4: An IPv4 address. Must be a valid IP address.  ipv6: An IPv6 address. Must be a valid IP address.  | [required] |
**value** | **String** | The string representation of the indicator | [required] |
**limit** | Option<**String**> | The maximum number of results to return. Use with the offset parameter to manage pagination of results. |  |
**offset** | Option<**String**> | The first process to return, where 0 is the latest offset. Use with the limit parameter to manage pagination of results. |  |

### Return type

[**models::ApiPeriodDevicesRanOnRespV1**](api.DevicesRanOnRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## indicator_period_get_period_processes_ran_on_period_v1

> models::ApiPeriodProcessesRanOnRespV1 indicator_period_get_period_processes_ran_on_period_v1(r#type, value, device_id, limit, offset)
Get the number of processes the indicator has run on

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  The type of the indicator. Valid types include:  sha256: A hex-encoded sha256 hash string. Length - min: 64, max: 64.  md5: A hex-encoded md5 hash string. Length - min 32, max: 32.  domain: A domain name. Length - min: 1, max: 200.  ipv4: An IPv4 address. Must be a valid IP address.  ipv6: An IPv6 address. Must be a valid IP address.  | [required] |
**value** | **String** | The string representation of the indicator | [required] |
**device_id** | **String** | Specify a host's ID to return only processes from that host. Get a host's ID from GET /devices/queries/devices/v1, the Falcon console, or the Streaming API. | [required] |
**limit** | Option<**String**> | The maximum number of results to return. Use with the offset parameter to manage pagination of results. |  |
**offset** | Option<**String**> | The first process to return, where 0 is the latest offset. Use with the limit parameter to manage pagination of results. |  |

### Return type

[**models::ApiPeriodProcessesRanOnRespV1**](api.ProcessesRanOnRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## indicator_period_get_period_v1

> models::ApiPeriodIndicatorRespV1 indicator_period_get_period_v1(ids)
Get Indicators by ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The ids of the Indicators to retrieve | [required] |

### Return type

[**models::ApiPeriodIndicatorRespV1**](api.IndicatorRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## indicator_period_search_period_v1

> models::ApiPeriodIndicatorQueryRespV1 indicator_period_search_period_v1(filter, offset, limit, sort, after, from_parent)
Search for Indicators.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results. |  |
**offset** | Option<**i32**> | The offset to start retrieving records from. Offset and After params are mutually exclusive. If none provided then scrolling will be used by default. To access more than 10k iocs, use the 'after' parameter instead of 'offset'. |  |
**limit** | Option<**i32**> | The maximum records to return. |  |
**sort** | Option<**String**> | The sort expression that should be used to sort the results. |  |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an 'after' token. On subsequent requests, provide the 'after' token from the previous response to continue from that place in the results. To access more than 10k indicators, use the 'after' parameter instead of 'offset'. |  |
**from_parent** | Option<**bool**> | The filter for returning either only indicators for the request customer or its MSSP parents |  |

### Return type

[**models::ApiPeriodIndicatorQueryRespV1**](api.IndicatorQueryRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## indicator_period_update_period_v1

> models::ApiPeriodIndicatorRespV1 indicator_period_update_period_v1(body, retrodetects, ignore_warnings)
Update Indicators.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodIndicatorUpdateReqsV1**](ApiPeriodIndicatorUpdateReqsV1.md) |  | [required] |
**retrodetects** | Option<**bool**> | Whether to submit to retrodetects |  |
**ignore_warnings** | Option<**bool**> | Set to true to ignore warnings and add all IOCs |  |[default to false]

### Return type

[**models::ApiPeriodIndicatorRespV1**](api.IndicatorRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ioc_type_period_query_period_v1

> models::ApiPeriodIndicatorQueryRespV1 ioc_type_period_query_period_v1(offset, limit)
Query IOC Types.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::ApiPeriodIndicatorQueryRespV1**](api.IndicatorQueryRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## platform_period_query_period_v1

> models::ApiPeriodIndicatorQueryRespV1 platform_period_query_period_v1(offset, limit)
Query Platforms.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::ApiPeriodIndicatorQueryRespV1**](api.IndicatorQueryRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## severity_period_query_period_v1

> models::ApiPeriodIndicatorQueryRespV1 severity_period_query_period_v1(offset, limit)
Query Severities.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::ApiPeriodIndicatorQueryRespV1**](api.IndicatorQueryRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
