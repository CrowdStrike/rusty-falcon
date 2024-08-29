# \UnidentifiedContainersApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_unidentified_containers_by_date_range_count**](UnidentifiedContainersApi.md#read_unidentified_containers_by_date_range_count) | **GET** /container-security/aggregates/unidentified-containers/count-by-date/v1 | Returns the count of Unidentified Containers over the last 7 days
[**read_unidentified_containers_count**](UnidentifiedContainersApi.md#read_unidentified_containers_count) | **GET** /container-security/aggregates/unidentified-containers/count/v1 | Returns the total count of Unidentified Containers over a time period
[**search_and_read_unidentified_containers**](UnidentifiedContainersApi.md#search_and_read_unidentified_containers) | **GET** /container-security/combined/unidentified-containers/v1 | Search Unidentified Containers by the provided search criteria

## read_unidentified_containers_by_date_range_count

> models::ModelsPeriodAggregateValuesByFieldResponse read_unidentified_containers_by_date_range_count(filter)
Returns the count of Unidentified Containers over the last 7 days

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter Unidentified Containers using a query in Falcon Query Language (FQL). Supported filters:  assessed_images_count,cid,cluster_name,containers_impacted_count,detections_count,image_assessment_detections_count,last_seen,namespace,node_name,severity,unassessed_images_count,visible_to_k8s |  |

### Return type

[**models::ModelsPeriodAggregateValuesByFieldResponse**](models.AggregateValuesByFieldResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_unidentified_containers_count

> models::UnidentifiedcontainersPeriodUnidentifiedContainersCountValue read_unidentified_containers_count(filter)
Returns the total count of Unidentified Containers over a time period

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter Unidentified Containers using a query in Falcon Query Language (FQL). Supported filters:  assessed_images_count,cid,cluster_name,containers_impacted_count,detections_count,image_assessment_detections_count,last_seen,namespace,node_name,severity,unassessed_images_count,visible_to_k8s |  |

### Return type

[**models::UnidentifiedcontainersPeriodUnidentifiedContainersCountValue**](unidentifiedcontainers.unidentifiedContainersCountValue.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## search_and_read_unidentified_containers

> models::UnidentifiedcontainersPeriodUnidentifiedContainerApiResponse search_and_read_unidentified_containers(filter, limit, offset, sort)
Search Unidentified Containers by the provided search criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Search Unidentified Containers using a query in Falcon Query Language (FQL). Supported filters:  assessed_images_count,cid,cluster_name,containers_impacted_count,detections_count,image_assessment_detections_count,last_seen,namespace,node_name,severity,unassessed_images_count,visible_to_k8s |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. |  |

### Return type

[**models::UnidentifiedcontainersPeriodUnidentifiedContainerApiResponse**](unidentifiedcontainers.UnidentifiedContainerAPIResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
