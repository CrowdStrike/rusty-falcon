# \ContainerDetectionsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_combined_detections**](ContainerDetectionsApi.md#read_combined_detections) | **GET** /container-security/combined/detections/v1 | Retrieve image assessment detections identified by the provided filter criteria
[**read_detections**](ContainerDetectionsApi.md#read_detections) | **GET** /container-security/entities/detections/v1 | Retrieve image assessment detection entities identified by the provided filter criteria
[**read_detections_count**](ContainerDetectionsApi.md#read_detections_count) | **GET** /container-security/aggregates/detections/count/v1 | Aggregate count of detections
[**read_detections_count_by_severity**](ContainerDetectionsApi.md#read_detections_count_by_severity) | **GET** /container-security/aggregates/detections/count-by-severity/v1 | Aggregate counts of detections by severity
[**read_detections_count_by_type**](ContainerDetectionsApi.md#read_detections_count_by_type) | **GET** /container-security/aggregates/detections/count-by-type/v1 | Aggregate counts of detections by detection type
[**search_detections**](ContainerDetectionsApi.md#search_detections) | **GET** /container-security/queries/detections/v1 | Retrieve image assessment detection entities identified by the provided filter criteria

## read_combined_detections

> models::DetectionsPeriodApiCombinedDetections read_combined_detections(filter, limit, offset, sort)
Retrieve image assessment detections identified by the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  cid,container_id,detection_type,id,image_digest,image_id,image_registry,image_repository,image_tag,name,severity |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. Supported columns:  [containers_impacted detection_name detection_severity detection_type images_impacted last_detected] |  |

### Return type

[**models::DetectionsPeriodApiCombinedDetections**](detections.apiCombinedDetections.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_detections

> models::DetectionsPeriodApiAssessmentDetections read_detections(filter, limit, offset)
Retrieve image assessment detection entities identified by the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  cid,detection_type,image_registry,image_repository,image_tag |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::DetectionsPeriodApiAssessmentDetections**](detections.apiAssessmentDetections.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_detections_count

> models::DetectionsPeriodApiDetectionsCount read_detections_count(filter)
Aggregate count of detections

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  cid,container_id,detection_type,id,image_digest,image_id,image_registry,image_repository,image_tag,name,severity |  |

### Return type

[**models::DetectionsPeriodApiDetectionsCount**](detections.apiDetectionsCount.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_detections_count_by_severity

> models::DetectionsPeriodApiDetectionsBySeverity read_detections_count_by_severity(filter)
Aggregate counts of detections by severity

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  cid,container_id,detection_type,id,image_digest,image_id,image_registry,image_repository,image_tag,name,severity |  |

### Return type

[**models::DetectionsPeriodApiDetectionsBySeverity**](detections.apiDetectionsBySeverity.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_detections_count_by_type

> models::DetectionsPeriodApiDetectionsByType read_detections_count_by_type(filter)
Aggregate counts of detections by detection type

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  cid,container_id,detection_type,id,image_digest,image_id,image_registry,image_repository,image_tag,name,severity |  |

### Return type

[**models::DetectionsPeriodApiDetectionsByType**](detections.apiDetectionsByType.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## search_detections

> models::CommonPeriodGenericEntityResponseLeftSquareBracketStringRightSquareBracket search_detections(filter, limit, offset)
Retrieve image assessment detection entities identified by the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter images using a query in Falcon Query Language (FQL). Supported filters:  cid,container_id,detection_type,id,image_digest,image_id,image_registry,image_repository,image_tag,name,severity |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::CommonPeriodGenericEntityResponseLeftSquareBracketStringRightSquareBracket**](common.GenericEntityResponse[string].md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
