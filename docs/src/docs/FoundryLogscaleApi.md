# \FoundryLogscaleApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_saved_searches_dynamic_execute_alt_v1**](FoundryLogscaleApi.md#create_saved_searches_dynamic_execute_alt_v1) | **POST** /loggingapi/entities/saved-searches-dynamic-execute/v1 | Execute a dynamic saved search
[**create_saved_searches_dynamic_execute_v1**](FoundryLogscaleApi.md#create_saved_searches_dynamic_execute_v1) | **POST** /loggingapi/entities/saved-searches/execute-dynamic/v1 | Execute a dynamic saved search
[**create_saved_searches_execute_alt_v1**](FoundryLogscaleApi.md#create_saved_searches_execute_alt_v1) | **POST** /loggingapi/entities/saved-searches-execute/v1 | Execute a saved search
[**create_saved_searches_execute_v1**](FoundryLogscaleApi.md#create_saved_searches_execute_v1) | **POST** /loggingapi/entities/saved-searches/execute/v1 | Execute a saved search
[**create_saved_searches_ingest_alt_v1**](FoundryLogscaleApi.md#create_saved_searches_ingest_alt_v1) | **POST** /loggingapi/entities/saved-searches-ingest/v1 | Populate a saved search
[**create_saved_searches_ingest_v1**](FoundryLogscaleApi.md#create_saved_searches_ingest_v1) | **POST** /loggingapi/entities/saved-searches/ingest/v1 | Populate a saved search
[**get_saved_searches_execute_alt_v1**](FoundryLogscaleApi.md#get_saved_searches_execute_alt_v1) | **GET** /loggingapi/entities/saved-searches-execute/v1 | Get the results of a saved search
[**get_saved_searches_execute_v1**](FoundryLogscaleApi.md#get_saved_searches_execute_v1) | **GET** /loggingapi/entities/saved-searches/execute/v1 | Get the results of a saved search
[**get_saved_searches_job_results_download_alt_v1**](FoundryLogscaleApi.md#get_saved_searches_job_results_download_alt_v1) | **GET** /loggingapi/entities/saved-searches-job-results-download/v1 | Get the results of a saved search as a file
[**get_saved_searches_job_results_download_v1**](FoundryLogscaleApi.md#get_saved_searches_job_results_download_v1) | **GET** /loggingapi/entities/saved-searches/job-results-download/v1 | Get the results of a saved search as a file
[**ingest_data_async_v1**](FoundryLogscaleApi.md#ingest_data_async_v1) | **POST** /loggingapi/entities/data-ingestion/ingest-async/v1 | Asynchronously ingest data into the application repository
[**ingest_data_v1**](FoundryLogscaleApi.md#ingest_data_v1) | **POST** /loggingapi/entities/data-ingestion/ingest/v1 | Synchronously ingest data into the application repository
[**list_repos_v1**](FoundryLogscaleApi.md#list_repos_v1) | **GET** /loggingapi/combined/repos/v1 | Lists available repositories and views
[**list_view_v1**](FoundryLogscaleApi.md#list_view_v1) | **GET** /loggingapi/entities/views/v1 | List views

## create_saved_searches_dynamic_execute_alt_v1

> models::ApidomainPeriodQueryResponseWrapperV1 create_saved_searches_dynamic_execute_alt_v1(body, app_id, include_schema_generation, include_test_data, infer_json_types, match_response_schema, metadata, mode)
Execute a dynamic saved search

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApidomainPeriodDynamicExecuteSearchRequestV1**](ApidomainPeriodDynamicExecuteSearchRequestV1.md) |  | [required] |
**app_id** | Option<**String**> | Application ID. |  |
**include_schema_generation** | Option<**bool**> | Include generated schemas in the response |  |[default to false]
**include_test_data** | Option<**bool**> | Include test data when executing searches |  |[default to false]
**infer_json_types** | Option<**bool**> | Whether to try to infer data types in json event response instead of returning map[string]string |  |[default to false]
**match_response_schema** | Option<**bool**> | Whether to validate search results against their schema |  |[default to false]
**metadata** | Option<**bool**> | Whether to include metadata in the response |  |[default to false]
**mode** | Option<**String**> | Mode to execute the query under. |  |

### Return type

[**models::ApidomainPeriodQueryResponseWrapperV1**](apidomain.QueryResponseWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_saved_searches_dynamic_execute_v1

> models::ApidomainPeriodQueryResponseWrapperV1 create_saved_searches_dynamic_execute_v1(body, app_id, include_schema_generation, include_test_data, infer_json_types, match_response_schema, metadata, mode)
Execute a dynamic saved search

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApidomainPeriodDynamicExecuteSearchRequestV1**](ApidomainPeriodDynamicExecuteSearchRequestV1.md) |  | [required] |
**app_id** | Option<**String**> | Application ID. |  |
**include_schema_generation** | Option<**bool**> | Include generated schemas in the response |  |[default to false]
**include_test_data** | Option<**bool**> | Include test data when executing searches |  |[default to false]
**infer_json_types** | Option<**bool**> | Whether to try to infer data types in json event response instead of returning map[string]string |  |[default to false]
**match_response_schema** | Option<**bool**> | Whether to validate search results against their schema |  |[default to false]
**metadata** | Option<**bool**> | Whether to include metadata in the response |  |[default to false]
**mode** | Option<**String**> | Mode to execute the query under. |  |

### Return type

[**models::ApidomainPeriodQueryResponseWrapperV1**](apidomain.QueryResponseWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_saved_searches_execute_alt_v1

> models::ApidomainPeriodQueryResponseWrapperV1 create_saved_searches_execute_alt_v1(body, app_id, detailed, include_test_data, infer_json_types, match_response_schema, metadata)
Execute a saved search

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApidomainPeriodSavedSearchExecuteRequestV1**](ApidomainPeriodSavedSearchExecuteRequestV1.md) |  | [required] |
**app_id** | Option<**String**> | Application ID. |  |
**detailed** | Option<**bool**> | Whether to include search field details |  |[default to false]
**include_test_data** | Option<**bool**> | Include test data when executing searches |  |[default to false]
**infer_json_types** | Option<**bool**> | Whether to try to infer data types in json event response instead of returning map[string]string |  |[default to false]
**match_response_schema** | Option<**bool**> | Whether to validate search results against their schema |  |[default to false]
**metadata** | Option<**bool**> | Whether to include metadata in the response |  |[default to false]

### Return type

[**models::ApidomainPeriodQueryResponseWrapperV1**](apidomain.QueryResponseWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_saved_searches_execute_v1

> models::ApidomainPeriodQueryResponseWrapperV1 create_saved_searches_execute_v1(body, app_id, detailed, include_test_data, infer_json_types, match_response_schema, metadata)
Execute a saved search

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApidomainPeriodSavedSearchExecuteRequestV1**](ApidomainPeriodSavedSearchExecuteRequestV1.md) |  | [required] |
**app_id** | Option<**String**> | Application ID. |  |
**detailed** | Option<**bool**> | Whether to include search field details |  |[default to false]
**include_test_data** | Option<**bool**> | Include test data when executing searches |  |[default to false]
**infer_json_types** | Option<**bool**> | Whether to try to infer data types in json event response instead of returning map[string]string |  |[default to false]
**match_response_schema** | Option<**bool**> | Whether to validate search results against their schema |  |[default to false]
**metadata** | Option<**bool**> | Whether to include metadata in the response |  |[default to false]

### Return type

[**models::ApidomainPeriodQueryResponseWrapperV1**](apidomain.QueryResponseWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_saved_searches_ingest_alt_v1

> models::ClientPeriodDataIngestResponseWrapperV1 create_saved_searches_ingest_alt_v1(app_id)
Populate a saved search

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | Option<**String**> | Application ID. |  |

### Return type

[**models::ClientPeriodDataIngestResponseWrapperV1**](client.DataIngestResponseWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_saved_searches_ingest_v1

> models::ClientPeriodDataIngestResponseWrapperV1 create_saved_searches_ingest_v1(app_id)
Populate a saved search

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | Option<**String**> | Application ID. |  |

### Return type

[**models::ClientPeriodDataIngestResponseWrapperV1**](client.DataIngestResponseWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_saved_searches_execute_alt_v1

> models::ApidomainPeriodQueryResponseWrapperV1 get_saved_searches_execute_alt_v1(job_id, app_id, infer_json_types, job_status_only, limit, match_response_schema, metadata, offset)
Get the results of a saved search

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job ID for a previously executed async query | [required] |
**app_id** | Option<**String**> | Application ID. |  |
**infer_json_types** | Option<**bool**> | Whether to try to infer data types in json event response instead of returning map[string]string |  |[default to false]
**job_status_only** | Option<**bool**> | If set to true, result rows are dropped from the response and only the job status is returned |  |[default to false]
**limit** | Option<**String**> | Maximum number of records to return. |  |
**match_response_schema** | Option<**bool**> | Whether to validate search results against their schema |  |[default to false]
**metadata** | Option<**bool**> | Whether to include metadata in the response |  |[default to false]
**offset** | Option<**String**> | Starting pagination offset of records to return. |  |

### Return type

[**models::ApidomainPeriodQueryResponseWrapperV1**](apidomain.QueryResponseWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_saved_searches_execute_v1

> models::ApidomainPeriodQueryResponseWrapperV1 get_saved_searches_execute_v1(job_id, app_id, infer_json_types, job_status_only, limit, match_response_schema, metadata, offset)
Get the results of a saved search

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job ID for a previously executed async query | [required] |
**app_id** | Option<**String**> | Application ID. |  |
**infer_json_types** | Option<**bool**> | Whether to try to infer data types in json event response instead of returning map[string]string |  |[default to false]
**job_status_only** | Option<**bool**> | If set to true, result rows are dropped from the response and only the job status is returned |  |[default to false]
**limit** | Option<**String**> | Maximum number of records to return. |  |
**match_response_schema** | Option<**bool**> | Whether to validate search results against their schema |  |[default to false]
**metadata** | Option<**bool**> | Whether to include metadata in the response |  |[default to false]
**offset** | Option<**String**> | Starting pagination offset of records to return. |  |

### Return type

[**models::ApidomainPeriodQueryResponseWrapperV1**](apidomain.QueryResponseWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_saved_searches_job_results_download_alt_v1

> std::path::PathBuf get_saved_searches_job_results_download_alt_v1(job_id, infer_json_types, result_format)
Get the results of a saved search as a file

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job ID for a previously executed async query | [required] |
**infer_json_types** | Option<**bool**> | Whether to try to infer data types in json event response instead of returning map[string]string |  |[default to false]
**result_format** | Option<**String**> | Result Format |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/csv, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_saved_searches_job_results_download_v1

> std::path::PathBuf get_saved_searches_job_results_download_v1(job_id, infer_json_types, result_format)
Get the results of a saved search as a file

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job ID for a previously executed async query | [required] |
**infer_json_types** | Option<**bool**> | Whether to try to infer data types in json event response instead of returning map[string]string |  |[default to false]
**result_format** | Option<**String**> | Result Format |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/csv, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ingest_data_async_v1

> models::ClientPeriodDataIngestResponseWrapperV1 ingest_data_async_v1(data_content, data_file, repo, tag, tag_source, test_data)
Asynchronously ingest data into the application repository

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_content** | Option<**String**> | JSON data to ingest |  |
**data_file** | Option<**std::path::PathBuf**> | Data file to ingest |  |
**repo** | Option<**String**> | Repository name if not part of a foundry app |  |
**tag** | Option<[**Vec<String>**](String.md)> | Custom tag for ingested data in the form tag:value |  |
**tag_source** | Option<**String**> | Tag the data with the specified source |  |
**test_data** | Option<**bool**> | Tag the data with test-ingest |  |[default to false]

### Return type

[**models::ClientPeriodDataIngestResponseWrapperV1**](client.DataIngestResponseWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## ingest_data_v1

> models::ClientPeriodDataIngestResponseWrapperV1 ingest_data_v1(data_content, data_file, tag, tag_source, test_data)
Synchronously ingest data into the application repository

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_content** | Option<**String**> | JSON data to ingest |  |
**data_file** | Option<**std::path::PathBuf**> | Data file to ingest |  |
**tag** | Option<[**Vec<String>**](String.md)> | Custom tag for ingested data in the form tag:value |  |
**tag_source** | Option<**String**> | Tag the data with the specified source |  |
**test_data** | Option<**bool**> | Tag the data with test-ingest |  |[default to false]

### Return type

[**models::ClientPeriodDataIngestResponseWrapperV1**](client.DataIngestResponseWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## list_repos_v1

> models::ApidomainPeriodRepoViewListItemWrapperV1 list_repos_v1(check_test_data)
Lists available repositories and views

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_test_data** | Option<**bool**> | Include whether test data is present in the application repository |  |[default to false]

### Return type

[**models::ApidomainPeriodRepoViewListItemWrapperV1**](apidomain.RepoViewListItemWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## list_view_v1

> models::ApidomainPeriodRepoViewListItemWrapperV1 list_view_v1(check_test_data)
List views

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_test_data** | Option<**bool**> | Include whether test data is present in the application repository |  |[default to false]

### Return type

[**models::ApidomainPeriodRepoViewListItemWrapperV1**](apidomain.RepoViewListItemWrapperV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
