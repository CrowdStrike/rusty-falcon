# \FalconCompleteDashboardApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aggregate_alerts**](FalconCompleteDashboardApi.md#aggregate_alerts) | **POST** /falcon-complete-dashboards/aggregates/alerts/GET/v1 | Retrieve aggregate alerts values based on the matched filter
[**aggregate_allow_list**](FalconCompleteDashboardApi.md#aggregate_allow_list) | **POST** /falcon-complete-dashboards/aggregates/allowlist/GET/v1 | Retrieve aggregate allowlist ticket values based on the matched filter
[**aggregate_block_list**](FalconCompleteDashboardApi.md#aggregate_block_list) | **POST** /falcon-complete-dashboards/aggregates/blocklist/GET/v1 | Retrieve aggregate blocklist ticket values based on the matched filter
[**aggregate_detections**](FalconCompleteDashboardApi.md#aggregate_detections) | **POST** /falcon-complete-dashboards/aggregates/detects/GET/v1 | Retrieve aggregate detection values based on the matched filter
[**aggregate_device_count_collection**](FalconCompleteDashboardApi.md#aggregate_device_count_collection) | **POST** /falcon-complete-dashboards/aggregates/devicecount-collections/GET/v1 | Retrieve aggregate host/devices count based on the matched filter
[**aggregate_escalations**](FalconCompleteDashboardApi.md#aggregate_escalations) | **POST** /falcon-complete-dashboards/aggregates/escalations/GET/v1 | Retrieve aggregate escalation ticket values based on the matched filter
[**aggregate_fc_incidents**](FalconCompleteDashboardApi.md#aggregate_fc_incidents) | **POST** /falcon-complete-dashboards/aggregates/incidents/GET/v1 | Retrieve aggregate incident values based on the matched filter
[**aggregate_prevention_policy**](FalconCompleteDashboardApi.md#aggregate_prevention_policy) | **POST** /falcon-complete-dashboards/aggregates/prevention-policies/v1 | Retrieve prevention policies aggregate values based on the matched filter
[**aggregate_remediations**](FalconCompleteDashboardApi.md#aggregate_remediations) | **POST** /falcon-complete-dashboards/aggregates/remediations/GET/v1 | Retrieve aggregate remediation ticket values based on the matched filter
[**aggregate_sensor_update_policy**](FalconCompleteDashboardApi.md#aggregate_sensor_update_policy) | **POST** /falcon-complete-dashboards/aggregates/sensor-update-policies/v1 | Retrieve sensor update policies aggregate values
[**aggregate_support_issues**](FalconCompleteDashboardApi.md#aggregate_support_issues) | **POST** /falcon-complete-dashboards/aggregates/support-issues/v1 | Retrieve aggregate support issue ticket values based on the matched filter
[**aggregate_total_device_counts**](FalconCompleteDashboardApi.md#aggregate_total_device_counts) | **POST** /falcon-complete-dashboards/aggregates/total-device-counts/v1 | Retrieve aggregate total host/devices based on the matched filter
[**get_device_count_collection_queries_by_filter**](FalconCompleteDashboardApi.md#get_device_count_collection_queries_by_filter) | **GET** /falcon-complete-dashboards/queries/devicecount-collections/v1 | Retrieve device count collection Ids that match the provided FQL filter, criteria with scrolling enabled
[**query_alert_ids_by_filter**](FalconCompleteDashboardApi.md#query_alert_ids_by_filter) | **GET** /falcon-complete-dashboards/queries/alerts/v1 | Retrieve Alerts Ids that match the provided FQL filter criteria with scrolling enabled
[**query_allow_list_filter**](FalconCompleteDashboardApi.md#query_allow_list_filter) | **GET** /falcon-complete-dashboards/queries/allowlist/v1 | Retrieve allowlist tickets that match the provided filter criteria with scrolling enabled
[**query_block_list_filter**](FalconCompleteDashboardApi.md#query_block_list_filter) | **GET** /falcon-complete-dashboards/queries/blocklist/v1 | Retrieve block listtickets that match the provided filter criteria with scrolling enabled
[**query_detection_ids_by_filter**](FalconCompleteDashboardApi.md#query_detection_ids_by_filter) | **GET** /falcon-complete-dashboards/queries/detects/v1 | Retrieve DetectionsIds that match the provided FQL filter, criteria with scrolling enabled
[**query_escalations_filter**](FalconCompleteDashboardApi.md#query_escalations_filter) | **GET** /falcon-complete-dashboards/queries/escalations/v1 | Retrieve escalation tickets that match the provided filter criteria with scrolling enabled
[**query_incident_ids_by_filter**](FalconCompleteDashboardApi.md#query_incident_ids_by_filter) | **GET** /falcon-complete-dashboards/queries/incidents/v1 | Retrieve incidents that match the provided filter criteria with scrolling enabled
[**query_remediations_filter**](FalconCompleteDashboardApi.md#query_remediations_filter) | **GET** /falcon-complete-dashboards/queries/remediations/v1 | Retrieve remediation tickets that match the provided filter criteria with scrolling enabled

## aggregate_alerts

> models::MsaPeriodAggregatesResponse aggregate_alerts(body)
Retrieve aggregate alerts values based on the matched filter

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

## aggregate_allow_list

> models::MsaPeriodAggregatesResponse aggregate_allow_list(body)
Retrieve aggregate allowlist ticket values based on the matched filter

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

## aggregate_block_list

> models::MsaPeriodAggregatesResponse aggregate_block_list(body)
Retrieve aggregate blocklist ticket values based on the matched filter

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

## aggregate_detections

> models::MsaPeriodAggregatesResponse aggregate_detections(body)
Retrieve aggregate detection values based on the matched filter

Fields allowed to aggregate on:  - indexed_time   - created_time   - detect_time   - ldt   - cid   - aid   - platform_name   - os_version   - device_tags   - host_name   - status   - severity   - adversary_ids   - behavior_ids   - behavior_names   - num_blocked_processes   - num_quarantined_files   - pattern_ids   - first_behavior_time   - last_behavior_time   - show_in_ui   - seconds_to_triaged   - seconds_to_resolved   - assigned_to_uid   - public_tags   - vertical_tags

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

## aggregate_device_count_collection

> models::MsaPeriodAggregatesResponse aggregate_device_count_collection(body)
Retrieve aggregate host/devices count based on the matched filter

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

## aggregate_escalations

> models::MsaPeriodAggregatesResponse aggregate_escalations(body)
Retrieve aggregate escalation ticket values based on the matched filter

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

## aggregate_fc_incidents

> models::MsaPeriodAggregatesResponse aggregate_fc_incidents(body)
Retrieve aggregate incident values based on the matched filter

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

## aggregate_prevention_policy

> models::MsaPeriodAggregatesResponse aggregate_prevention_policy(body)
Retrieve prevention policies aggregate values based on the matched filter

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

## aggregate_remediations

> models::MsaPeriodAggregatesResponse aggregate_remediations(body)
Retrieve aggregate remediation ticket values based on the matched filter

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

## aggregate_sensor_update_policy

> models::MsaPeriodAggregatesResponse aggregate_sensor_update_policy(body)
Retrieve sensor update policies aggregate values

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

## aggregate_support_issues

> models::MsaPeriodAggregatesResponse aggregate_support_issues(body)
Retrieve aggregate support issue ticket values based on the matched filter

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

## aggregate_total_device_counts

> models::MsaPeriodAggregatesResponse aggregate_total_device_counts(body)
Retrieve aggregate total host/devices based on the matched filter

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

## get_device_count_collection_queries_by_filter

> models::MsaspecPeriodQueryResponse get_device_count_collection_queries_by_filter(limit, sort, filter, offset)
Retrieve device count collection Ids that match the provided FQL filter, criteria with scrolling enabled

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_alert_ids_by_filter

> models::MsaspecPeriodQueryResponse query_alert_ids_by_filter(limit, sort, filter, offset)
Retrieve Alerts Ids that match the provided FQL filter criteria with scrolling enabled

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_allow_list_filter

> models::MsaspecPeriodQueryResponse query_allow_list_filter(limit, sort, filter, offset)
Retrieve allowlist tickets that match the provided filter criteria with scrolling enabled

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_block_list_filter

> models::MsaspecPeriodQueryResponse query_block_list_filter(limit, sort, filter, offset)
Retrieve block listtickets that match the provided filter criteria with scrolling enabled

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_detection_ids_by_filter

> models::MsaspecPeriodQueryResponse query_detection_ids_by_filter(limit, sort, filter, offset)
Retrieve DetectionsIds that match the provided FQL filter, criteria with scrolling enabled

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_escalations_filter

> models::MsaspecPeriodQueryResponse query_escalations_filter(limit, sort, filter, offset)
Retrieve escalation tickets that match the provided filter criteria with scrolling enabled

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_incident_ids_by_filter

> models::MsaspecPeriodQueryResponse query_incident_ids_by_filter(limit, sort, filter, offset)
Retrieve incidents that match the provided filter criteria with scrolling enabled

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_remediations_filter

> models::MsaspecPeriodQueryResponse query_remediations_filter(limit, sort, filter, offset)
Retrieve remediation tickets that match the provided filter criteria with scrolling enabled

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |
**sort** | Option<**String**> | The property to sort on, followed by a dot (.), followed by the sort direction, either \"asc\" or \"desc\". |  |
**filter** | Option<**String**> | Optional filter and sort criteria in the form of an FQL query. For more information about FQL queries, see [our FQL documentation in Falcon](https://falcon.crowdstrike.com/support/documentation/45/falcon-query-language-feature-guide). |  |
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
