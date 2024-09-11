# \ContainerVulnerabilitiesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_combined_vulnerabilities**](ContainerVulnerabilitiesApi.md#read_combined_vulnerabilities) | **GET** /container-security/combined/vulnerabilities/v1 | Retrieve vulnerability and aggregate data filtered by the provided FQL
[**read_combined_vulnerabilities_details**](ContainerVulnerabilitiesApi.md#read_combined_vulnerabilities_details) | **GET** /container-security/combined/vulnerabilities/details/v1 | Retrieve vulnerability details related to an image
[**read_combined_vulnerabilities_info**](ContainerVulnerabilitiesApi.md#read_combined_vulnerabilities_info) | **GET** /container-security/combined/vulnerabilities/info/v1 | Retrieve vulnerability and package related info for this customer
[**read_vulnerabilities_by_image_count**](ContainerVulnerabilitiesApi.md#read_vulnerabilities_by_image_count) | **GET** /container-security/combined/vulnerabilities/by-image-count/v1 | Retrieve top x vulnerabilities with the most impacted images
[**read_vulnerabilities_publication_date**](ContainerVulnerabilitiesApi.md#read_vulnerabilities_publication_date) | **GET** /container-security/combined/vulnerabilities/by-published-date/v1 | Retrieve top x vulnerabilities with the most recent publication date
[**read_vulnerability_count**](ContainerVulnerabilitiesApi.md#read_vulnerability_count) | **GET** /container-security/aggregates/vulnerabilities/count/v1 | Aggregate count of vulnerabilities
[**read_vulnerability_count_by_actively_exploited**](ContainerVulnerabilitiesApi.md#read_vulnerability_count_by_actively_exploited) | **GET** /container-security/aggregates/vulnerabilities/count-by-actively-exploited/v1 | Aggregate count of vulnerabilities grouped by actively exploited
[**read_vulnerability_count_by_cps_rating**](ContainerVulnerabilitiesApi.md#read_vulnerability_count_by_cps_rating) | **GET** /container-security/aggregates/vulnerabilities/count-by-cps-rating/v1 | Aggregate count of vulnerabilities grouped by csp_rating
[**read_vulnerability_count_by_cvss_score**](ContainerVulnerabilitiesApi.md#read_vulnerability_count_by_cvss_score) | **GET** /container-security/aggregates/vulnerabilities/count-by-cvss-score/v1 | Aggregate count of vulnerabilities grouped by cvss score
[**read_vulnerability_count_by_severity**](ContainerVulnerabilitiesApi.md#read_vulnerability_count_by_severity) | **GET** /container-security/aggregates/vulnerabilities/count-by-severity/v1 | Aggregate count of vulnerabilities grouped by severity

## read_combined_vulnerabilities

> models::VulnerabilitiesPeriodApiCombinedVulnerability read_combined_vulnerabilities(filter, limit, offset, sort)
Retrieve vulnerability and aggregate data filtered by the provided FQL

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter vulnerabilities using a query in Falcon Query Language (FQL). Supported filters: base_os,cid,container_id,container_running_status,containers_impacted_range,cps_rating,cve_id,cvss_score,description,exploited_status,exploited_status_name,fix_status,image_digest,image_id,images_impacted_range,package_name_version,registry,repository,severity,tag |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |
**sort** | Option<**String**> | The fields to sort the records on. Supported columns:  [cps_current_rating cve_id cvss_score description images_impacted packages_impacted severity] |  |

### Return type

[**models::VulnerabilitiesPeriodApiCombinedVulnerability**](vulnerabilities.apiCombinedVulnerability.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_combined_vulnerabilities_details

> models::VulnerabilitiesPeriodApiCombinedVulnerabilityDetails read_combined_vulnerabilities_details(id, filter, limit, offset)
Retrieve vulnerability details related to an image

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Image UUID | [required] |
**filter** | Option<**String**> | Filter the vulnerabilities using a query in Falcon Query Language (FQL). Supported vulnerability filters: cid,cps_rating,cve_id,cvss_score,exploited_status,exploited_status_name,is_zero_day,remediation_available,severity |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::VulnerabilitiesPeriodApiCombinedVulnerabilityDetails**](vulnerabilities.apiCombinedVulnerabilityDetails.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_combined_vulnerabilities_info

> models::VulnerabilitiesPeriodApiCombinedVulnerabilityInfo read_combined_vulnerabilities_info(cve_id, limit, offset)
Retrieve vulnerability and package related info for this customer

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cve_id** | **String** | Vulnerability CVE ID | [required] |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::VulnerabilitiesPeriodApiCombinedVulnerabilityInfo**](vulnerabilities.apiCombinedVulnerabilityInfo.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_vulnerabilities_by_image_count

> models::VulnerabilitiesPeriodApiVulnByImageCount read_vulnerabilities_by_image_count(filter, limit, offset)
Retrieve top x vulnerabilities with the most impacted images

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter vulnerabilities using a query in Falcon Query Language (FQL). Supported filters: cid,cve_id,registry,repository,tag |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::VulnerabilitiesPeriodApiVulnByImageCount**](vulnerabilities.apiVulnByImageCount.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_vulnerabilities_publication_date

> models::VulnerabilitiesPeriodApiVulnByPublication read_vulnerabilities_publication_date(filter, limit, offset)
Retrieve top x vulnerabilities with the most recent publication date

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter vulnerabilities using a query in Falcon Query Language (FQL). Supported filters: cid,cve_id,registry,repository,tag |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::VulnerabilitiesPeriodApiVulnByPublication**](vulnerabilities.apiVulnByPublication.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_vulnerability_count

> models::VulnerabilitiesPeriodApiVulnCount read_vulnerability_count(filter, limit, offset)
Aggregate count of vulnerabilities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter vulnerabilities using a query in Falcon Query Language (FQL). Supported filters: base_os,cid,container_id,container_running_status,containers_impacted_range,cps_rating,cve_id,cvss_score,description,exploited_status,exploited_status_name,fix_status,image_digest,image_id,images_impacted_range,package_name_version,registry,repository,severity,tag |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::VulnerabilitiesPeriodApiVulnCount**](vulnerabilities.apiVulnCount.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_vulnerability_count_by_actively_exploited

> models::VulnerabilitiesPeriodApiVulnCountByActivelyExploited read_vulnerability_count_by_actively_exploited(filter, limit, offset)
Aggregate count of vulnerabilities grouped by actively exploited

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter vulnerabilities using a query in Falcon Query Language (FQL). Supported filters: base_os,cid,container_id,container_running_status,containers_impacted_range,cps_rating,cve_id,cvss_score,description,exploited_status,exploited_status_name,fix_status,image_digest,image_id,images_impacted_range,package_name_version,registry,repository,severity,tag |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::VulnerabilitiesPeriodApiVulnCountByActivelyExploited**](vulnerabilities.apiVulnCountByActivelyExploited.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_vulnerability_count_by_cps_rating

> models::VulnerabilitiesPeriodApiVulnCountByCspRating read_vulnerability_count_by_cps_rating(filter, limit, offset)
Aggregate count of vulnerabilities grouped by csp_rating

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter vulnerabilities using a query in Falcon Query Language (FQL). Supported filters: base_os,cid,container_id,container_running_status,containers_impacted_range,cps_rating,cve_id,cvss_score,description,exploited_status,exploited_status_name,fix_status,image_digest,image_id,images_impacted_range,package_name_version,registry,repository,severity,tag |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::VulnerabilitiesPeriodApiVulnCountByCspRating**](vulnerabilities.apiVulnCountByCSPRating.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_vulnerability_count_by_cvss_score

> models::VulnerabilitiesPeriodApiVulnCountByCvssScore read_vulnerability_count_by_cvss_score(filter, limit, offset)
Aggregate count of vulnerabilities grouped by cvss score

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter vulnerabilities using a query in Falcon Query Language (FQL). Supported filters: base_os,cid,container_id,container_running_status,containers_impacted_range,cps_rating,cve_id,cvss_score,description,exploited_status,exploited_status_name,fix_status,image_digest,image_id,images_impacted_range,package_name_version,registry,repository,severity,tag |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::VulnerabilitiesPeriodApiVulnCountByCvssScore**](vulnerabilities.apiVulnCountByCVSSScore.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_vulnerability_count_by_severity

> models::VulnerabilitiesPeriodApiVulnCountBySeverity read_vulnerability_count_by_severity(filter, limit, offset)
Aggregate count of vulnerabilities grouped by severity

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter vulnerabilities using a query in Falcon Query Language (FQL). Supported filters: base_os,cid,container_id,container_running_status,containers_impacted_range,cps_rating,cve_id,cvss_score,description,exploited_status,exploited_status_name,fix_status,image_digest,image_id,images_impacted_range,package_name_version,registry,repository,severity,tag |  |
**limit** | Option<**i32**> | The upper-bound on the number of records to retrieve. |  |
**offset** | Option<**i32**> | The offset from where to begin. |  |

### Return type

[**models::VulnerabilitiesPeriodApiVulnCountBySeverity**](vulnerabilities.apiVulnCountBySeverity.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
