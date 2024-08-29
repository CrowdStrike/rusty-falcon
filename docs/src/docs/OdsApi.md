# \OdsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aggregate_query_scan_host_metadata**](OdsApi.md#aggregate_query_scan_host_metadata) | **POST** /ods/aggregates/scan-hosts/v1 | Get aggregates on ODS scan-hosts data.
[**aggregate_scans**](OdsApi.md#aggregate_scans) | **POST** /ods/aggregates/scans/v1 | Get aggregates on ODS scan data.
[**aggregate_scheduled_scans**](OdsApi.md#aggregate_scheduled_scans) | **POST** /ods/aggregates/scheduled-scans/v1 | Get aggregates on ODS scheduled-scan data.
[**cancel_scans**](OdsApi.md#cancel_scans) | **POST** /ods/entities/scan-control-actions/cancel/v1 | Cancel ODS scans for the given scan ids.
[**create_scan**](OdsApi.md#create_scan) | **POST** /ods/entities/scans/v1 | Create ODS scan and start or schedule scan for the given scan request.
[**delete_scheduled_scans**](OdsApi.md#delete_scheduled_scans) | **DELETE** /ods/entities/scheduled-scans/v1 | Delete ODS scheduled-scans for the given scheduled-scan ids.
[**get_malicious_files_by_ids**](OdsApi.md#get_malicious_files_by_ids) | **GET** /ods/entities/malicious-files/v1 | Get malicious files by ids.
[**get_scan_host_metadata_by_ids**](OdsApi.md#get_scan_host_metadata_by_ids) | **GET** /ods/entities/scan-hosts/v1 | Get scan hosts by ids.
[**get_scans_by_scan_ids**](OdsApi.md#get_scans_by_scan_ids) | **GET** /ods/entities/scans/v1 | Get Scans by IDs.
[**get_scans_by_scan_ids_v2**](OdsApi.md#get_scans_by_scan_ids_v2) | **GET** /ods/entities/scans/v2 | Get Scans by IDs.
[**get_scheduled_scans_by_scan_ids**](OdsApi.md#get_scheduled_scans_by_scan_ids) | **GET** /ods/entities/scheduled-scans/v1 | Get ScheduledScans by IDs.
[**query_malicious_files**](OdsApi.md#query_malicious_files) | **GET** /ods/queries/malicious-files/v1 | Query malicious files.
[**query_scan_host_metadata**](OdsApi.md#query_scan_host_metadata) | **GET** /ods/queries/scan-hosts/v1 | Query scan hosts.
[**query_scans**](OdsApi.md#query_scans) | **GET** /ods/queries/scans/v1 | Query Scans.
[**query_scheduled_scans**](OdsApi.md#query_scheduled_scans) | **GET** /ods/queries/scheduled-scans/v1 | Query ScheduledScans.
[**schedule_scan**](OdsApi.md#schedule_scan) | **POST** /ods/entities/scheduled-scans/v1 | Create ODS scan and start or schedule scan for the given scan request.

## aggregate_query_scan_host_metadata

> models::MsaPeriodAggregatesResponse aggregate_query_scan_host_metadata(body)
Get aggregates on ODS scan-hosts data.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregate_scans

> models::MsaPeriodAggregatesResponse aggregate_scans(body)
Get aggregates on ODS scan data.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## aggregate_scheduled_scans

> models::MsaPeriodAggregatesResponse aggregate_scheduled_scans(body)
Get aggregates on ODS scheduled-scan data.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<models::MsaPeriodAggregateQueryRequest>**](msa.AggregateQueryRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## cancel_scans

> models::MsaspecPeriodQueryResponse cancel_scans(body)
Cancel ODS scans for the given scan ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EntitiesPeriodOdsCancelScanRequest**](EntitiesPeriodOdsCancelScanRequest.md) |  | [required] |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_scan

> models::EntitiesPeriodOdsScanResponse create_scan(body)
Create ODS scan and start or schedule scan for the given scan request.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EntitiesPeriodOdsScanRequest**](EntitiesPeriodOdsScanRequest.md) |  | [required] |

### Return type

[**models::EntitiesPeriodOdsScanResponse**](entities.ODSScanResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_scheduled_scans

> models::MsaspecPeriodQueryResponse delete_scheduled_scans(ids, filter)
Delete ODS scheduled-scans for the given scheduled-scan ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The scan IDs to retrieve the scan entities | [required] |
**filter** | Option<**String**> | A FQL compatible query string. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_malicious_files_by_ids

> models::EntitiesPeriodOdsScanMaliciousFileResponse get_malicious_files_by_ids(ids)
Get malicious files by ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The scan IDs to retrieve the scan entities | [required] |

### Return type

[**models::EntitiesPeriodOdsScanMaliciousFileResponse**](entities.ODSScanMaliciousFileResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_scan_host_metadata_by_ids

> models::EntitiesPeriodOdsScanHostResponse get_scan_host_metadata_by_ids(ids)
Get scan hosts by ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The scan IDs to retrieve the scan entities | [required] |

### Return type

[**models::EntitiesPeriodOdsScanHostResponse**](entities.ODSScanHostResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_scans_by_scan_ids

> models::EntitiesPeriodOdsScanResponse get_scans_by_scan_ids(ids)
Get Scans by IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The scan IDs to retrieve the scan entities | [required] |

### Return type

[**models::EntitiesPeriodOdsScanResponse**](entities.ODSScanResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_scans_by_scan_ids_v2

> models::EntitiesPeriodOdsScanResponseV2 get_scans_by_scan_ids_v2(ids)
Get Scans by IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The scan IDs to retrieve the scan entities | [required] |

### Return type

[**models::EntitiesPeriodOdsScanResponseV2**](entities.ODSScanResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_scheduled_scans_by_scan_ids

> models::EntitiesPeriodOdsScheduleScanResponse get_scheduled_scans_by_scan_ids(ids)
Get ScheduledScans by IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The scan IDs to retrieve the scan entities | [required] |

### Return type

[**models::EntitiesPeriodOdsScheduleScanResponse**](entities.ODSScheduleScanResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_malicious_files

> models::MsaspecPeriodQueryResponse query_malicious_files(filter, offset, limit, sort)
Query malicious files.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | A FQL compatible query string. Terms: [id scan_id host_id host_scan_id filepath filename hash pattern_id severity quarantined last_updated] |  |
**offset** | Option<**i32**> | Index of the starting resource |  |[default to 0]
**limit** | Option<**i32**> | The max number of resources to return |  |[default to 500]
**sort** | Option<**String**> | The property to sort on, followed by a |, followed by the sort direction, either \"asc\" or \"desc\" |  |[default to last_updated|desc]

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_scan_host_metadata

> models::MsaspecPeriodQueryResponse query_scan_host_metadata(filter, offset, limit, sort)
Query scan hosts.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | A FQL compatible query string. Terms: [id profile_id host_id scan_id host_scan_id filecount.scanned filecount.malicious filecount.quarantined filecount.skipped affected_hosts_count status severity completed_on started_on last_updated scan_control_reason] |  |
**offset** | Option<**i32**> | Index of the starting resource |  |[default to 0]
**limit** | Option<**i32**> | The max number of resources to return |  |[default to 500]
**sort** | Option<**String**> | The property to sort on, followed by a |, followed by the sort direction, either \"asc\" or \"desc\" |  |[default to last_updated|desc]

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_scans

> models::MsaspecPeriodQueryResponse query_scans(filter, offset, limit, sort)
Query Scans.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | A FQL compatible query string. Terms: [id profile_id description.keyword initiated_from filecount.scanned filecount.malicious filecount.quarantined filecount.skipped affected_hosts_count status severity scan_started_on scan_completed_on created_on created_by last_updated targeted_host_count missing_host_count] |  |
**offset** | Option<**i32**> | Index of the starting resource |  |[default to 0]
**limit** | Option<**i32**> | The max number of resources to return |  |[default to 500]
**sort** | Option<**String**> | The property to sort on, followed by a |, followed by the sort direction, either \"asc\" or \"desc\" |  |[default to created_on|desc]

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_scheduled_scans

> models::MsaspecPeriodQueryResponse query_scheduled_scans(filter, offset, limit, sort)
Query ScheduledScans.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | A FQL compatible query string. Terms: [id description initiated_from status schedule.start_timestamp schedule.Interval created_on created_by last_updated deleted] |  |
**offset** | Option<**i32**> | Index of the starting resource |  |[default to 0]
**limit** | Option<**i32**> | The max number of resources to return |  |[default to 500]
**sort** | Option<**String**> | The property to sort on, followed by a |, followed by the sort direction, either \"asc\" or \"desc\" |  |[default to schedule.start_timestamp|desc]

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## schedule_scan

> models::EntitiesPeriodOdsScheduleScanResponse schedule_scan(body)
Create ODS scan and start or schedule scan for the given scan request.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EntitiesPeriodOdsScheduleScanRequest**](EntitiesPeriodOdsScheduleScanRequest.md) |  | [required] |

### Return type

[**models::EntitiesPeriodOdsScheduleScanResponse**](entities.ODSScheduleScanResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
