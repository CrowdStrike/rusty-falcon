# \QuickScanApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_scans**](QuickScanApi.md#get_scans) | **GET** /scanner/entities/scans/v1 | Check the status of a volume scan. Time required for analysis increases with the number of samples in a volume but usually it should take less than 1 minute
[**get_scans_aggregates**](QuickScanApi.md#get_scans_aggregates) | **POST** /scanner/aggregates/scans/GET/v1 | Get scans aggregations as specified via json in request body.
[**query_submissions_mixin0**](QuickScanApi.md#query_submissions_mixin0) | **GET** /scanner/queries/scans/v1 | Find IDs for submitted scans by providing an FQL filter and paging details. Returns a set of volume IDs that match your criteria.
[**scan_samples**](QuickScanApi.md#scan_samples) | **POST** /scanner/entities/scans/v1 | Submit a volume of files for ml scanning. Time required for analysis increases with the number of samples in a volume but usually it should take less than 1 minute

## get_scans

> models::MlscannerapiPeriodScanV1Response get_scans(ids)
Check the status of a volume scan. Time required for analysis increases with the number of samples in a volume but usually it should take less than 1 minute

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | ID of a submitted scan | [required] |

### Return type

[**models::MlscannerapiPeriodScanV1Response**](mlscannerapi.ScanV1Response.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_scans_aggregates

> get_scans_aggregates(body)
Get scans aggregations as specified via json in request body.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodAggregateQueryRequest**](MsaPeriodAggregateQueryRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_submissions_mixin0

> models::MlscannerapiPeriodQueryResponse query_submissions_mixin0(filter, offset, limit, sort)
Find IDs for submitted scans by providing an FQL filter and paging details. Returns a set of volume IDs that match your criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | The offset to start retrieving submissions from. |  |
**limit** | Option<**i32**> | Maximum number of volume IDs to return. Max: 5000. |  |
**sort** | Option<**String**> | Sort order: `asc` or `desc`. |  |

### Return type

[**models::MlscannerapiPeriodQueryResponse**](mlscannerapi.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## scan_samples

> models::MlscannerapiPeriodQueryResponse scan_samples(body)
Submit a volume of files for ml scanning. Time required for analysis increases with the number of samples in a volume but usually it should take less than 1 minute

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MlscannerapiPeriodSamplesScanParameters**](MlscannerapiPeriodSamplesScanParameters.md) | Submit a batch of SHA256s for ml scanning. The samples must have been previously uploaded through `/samples/entities/samples/v3` | [required] |

### Return type

[**models::MlscannerapiPeriodQueryResponse**](mlscannerapi.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
