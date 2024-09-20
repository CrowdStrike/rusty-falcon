# \DatascannerApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_data_scanner_tasks**](DatascannerApi.md#get_data_scanner_tasks) | **GET** /data-security-dspm/entities/scanner-tasks/v1 |
[**get_image_registry_credentials**](DatascannerApi.md#get_image_registry_credentials) | **GET** /data-security-dspm/entities/image-registry-credentials/v1 |
[**update_data_scanner_tasks**](DatascannerApi.md#update_data_scanner_tasks) | **PATCH** /data-security-dspm/entities/scanner-tasks/v1 |

## get_data_scanner_tasks

> std::collections::HashMap<String, String> get_data_scanner_tasks(x_scanner_id)

- GetDataScannerTasks is a route for the data scanner to fetch pending tasks

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_scanner_id** | **String** | ID of the data scanner | [required] |

### Return type

**std::collections::HashMap<String, String>**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_image_registry_credentials

> std::collections::HashMap<String, String> get_image_registry_credentials()

- GetImageRegistryCredentials is a route that gets credentials in order to fetch the data scanner image.

### Parameters

This endpoint does not need any parameter.

### Return type

**std::collections::HashMap<String, String>**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_data_scanner_tasks

> std::collections::HashMap<String, String> update_data_scanner_tasks(x_scanner_id, x_machine_id)

- PatchDataScannerTasks is a route for the data scanner to report back on tasks statuses

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_scanner_id** | **String** | ID of the data scanner | [required] |
**x_machine_id** | **String** | Provider ID of machine | [required] |

### Return type

**std::collections::HashMap<String, String>**

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
