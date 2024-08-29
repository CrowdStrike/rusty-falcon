# \IoaExclusionsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ioa_exclusions_v1**](IoaExclusionsApi.md#create_ioa_exclusions_v1) | **POST** /policy/entities/ioa-exclusions/v1 | Create the IOA exclusions
[**delete_ioa_exclusions_v1**](IoaExclusionsApi.md#delete_ioa_exclusions_v1) | **DELETE** /policy/entities/ioa-exclusions/v1 | Delete the IOA exclusions by id
[**get_ioa_exclusions_v1**](IoaExclusionsApi.md#get_ioa_exclusions_v1) | **GET** /policy/entities/ioa-exclusions/v1 | Get a set of IOA Exclusions by specifying their IDs
[**query_ioa_exclusions_v1**](IoaExclusionsApi.md#query_ioa_exclusions_v1) | **GET** /policy/queries/ioa-exclusions/v1 | Search for IOA exclusions.
[**update_ioa_exclusions_v1**](IoaExclusionsApi.md#update_ioa_exclusions_v1) | **PATCH** /policy/entities/ioa-exclusions/v1 | Update the IOA exclusions

## create_ioa_exclusions_v1

> models::IoaExclusionsPeriodIoaExclusionsRespV1 create_ioa_exclusions_v1(body)
Create the IOA exclusions

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**IoaExclusionsPeriodIoaExclusionCreateReqV1**](IoaExclusionsPeriodIoaExclusionCreateReqV1.md) |  | [required] |

### Return type

[**models::IoaExclusionsPeriodIoaExclusionsRespV1**](ioa_exclusions.IoaExclusionsRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_ioa_exclusions_v1

> models::MsaPeriodQueryResponse delete_ioa_exclusions_v1(ids, comment)
Delete the IOA exclusions by id

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

## get_ioa_exclusions_v1

> models::IoaExclusionsPeriodIoaExclusionsRespV1 get_ioa_exclusions_v1(ids)
Get a set of IOA Exclusions by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The ids of the exclusions to retrieve | [required] |

### Return type

[**models::IoaExclusionsPeriodIoaExclusionsRespV1**](ioa_exclusions.IoaExclusionsRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_ioa_exclusions_v1

> models::MsaPeriodQueryResponse query_ioa_exclusions_v1(filter, offset, limit, sort)
Search for IOA exclusions.

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

## update_ioa_exclusions_v1

> models::IoaExclusionsPeriodIoaExclusionsRespV1 update_ioa_exclusions_v1(body)
Update the IOA exclusions

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**IoaExclusionsPeriodIoaExclusionUpdateReqV1**](IoaExclusionsPeriodIoaExclusionUpdateReqV1.md) |  | [required] |

### Return type

[**models::IoaExclusionsPeriodIoaExclusionsRespV1**](ioa_exclusions.IoaExclusionsRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
