# \SpotlightVulnerabilitiesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**combined_query_vulnerabilities**](SpotlightVulnerabilitiesApi.md#combined_query_vulnerabilities) | **GET** /spotlight/combined/vulnerabilities/v1 | Search for Vulnerabilities in your environment by providing an FQL filter and paging details. Returns a set of Vulnerability entities which match the filter criteria
[**get_remediations_v2**](SpotlightVulnerabilitiesApi.md#get_remediations_v2) | **GET** /spotlight/entities/remediations/v2 | Get details on remediation by providing one or more IDs
[**get_vulnerabilities**](SpotlightVulnerabilitiesApi.md#get_vulnerabilities) | **GET** /spotlight/entities/vulnerabilities/v2 | Get details on vulnerabilities by providing one or more IDs
[**query_vulnerabilities**](SpotlightVulnerabilitiesApi.md#query_vulnerabilities) | **GET** /spotlight/queries/vulnerabilities/v1 | Search for Vulnerabilities in your environment by providing an FQL filter and paging details. Returns a set of Vulnerability IDs which match the filter criteria

## combined_query_vulnerabilities

> models::DomainPeriodSpapiCombinedVulnerabilitiesResponse combined_query_vulnerabilities(filter, after, limit, sort, facet)
Search for Vulnerabilities in your environment by providing an FQL filter and paging details. Returns a set of Vulnerability entities which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | Filter items using a query in Falcon Query Language (FQL). Wildcards *and empty filter values are unsupported.    Available filter fields that supports match (~): N/A    Available filter fields that supports exact match: aid, cid, last_seen_within, status, cve.id, cve.is_cisa_kev, cve.remediation_level, cve.cps_rating, cve.exprt_rating, cve.exploit_status_to_include, cve.severity, cve.types, host_info.asset_criticality, host_info.asset_roles, host_info.internet_exposure, host_info.tags, host_info.groups, host_info.product_type_desc, host_info.platform_name, suppression_info.is_suppressed, suppression_info.reason    Available filter fields that supports wildcard (*): N/A    Available filter fields that supports range comparisons (>, <, >=, <=): created_timestamp, closed_timestamp, updated_timestamp     | [required] |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of items to return in this response (default: 100, max: 5000). Use with the after parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort vulnerabilities by their properties. Common sort options include:  <ul><li>updated_timestamp|asc</li><li>closed_timestamp|asc</li></ul> |  |
**facet** | Option<[**Vec<String>**](String.md)> | Select various details blocks to be returned for each vulnerability entity. Supported values:  <ul><li>host_info</li><li>remediation</li><li>cve</li><li>evaluation_logic</li></ul> |  |

### Return type

[**models::DomainPeriodSpapiCombinedVulnerabilitiesResponse**](domain.SPAPICombinedVulnerabilitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_remediations_v2

> models::DomainPeriodSpapiRemediationEntitiesResponseV2 get_remediations_v2(ids)
Get details on remediation by providing one or more IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more remediation IDs | [required] |

### Return type

[**models::DomainPeriodSpapiRemediationEntitiesResponseV2**](domain.SPAPIRemediationEntitiesResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_vulnerabilities

> models::DomainPeriodSpapiVulnerabilitiesEntitiesResponseV2 get_vulnerabilities(ids)
Get details on vulnerabilities by providing one or more IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more vulnerability IDs (max: 400). Find vulnerability IDs with GET /spotlight/queries/vulnerabilities/v1 | [required] |

### Return type

[**models::DomainPeriodSpapiVulnerabilitiesEntitiesResponseV2**](domain.SPAPIVulnerabilitiesEntitiesResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_vulnerabilities

> models::DomainPeriodSpapiQueryResponse query_vulnerabilities(filter, after, limit, sort)
Search for Vulnerabilities in your environment by providing an FQL filter and paging details. Returns a set of Vulnerability IDs which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | Filter items using a query in Falcon Query Language (FQL). Wildcards *and empty filter values are unsupported.    Available filter fields that supports match (~): N/A    Available filter fields that supports exact match: aid, cid, last_seen_within, status, cve.id, cve.is_cisa_kev, cve.remediation_level, cve.cps_rating, cve.exprt_rating, cve.exploit_status_to_include, cve.severity, cve.types, host_info.asset_criticality, host_info.asset_roles, host_info.internet_exposure, host_info.tags, host_info.groups, host_info.product_type_desc, host_info.platform_name, suppression_info.is_suppressed, suppression_info.reason    Available filter fields that supports wildcard (*): N/A    Available filter fields that supports range comparisons (>, <, >=, <=): created_timestamp, closed_timestamp, updated_timestamp     | [required] |
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of items to return in this response (default: 100, max: 400). Use with the after parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort vulnerabilities by their properties. Available sort options:  <ul><li>updated_timestamp|asc/desc</li><li>closed_timestamp|asc</li><li>updated_timestamp|asc/desc</li></ul>. Can be used in a format <field>|asc for ascending order or <field>|desc for descending order. |  |

### Return type

[**models::DomainPeriodSpapiQueryResponse**](domain.SPAPIQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
