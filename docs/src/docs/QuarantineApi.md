# \QuarantineApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**action_update_count**](QuarantineApi.md#action_update_count) | **GET** /quarantine/aggregates/action-update-count/v1 | Returns count of potentially affected quarantined files for each action.
[**get_aggregate_files**](QuarantineApi.md#get_aggregate_files) | **POST** /quarantine/aggregates/quarantined-files/GET/v1 | Get quarantine file aggregates as specified via json in request body.
[**get_quarantine_files**](QuarantineApi.md#get_quarantine_files) | **POST** /quarantine/entities/quarantined-files/GET/v1 | Get quarantine file metadata for specified ids.
[**query_quarantine_files**](QuarantineApi.md#query_quarantine_files) | **GET** /quarantine/queries/quarantined-files/v1 | Get quarantine file ids that match the provided filter criteria.
[**update_qf_by_query**](QuarantineApi.md#update_qf_by_query) | **PATCH** /quarantine/queries/quarantined-files/v1 | Apply quarantine file actions by query.
[**update_quarantined_detects_by_ids**](QuarantineApi.md#update_quarantined_detects_by_ids) | **PATCH** /quarantine/entities/quarantined-files/v1 | Apply action by quarantine file ids

## action_update_count

> models::MsaPeriodAggregatesResponse action_update_count(filter)
Returns count of potentially affected quarantined files for each action.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | FQL specifying filter parameters. | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_aggregate_files

> models::MsaPeriodAggregatesResponse get_aggregate_files(body)
Get quarantine file aggregates as specified via json in request body.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodAggregateQueryRequest**](MsaPeriodAggregateQueryRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodAggregatesResponse**](msa.AggregatesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_quarantine_files

> models::DomainPeriodMsaQfResponse get_quarantine_files(body)
Get quarantine file metadata for specified ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DomainPeriodMsaQfResponse**](domain.MsaQfResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_quarantine_files

> models::MsaPeriodQueryResponse query_quarantine_files(offset, limit, sort, filter, q)
Get quarantine file ids that match the provided filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | Starting index of overall result set from which to return ids. |  |
**limit** | Option<**i32**> | Number of ids to return. |  |
**sort** | Option<**String**> | Possible order by fields: hostname, username, date_updated, date_created, paths.path, state, paths.state. Ex: 'date_created|asc'. |  |
**filter** | Option<**String**> | FQL query specifying the filter parameters. Special value '*' means to not filter on anything. Filter term criteria: status, adversary_id, device.device_id, device.country, device.hostname, behaviors.behavior_id, behaviors.ioc_type, behaviors.ioc_value, behaviors.username, behaviors.tree_root_hash. Filter range criteria:, max_severity, max_confidence, first_behavior, last_behavior. |  |
**q** | Option<**String**> | Match phrase_prefix query criteria; included fields:_all (all filter string fields), sha256, state, paths.path, paths.state, hostname, username, date_updated, date_created. |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_qf_by_query

> models::MsaPeriodReplyMetaOnly update_qf_by_query(body)
Apply quarantine file actions by query.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodQueriesPatchRequest**](DomainPeriodQueriesPatchRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_quarantined_detects_by_ids

> models::MsaPeriodReplyMetaOnly update_quarantined_detects_by_ids(body)
Apply action by quarantine file ids

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodEntitiesPatchRequest**](DomainPeriodEntitiesPatchRequest.md) |  | [required] |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
