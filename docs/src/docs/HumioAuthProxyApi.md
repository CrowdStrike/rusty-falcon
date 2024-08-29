# \HumioAuthProxyApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lookup_from_package_v1**](HumioAuthProxyApi.md#get_lookup_from_package_v1) | **GET** /humio/api/v1/repositories/{repository}/files/{package}/{filename} | Download lookup file in package from NGSIEM
[**get_lookup_from_package_with_namespace_v1**](HumioAuthProxyApi.md#get_lookup_from_package_with_namespace_v1) | **GET** /humio/api/v1/repositories/{repository}/files/{namespace}/{package}/{filename} | Download lookup file in namespaced package from NGSIEM
[**get_lookup_v1**](HumioAuthProxyApi.md#get_lookup_v1) | **GET** /humio/api/v1/repositories/{repository}/files/{filename} | Download lookup file from NGSIEM
[**upload_lookup_v1**](HumioAuthProxyApi.md#upload_lookup_v1) | **POST** /humio/api/v1/repositories/{repository}/files | Upload file to NGSIEM

## get_lookup_from_package_v1

> get_lookup_from_package_v1(repository, package, filename)
Download lookup file in package from NGSIEM

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | name of repository | [required] |
**package** | **String** | name of package | [required] |
**filename** | **String** | name of lookup file | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_lookup_from_package_with_namespace_v1

> get_lookup_from_package_with_namespace_v1(repository, namespace, package, filename)
Download lookup file in namespaced package from NGSIEM

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | name of repository | [required] |
**namespace** | **String** | name of namespace | [required] |
**package** | **String** | name of package | [required] |
**filename** | **String** | name of lookup file | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_lookup_v1

> get_lookup_v1(repository, filename)
Download lookup file from NGSIEM

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | name of repository | [required] |
**filename** | **String** | name of lookup file | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## upload_lookup_v1

> upload_lookup_v1(repository)
Upload file to NGSIEM

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | name of repository | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
