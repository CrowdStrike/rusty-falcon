# \FalconContainerCliApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_image_vulnerabilities**](FalconContainerCliApi.md#read_image_vulnerabilities) | **POST** /image-assessment/combined/vulnerability-lookups/v1 | Retrieve known vulnerabilities for the provided image

## read_image_vulnerabilities

> models::CorePeriodEntitiesResponse read_image_vulnerabilities(body)
Retrieve known vulnerabilities for the provided image

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPeriodImageLookupRequest**](ApiPeriodImageLookupRequest.md) |  | [required] |

### Return type

[**models::CorePeriodEntitiesResponse**](core.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
