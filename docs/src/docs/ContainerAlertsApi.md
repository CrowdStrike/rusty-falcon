# ContainerAlertsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_container_alerts_count**](ContainerAlertsApi.md#read_container_alerts_count) | **GET** /container-security/aggregates/container-alerts/count/v1 | Search Container Alerts by the provided search criteria
[**read_container_alerts_count_by_severity**](ContainerAlertsApi.md#read_container_alerts_count_by_severity) | **GET** /container-security/aggregates/container-alerts/count-by-severity/v1 | Get Container Alerts counts by severity
[**search_and_read_container_alerts**](ContainerAlertsApi.md#search_and_read_container_alerts) | **GET** /container-security/combined/container-alerts/v1 | Search Container Alerts by the provided search criteria

## read_container_alerts_count

> models::AlertsPeriodContainerAlertsCountValue read_container_alerts_count(filter)
Search Container Alerts by the provided search criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Container Alerts using a query in Falcon Query Language (FQL). Supported filters:  cid,container_id,last_seen |  |

### Return type

[**models::AlertsPeriodContainerAlertsCountValue**](alerts.containerAlertsCountValue.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_container_alerts_count_by_severity

> models::AlertsPeriodContainerAlertsCountValue read_container_alerts_count_by_severity(filter)
Get Container Alerts counts by severity

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Container Alerts using a query in Falcon Query Language (FQL). Supported filters: cid,container_id,last_seen |  |

### Return type

[**models::AlertsPeriodContainerAlertsCountValue**](alerts.containerAlertsCountValue.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## search_and_read_container_alerts

> models::AlertsPeriodContainerAlertsEntityResponse search_and_read_container_alerts(filter, limit, offset, sort)
Search Container Alerts by the provided search criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Container Alerts using a query in Falcon Query Language (FQL). Supported filters:  cid,container_id,last_seen,name,severity |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. |  |

### Return type

[**models::AlertsPeriodContainerAlertsEntityResponse**](alerts.ContainerAlertsEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
