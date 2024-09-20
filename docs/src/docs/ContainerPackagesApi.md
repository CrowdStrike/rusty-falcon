# \ContainerPackagesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_packages_by_fixable_vuln_count**](ContainerPackagesApi.md#read_packages_by_fixable_vuln_count) | **GET** /container-security/combined/packages/app-by-fixable-vulnerability-count/v1 | Retrieve top x app packages with the most fixable vulnerabilities
[**read_packages_by_vuln_count**](ContainerPackagesApi.md#read_packages_by_vuln_count) | **GET** /container-security/combined/packages/by-vulnerability-count/v1 | Retrieve top x packages with the most vulnerabilities
[**read_packages_combined**](ContainerPackagesApi.md#read_packages_combined) | **GET** /container-security/combined/packages/v1 | Retrieve packages identified by the provided filter criteria
[**read_packages_combined_export**](ContainerPackagesApi.md#read_packages_combined_export) | **GET** /container-security/combined/packages/export/v1 | Retrieve packages identified by the provided filter criteria for the purpose of export
[**read_packages_count_by_zero_day**](ContainerPackagesApi.md#read_packages_count_by_zero_day) | **GET** /container-security/aggregates/packages/count-by-zero-day/v1 | Retrieve packages count affected by zero day vulnerabilities

## read_packages_by_fixable_vuln_count

> models::PackagesPeriodApiPackagesByVulnCount read_packages_by_fixable_vuln_count(filter, limit, offset)
Retrieve top x app packages with the most fixable vulnerabilities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter packages using a query in Falcon Query Language (FQL). Supported filters: cid,container_id,cveid,fix_status,image_digest,license,package_name_version,severity,type,vulnerability_count |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::PackagesPeriodApiPackagesByVulnCount**](packages.apiPackagesByVulnCount.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_packages_by_vuln_count

> models::PackagesPeriodApiPackagesByVulnCount read_packages_by_vuln_count(filter, limit, offset)
Retrieve top x packages with the most vulnerabilities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter packages using a query in Falcon Query Language (FQL). Supported filters: cid,container_id,cveid,fix_status,image_digest,license,package_name_version,severity,type,vulnerability_count |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::PackagesPeriodApiPackagesByVulnCount**](packages.apiPackagesByVulnCount.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_packages_combined

> models::PackagesPeriodApiCombinedPackage read_packages_combined(filter, only_zero_day_affected, limit, offset, sort)
Retrieve packages identified by the provided filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter packages using a query in Falcon Query Language (FQL). Supported filters:  cid,container_id,cveid,fix_status,image_digest,license,package_name_version,severity,type,vulnerability_count |  |
**only_zero_day_affected** | Option<**bool**> | (true/false) load zero day affected packages, default is false |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. Supported columns:  [license package_name_version type] |  |

### Return type

[**models::PackagesPeriodApiCombinedPackage**](packages.apiCombinedPackage.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_packages_combined_export

> models::PackagesPeriodApiCombinedPackageExport read_packages_combined_export(filter, only_zero_day_affected, limit, offset, sort)
Retrieve packages identified by the provided filter criteria for the purpose of export

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter packages using a query in Falcon Query Language (FQL). Supported filters:  cid,container_id,cveid,fix_status,image_digest,license,package_name_version,severity,type,vulnerability_count |  |
**only_zero_day_affected** | Option<**bool**> | (true/false) load zero day affected packages, default is false |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. Supported columns:  [license package_name_version type] |  |

### Return type

[**models::PackagesPeriodApiCombinedPackageExport**](packages.apiCombinedPackageExport.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_packages_count_by_zero_day

> models::CommonPeriodCountResponse read_packages_count_by_zero_day(filter)
Retrieve packages count affected by zero day vulnerabilities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter packages using a query in Falcon Query Language (FQL). Supported filters: cid |  |

### Return type

[**models::CommonPeriodCountResponse**](common.CountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
