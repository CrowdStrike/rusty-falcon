# \SensorDownloadApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_sensor_installer_by_id**](SensorDownloadApi.md#download_sensor_installer_by_id) | **GET** /sensors/entities/download-installer/v1 | Download sensor installer by SHA256 ID
[**download_sensor_installer_by_id_v2**](SensorDownloadApi.md#download_sensor_installer_by_id_v2) | **GET** /sensors/entities/download-installer/v2 | Download sensor installer by SHA256 ID
[**get_combined_sensor_installers_by_query**](SensorDownloadApi.md#get_combined_sensor_installers_by_query) | **GET** /sensors/combined/installers/v1 | Get sensor installer details by provided query
[**get_combined_sensor_installers_by_query_v2**](SensorDownloadApi.md#get_combined_sensor_installers_by_query_v2) | **GET** /sensors/combined/installers/v2 | Get sensor installer details by provided query
[**get_sensor_installers_by_query**](SensorDownloadApi.md#get_sensor_installers_by_query) | **GET** /sensors/queries/installers/v1 | Get sensor installer IDs by provided query
[**get_sensor_installers_by_query_v2**](SensorDownloadApi.md#get_sensor_installers_by_query_v2) | **GET** /sensors/queries/installers/v2 | Get sensor installer IDs by provided query
[**get_sensor_installers_ccidby_query**](SensorDownloadApi.md#get_sensor_installers_ccidby_query) | **GET** /sensors/queries/installers/ccid/v1 | Get CCID to use with sensor installers
[**get_sensor_installers_entities**](SensorDownloadApi.md#get_sensor_installers_entities) | **GET** /sensors/entities/installers/v1 | Get sensor installer details by provided SHA256 IDs
[**get_sensor_installers_entities_v2**](SensorDownloadApi.md#get_sensor_installers_entities_v2) | **GET** /sensors/entities/installers/v2 | Get sensor installer details by provided SHA256 IDs

## download_sensor_installer_by_id

> serde_json::Value download_sensor_installer_by_id(id)
Download sensor installer by SHA256 ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | SHA256 of the installer to download | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## download_sensor_installer_by_id_v2

> serde_json::Value download_sensor_installer_by_id_v2(id)
Download sensor installer by SHA256 ID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | SHA256 of the installer to download | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_combined_sensor_installers_by_query

> models::DomainPeriodSensorInstallersV1 get_combined_sensor_installers_by_query(offset, limit, sort, filter)
Get sensor installer details by provided query

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The first item to return, where 0 is the latest item. Use with the limit parameter to manage pagination of results. |  |
**limit** | Option<**i32**> | The number of items to return in this response (default: 100, max: 500). Use with the offset parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort items using their properties. Common sort options include:  <ul><li>version|asc</li><li>release_date|desc</li></ul> |  |
**filter** | Option<**String**> | Filter items using a query in Falcon Query Language (FQL). An asterisk wildcard * includes all results.  Common filter options include: <ul><li>platform:\"windows\"</li><li>version:>\"5.2\"</li></ul> |  |

### Return type

[**models::DomainPeriodSensorInstallersV1**](domain.SensorInstallersV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_combined_sensor_installers_by_query_v2

> models::DomainPeriodSensorInstallersV2 get_combined_sensor_installers_by_query_v2(offset, limit, sort, filter)
Get sensor installer details by provided query

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The first item to return, where 0 is the latest item. Use with the limit parameter to manage pagination of results. |  |
**limit** | Option<**i32**> | The number of items to return in this response (default: 100, max: 500). Use with the offset parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort items using their properties. Common sort options include:  <ul><li>version|asc</li><li>release_date|desc</li></ul> |  |
**filter** | Option<**String**> | Filter items using a query in Falcon Query Language (FQL). An asterisk wildcard * includes all results.  Common filter options include: <ul><li>platform:\"windows\"</li><li>version:>\"5.2\"</li></ul> |  |

### Return type

[**models::DomainPeriodSensorInstallersV2**](domain.SensorInstallersV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sensor_installers_by_query

> models::MsaspecPeriodQueryResponse get_sensor_installers_by_query(offset, limit, sort, filter)
Get sensor installer IDs by provided query

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The first item to return, where 0 is the latest item. Use with the limit parameter to manage pagination of results. |  |
**limit** | Option<**i32**> | The number of items to return in this response (default: 100, max: 500). Use with the offset parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort items using their properties. Common sort options include:  <ul><li>version|asc</li><li>release_date|desc</li></ul> |  |
**filter** | Option<**String**> | Filter items using a query in Falcon Query Language (FQL). An asterisk wildcard * includes all results.  Common filter options include: <ul><li>platform:\"windows\"</li><li>version:>\"5.2\"</li></ul> |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sensor_installers_by_query_v2

> models::MsaspecPeriodQueryResponse get_sensor_installers_by_query_v2(offset, limit, sort, filter)
Get sensor installer IDs by provided query

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The first item to return, where 0 is the latest item. Use with the limit parameter to manage pagination of results. |  |
**limit** | Option<**i32**> | The number of items to return in this response (default: 100, max: 500). Use with the offset parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort items using their properties. Common sort options include:  <ul><li>version|asc</li><li>release_date|desc</li></ul> |  |
**filter** | Option<**String**> | Filter items using a query in Falcon Query Language (FQL). An asterisk wildcard * includes all results.  Common filter options include: <ul><li>platform:\"windows\"</li><li>version:>\"5.2\"</li></ul> |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sensor_installers_ccidby_query

> models::MsaspecPeriodQueryResponse get_sensor_installers_ccidby_query()
Get CCID to use with sensor installers

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sensor_installers_entities

> models::DomainPeriodSensorInstallersV1 get_sensor_installers_entities(ids)
Get sensor installer details by provided SHA256 IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the installers | [required] |

### Return type

[**models::DomainPeriodSensorInstallersV1**](domain.SensorInstallersV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sensor_installers_entities_v2

> models::DomainPeriodSensorInstallersV2 get_sensor_installers_entities_v2(ids)
Get sensor installer details by provided SHA256 IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the installers | [required] |

### Return type

[**models::DomainPeriodSensorInstallersV2**](domain.SensorInstallersV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
