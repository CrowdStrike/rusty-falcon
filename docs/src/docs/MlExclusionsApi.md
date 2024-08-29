# \MlExclusionsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ml_exclusions_v1**](MlExclusionsApi.md#create_ml_exclusions_v1) | **POST** /policy/entities/ml-exclusions/v1 | Create the ML exclusions
[**delete_ml_exclusions_v1**](MlExclusionsApi.md#delete_ml_exclusions_v1) | **DELETE** /policy/entities/ml-exclusions/v1 | Delete the ML exclusions by id
[**get_ml_exclusions_v1**](MlExclusionsApi.md#get_ml_exclusions_v1) | **GET** /policy/entities/ml-exclusions/v1 | Get a set of ML Exclusions by specifying their IDs
[**query_ml_exclusions_v1**](MlExclusionsApi.md#query_ml_exclusions_v1) | **GET** /policy/queries/ml-exclusions/v1 | Search for ML exclusions.
[**update_ml_exclusions_v1**](MlExclusionsApi.md#update_ml_exclusions_v1) | **PATCH** /policy/entities/ml-exclusions/v1 | Update the ML exclusions

## create_ml_exclusions_v1

> models::ExclusionsPeriodRespV1 create_ml_exclusions_v1(body)
Create the ML exclusions

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ExclusionsPeriodCreateReqV1**](ExclusionsPeriodCreateReqV1.md) |  | [required] |

### Return type

[**models::ExclusionsPeriodRespV1**](exclusions.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_ml_exclusions_v1

> models::ExclusionsPeriodRespV1 delete_ml_exclusions_v1(ids, comment)
Delete the ML exclusions by id

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The ids of the exclusions to delete | [required] |
**comment** | Option<**String**> | Explains why this exclusions was deleted |  |

### Return type

[**models::ExclusionsPeriodRespV1**](exclusions.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_ml_exclusions_v1

> models::ExclusionsPeriodRespV1 get_ml_exclusions_v1(ids)
Get a set of ML Exclusions by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The ids of the exclusions to retrieve | [required] |

### Return type

[**models::ExclusionsPeriodRespV1**](exclusions.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_ml_exclusions_v1

> models::MsaPeriodQueryResponse query_ml_exclusions_v1(filter, offset, limit, sort)
Search for ML exclusions.

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

## update_ml_exclusions_v1

> models::ExclusionsPeriodRespV1 update_ml_exclusions_v1(body)
Update the ML exclusions

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SvExclusionsPeriodUpdateReqV1**](SvExclusionsPeriodUpdateReqV1.md) |  | [required] |

### Return type

[**models::ExclusionsPeriodRespV1**](exclusions.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
