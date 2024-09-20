# \SensorVisibilityExclusionsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sv_exclusions_v1**](SensorVisibilityExclusionsApi.md#create_sv_exclusions_v1) | **POST** /policy/entities/sv-exclusions/v1 | Create the sensor visibility exclusions
[**delete_sensor_visibility_exclusions_v1**](SensorVisibilityExclusionsApi.md#delete_sensor_visibility_exclusions_v1) | **DELETE** /policy/entities/sv-exclusions/v1 | Delete the sensor visibility exclusions by id
[**get_sensor_visibility_exclusions_v1**](SensorVisibilityExclusionsApi.md#get_sensor_visibility_exclusions_v1) | **GET** /policy/entities/sv-exclusions/v1 | Get a set of Sensor Visibility Exclusions by specifying their IDs
[**query_sensor_visibility_exclusions_v1**](SensorVisibilityExclusionsApi.md#query_sensor_visibility_exclusions_v1) | **GET** /policy/queries/sv-exclusions/v1 | Search for sensor visibility exclusions.
[**update_sensor_visibility_exclusions_v1**](SensorVisibilityExclusionsApi.md#update_sensor_visibility_exclusions_v1) | **PATCH** /policy/entities/sv-exclusions/v1 | Update the sensor visibility exclusions

## create_sv_exclusions_v1

> models::ExclusionsPeriodRespV1 create_sv_exclusions_v1(body)
Create the sensor visibility exclusions

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SvExclusionsPeriodCreateReqV1**](SvExclusionsPeriodCreateReqV1.md) |  | [required] |

### Return type

[**models::ExclusionsPeriodRespV1**](exclusions.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_sensor_visibility_exclusions_v1

> models::MsaPeriodQueryResponse delete_sensor_visibility_exclusions_v1(ids, comment)
Delete the sensor visibility exclusions by id

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The ids of the exclusions to delete | [required] |
**comment** | Option<**String**> | Explains why this exclusions was deleted |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sensor_visibility_exclusions_v1

> models::SvExclusionsPeriodRespV1 get_sensor_visibility_exclusions_v1(ids)
Get a set of Sensor Visibility Exclusions by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The ids of the exclusions to retrieve | [required] |

### Return type

[**models::SvExclusionsPeriodRespV1**](sv_exclusions.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_sensor_visibility_exclusions_v1

> models::MsaPeriodQueryResponse query_sensor_visibility_exclusions_v1(filter, offset, limit, sort)
Search for sensor visibility exclusions.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results. |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The sort expression that should be used to sort the results. |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_sensor_visibility_exclusions_v1

> models::SvExclusionsPeriodRespV1 update_sensor_visibility_exclusions_v1(body)
Update the sensor visibility exclusions

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SvExclusionsPeriodUpdateReqV1**](SvExclusionsPeriodUpdateReqV1.md) |  | [required] |

### Return type

[**models::SvExclusionsPeriodRespV1**](sv_exclusions.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
