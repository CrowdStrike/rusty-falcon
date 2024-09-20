# \ReportExecutionsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**report_executions_download_period_get**](ReportExecutionsApi.md#report_executions_download_period_get) | **GET** /reports/entities/report-executions-download/v1 | Get report entity download
[**report_executions_period_get**](ReportExecutionsApi.md#report_executions_period_get) | **GET** /reports/entities/report-executions/v1 | Retrieve report details for the provided report IDs.
[**report_executions_period_query**](ReportExecutionsApi.md#report_executions_period_query) | **GET** /reports/queries/report-executions/v1 | Find all report execution IDs matching the query with filter
[**report_executions_period_retry**](ReportExecutionsApi.md#report_executions_period_retry) | **POST** /reports/entities/report-executions-retry/v1 | This endpoint will be used to retry report executions

## report_executions_download_period_get

> Vec<i32> report_executions_download_period_get(ids)
Get report entity download

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The report_execution id to download | [required] |

### Return type

**Vec<i32>**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## report_executions_period_get

> models::DomainPeriodReportExecutionsResponseV1 report_executions_period_get(ids)
Retrieve report details for the provided report IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The report_execution id to get details about. | [required] |

### Return type

[**models::DomainPeriodReportExecutionsResponseV1**](domain.ReportExecutionsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## report_executions_period_query

> models::MsaPeriodQueryResponse report_executions_period_query(sort, filter, q, offset, limit)
Find all report execution IDs matching the query with filter

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Possible order by fields: created_on, last_updated_on |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Filter term criteria: type, scheduled_report_id, status. Filter range criteria: created_on, last_updated_on, expiration_on; use any common date format, such as '2010-05-15T14:55:21.892315096Z'. |  |
**q** | Option<**String**> | Match query criteria, which includes all the filter string fields |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## report_executions_period_retry

> models::DomainPeriodReportExecutionsResponseV1 report_executions_period_retry(body)
This endpoint will be used to retry report executions

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::DomainPeriodReportExecutionRetryRequestV1>**](domain.ReportExecutionRetryRequestV1.md) |  | [required] |

### Return type

[**models::DomainPeriodReportExecutionsResponseV1**](domain.ReportExecutionsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
