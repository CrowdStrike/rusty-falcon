# \ScheduledReportsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scheduled_reports_period_get**](ScheduledReportsApi.md#scheduled_reports_period_get) | **GET** /reports/entities/scheduled-reports/v1 | Retrieve scheduled reports for the provided report IDs.
[**scheduled_reports_period_launch**](ScheduledReportsApi.md#scheduled_reports_period_launch) | **POST** /reports/entities/scheduled-reports/execution/v1 | Launch scheduled reports executions for the provided report IDs.
[**scheduled_reports_period_query**](ScheduledReportsApi.md#scheduled_reports_period_query) | **GET** /reports/queries/scheduled-reports/v1 | Find all report IDs matching the query with filter

## scheduled_reports_period_get

> models::DomainPeriodScheduledReportsResultV1 scheduled_reports_period_get(ids)
Retrieve scheduled reports for the provided report IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The scheduled_report id to get details about. | [required] |

### Return type

[**models::DomainPeriodScheduledReportsResultV1**](domain.ScheduledReportsResultV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## scheduled_reports_period_launch

> models::DomainPeriodReportExecutionsResponseV1 scheduled_reports_period_launch(body)
Launch scheduled reports executions for the provided report IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::DomainPeriodReportExecutionLaunchRequestV1>**](domain.ReportExecutionLaunchRequestV1.md) |  | [required] |

### Return type

[**models::DomainPeriodReportExecutionsResponseV1**](domain.ReportExecutionsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## scheduled_reports_period_query

> models::MsaPeriodQueryResponse scheduled_reports_period_query(sort, filter, q, offset, limit)
Find all report IDs matching the query with filter

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Possible order by fields: created_on, last_updated_on, last_execution_on, next_execution_on |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Filter term criteria: type, trigger_reference, recipients, user_uuid, cid, trigger_params.metadata. Filter range criteria: created_on, modified_on; use any common date format, such as '2010-05-15T14:55:21.892315096Z'. |  |
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
