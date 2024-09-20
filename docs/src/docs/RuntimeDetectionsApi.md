# \RuntimeDetectionsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_runtime_detections_combined_v2**](RuntimeDetectionsApi.md#get_runtime_detections_combined_v2) | **GET** /container-security/combined/runtime-detections/v2 | Retrieve container runtime detections by the provided search criteria

## get_runtime_detections_combined_v2

> models::RuntimedetectionsPeriodDetectionsEntityResponse get_runtime_detections_combined_v2(filter, limit, offset, sort)
Retrieve container runtime detections by the provided search criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter Container Runtime Detections using a query in Falcon Query Language (FQL). Supported filters:  action_taken,aid,cid,cloud,cluster_name,command_line,computer_name,container_id,detect_timestamp,detection_description,detection_id,file_name,file_path,host_id,host_type,image_id,name,namespace,pod_name,severity,tactic |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The field to sort the records on. |  |

### Return type

[**models::RuntimedetectionsPeriodDetectionsEntityResponse**](runtimedetections.DetectionsEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
