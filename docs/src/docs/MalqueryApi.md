# \MalqueryApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_mal_query_download_v1**](MalqueryApi.md#get_mal_query_download_v1) | **GET** /malquery/entities/download-files/v1 | Download a file indexed by MalQuery. Specify the file using its SHA256. Only one file is supported at this time
[**get_mal_query_entities_samples_fetch_v1**](MalqueryApi.md#get_mal_query_entities_samples_fetch_v1) | **GET** /malquery/entities/samples-fetch/v1 | Fetch a zip archive with password 'infected' containing the samples. Call this once the /entities/samples-multidownload request has finished processing
[**get_mal_query_metadata_v1**](MalqueryApi.md#get_mal_query_metadata_v1) | **GET** /malquery/entities/metadata/v1 | Retrieve indexed files metadata by their hash
[**get_mal_query_quotas_v1**](MalqueryApi.md#get_mal_query_quotas_v1) | **GET** /malquery/aggregates/quotas/v1 | Get information about search and download quotas in your environment
[**get_mal_query_request_v1**](MalqueryApi.md#get_mal_query_request_v1) | **GET** /malquery/entities/requests/v1 | Check the status and results of an asynchronous request, such as hunt or exact-search. Supports a single request id at this time.
[**post_mal_query_entities_samples_multidownload_v1**](MalqueryApi.md#post_mal_query_entities_samples_multidownload_v1) | **POST** /malquery/entities/samples-multidownload/v1 | Schedule samples for download. Use the result id with the /request endpoint to check if the download is ready after which you can call the /entities/samples-fetch to get the zip
[**post_mal_query_exact_search_v1**](MalqueryApi.md#post_mal_query_exact_search_v1) | **POST** /malquery/queries/exact-search/v1 | Search Falcon MalQuery for a combination of hex patterns and strings in order to identify samples based upon file content at byte level granularity. You can filter results on criteria such as file type, file size and first seen date. Returns a request id which can be used with the /request endpoint
[**post_mal_query_fuzzy_search_v1**](MalqueryApi.md#post_mal_query_fuzzy_search_v1) | **POST** /malquery/combined/fuzzy-search/v1 | Search Falcon MalQuery quickly, but with more potential for false positives. Search for a combination of hex patterns and strings in order to identify samples based upon file content at byte level granularity.
[**post_mal_query_hunt_v1**](MalqueryApi.md#post_mal_query_hunt_v1) | **POST** /malquery/queries/hunt/v1 | Schedule a YARA-based search for execution. Returns a request id which can be used with the /request endpoint

## get_mal_query_download_v1

> get_mal_query_download_v1(ids)
Download a file indexed by MalQuery. Specify the file using its SHA256. Only one file is supported at this time

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The file SHA256. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_mal_query_entities_samples_fetch_v1

> get_mal_query_entities_samples_fetch_v1(ids)
Fetch a zip archive with password 'infected' containing the samples. Call this once the /entities/samples-multidownload request has finished processing

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Multidownload job id | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_mal_query_metadata_v1

> models::MalqueryPeriodSampleMetadataResponse get_mal_query_metadata_v1(ids)
Retrieve indexed files metadata by their hash

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The file SHA256. | [required] |

### Return type

[**models::MalqueryPeriodSampleMetadataResponse**](malquery.SampleMetadataResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_mal_query_quotas_v1

> models::MalqueryPeriodRateLimitsResponse get_mal_query_quotas_v1()
Get information about search and download quotas in your environment

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MalqueryPeriodRateLimitsResponse**](malquery.RateLimitsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_mal_query_request_v1

> models::MalqueryPeriodRequestResponse get_mal_query_request_v1(ids)
Check the status and results of an asynchronous request, such as hunt or exact-search. Supports a single request id at this time.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | Identifier of a MalQuery request | [required] |

### Return type

[**models::MalqueryPeriodRequestResponse**](malquery.RequestResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_mal_query_entities_samples_multidownload_v1

> models::MalqueryPeriodExternalQueryResponse post_mal_query_entities_samples_multidownload_v1(body)
Schedule samples for download. Use the result id with the /request endpoint to check if the download is ready after which you can call the /entities/samples-fetch to get the zip

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MalqueryPeriodMultiDownloadRequestV1**](MalqueryPeriodMultiDownloadRequestV1.md) | Download request. See model for more details. | [required] |

### Return type

[**models::MalqueryPeriodExternalQueryResponse**](malquery.ExternalQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_mal_query_exact_search_v1

> models::MalqueryPeriodExternalQueryResponse post_mal_query_exact_search_v1(body)
Search Falcon MalQuery for a combination of hex patterns and strings in order to identify samples based upon file content at byte level granularity. You can filter results on criteria such as file type, file size and first seen date. Returns a request id which can be used with the /request endpoint

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MalqueryPeriodExternalExactSearchParametersV1**](MalqueryPeriodExternalExactSearchParametersV1.md) | Exact search parameters. See model for more details. | [required] |

### Return type

[**models::MalqueryPeriodExternalQueryResponse**](malquery.ExternalQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_mal_query_fuzzy_search_v1

> models::MalqueryPeriodFuzzySearchResponse post_mal_query_fuzzy_search_v1(body)
Search Falcon MalQuery quickly, but with more potential for false positives. Search for a combination of hex patterns and strings in order to identify samples based upon file content at byte level granularity.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MalqueryPeriodFuzzySearchParametersV1**](MalqueryPeriodFuzzySearchParametersV1.md) | Fuzzy search parameters. See model for more details. | [required] |

### Return type

[**models::MalqueryPeriodFuzzySearchResponse**](malquery.FuzzySearchResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_mal_query_hunt_v1

> models::MalqueryPeriodExternalQueryResponse post_mal_query_hunt_v1(body)
Schedule a YARA-based search for execution. Returns a request id which can be used with the /request endpoint

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MalqueryPeriodExternalHuntParametersV1**](MalqueryPeriodExternalHuntParametersV1.md) | Hunt parameters. See model for more details. | [required] |

### Return type

[**models::MalqueryPeriodExternalQueryResponse**](malquery.ExternalQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
