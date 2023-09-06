# \FilevantageApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_changes**](FilevantageApi.md#get_changes) | **GET** /filevantage/entities/changes/v2 | Retrieve information on changes
[**high_volume_query_changes**](FilevantageApi.md#high_volume_query_changes) | **GET** /filevantage/queries/changes/v3 | Returns 1 or more change ids
[**query_changes**](FilevantageApi.md#query_changes) | **GET** /filevantage/queries/changes/v2 | Returns 1 or more change ids

## get_changes

> crate::models::ChangesPeriodGetChangesResponse get_changes(ids)
Retrieve information on changes

Retrieve key attributes of Falcon FileVantage changes for the specified ids.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more change ids in the form of `ids=ID1&ids=ID2`. The maximum number of ids that can be requested at once is `500`. | [required] |

### Return type

[**crate::models::ChangesPeriodGetChangesResponse**](changes.GetChangesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## high_volume_query_changes

> crate::models::ChangesPeriodHighVolumeQueryResponse high_volume_query_changes(after, limit, sort, filter)
Returns 1 or more change ids

Returns a list of Falcon FileVantage change IDs filtered, sorted and limited by the query parameters provided. It can retrieve an unlimited number of results using multiple requests.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request don't provide a value for the `after` token. On subsequent requests provide the `after` token value from the previous response to continue pagination from where you left. If the response returns an empty `after` token it means there are no more results to return. |  |
**limit** | Option<**i32**> | The maximum number of ids to return. Defaults to `100` if not specified. The maximum number of results that can be returned in a single call is `5000`. |  |[default to 100]
**sort** | Option<**String**> | Sort results using options like:  - `action_timestamp` (timestamp of the change occurrence)   Sort either `asc` (ascending) or `desc` (descending). For example: `action_timestamp|asc`. Defaults to`action_timestamp|desc` no value is specified. The full list of allowed sorting options can be reviewed in our API documentation. |  |[default to action_timestamp|desc]
**filter** | Option<**String**> | Filter changes using a query in Falcon Query Language (FQL).   Common filter options include:   - `host.name`  - `action_timestamp`   The full list of allowed filter parameters can be reviewed in our API documentation. |  |

### Return type

[**crate::models::ChangesPeriodHighVolumeQueryResponse**](changes.HighVolumeQueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_changes

> crate::models::MsaspecPeriodQueryResponse query_changes(offset, limit, sort, filter)
Returns 1 or more change ids

Returns a list of Falcon FileVantage change IDs filtered, sorted and limited by the query parameters provided. Using this endpoint you can retrieve up to `10000` results by using pagination with multiple requests. If you need to retrieve more than `10000` results consider using the `/queries/changes/v3` endpoint

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The offset to start retrieving records from. Defaults to `0` if not specified. |  |[default to 0]
**limit** | Option<**i32**> | The maximum number of ids to return. Defaults to `100` if not specified. The maximum number of results that can be returned in a single call is `500`. |  |[default to 100]
**sort** | Option<**String**> | Sort results using options like:  - `action_timestamp` (timestamp of the change occurrence)   Sort either `asc` (ascending) or `desc` (descending). For example: `action_timestamp|asc`. The full list of allowed sorting options can be reviewed in our API documentation. |  |
**filter** | Option<**String**> | Filter changes using a query in Falcon Query Language (FQL).   Common filter options include:   - `host.name`  - `action_timestamp`   The full list of allowed filter parameters can be reviewed in our API documentation. |  |

### Return type

[**crate::models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
