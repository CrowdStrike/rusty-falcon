# \MobileEnrollmentApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**request_device_enrollment_v3**](MobileEnrollmentApi.md#request_device_enrollment_v3) | **POST** /enrollments/entities/details/v3 | Trigger on-boarding process for a mobile device
[**request_device_enrollment_v4**](MobileEnrollmentApi.md#request_device_enrollment_v4) | **POST** /enrollments/entities/details/v4 | Trigger on-boarding process for a mobile device

## request_device_enrollment_v3

> models::ApiPeriodPostEnrollmentDetailsResponse request_device_enrollment_v3(body, action_name, filter)
Trigger on-boarding process for a mobile device

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodPostEnrollmentDetails**](ApiPeriodPostEnrollmentDetails.md) |  | [required] |
**action_name** | Option<**String**> | Action to perform |  |
**filter** | Option<**String**> | FQL filter |  |

### Return type

[**models::ApiPeriodPostEnrollmentDetailsResponse**](api.postEnrollmentDetailsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## request_device_enrollment_v4

> models::ApiPeriodPostEnrollmentDetailsResponse request_device_enrollment_v4(body, action_name, filter)
Trigger on-boarding process for a mobile device

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodPostEnrollmentDetailsV4**](ApiPeriodPostEnrollmentDetailsV4.md) |  | [required] |
**action_name** | Option<**String**> | Action to perform |  |
**filter** | Option<**String**> | FQL filter |  |

### Return type

[**models::ApiPeriodPostEnrollmentDetailsResponse**](api.postEnrollmentDetailsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
