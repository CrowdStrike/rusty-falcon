# \FilevantageApi

All URIs are relative to *https://api.crowdstrike.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_changes**](FilevantageApi.md#get_changes) | **GET** /filevantage/entities/changes/v2 | Retrieve information on changes
[**query_changes**](FilevantageApi.md#query_changes) | **GET** /filevantage/queries/changes/v2 | Returns one or more change IDs



## get_changes

> crate::models::PublicPeriodGetChangesResponse get_changes(ids)
Retrieve information on changes

Retrieve key attributes of Falcon FileVantage changes for the specified ids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more change ids in the form of ids=ID1&ids=ID2 | [required] |

### Return type

[**crate::models::PublicPeriodGetChangesResponse**](public.GetChangesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_changes

> crate::models::MsaspecPeriodQueryResponse query_changes(offset, limit, sort, filter)
Returns one or more change IDs

Returns a list of Falcon FileVantage change IDs filtered, sorted and limited by the query parameters provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The first change index to return in the response. If not provided it will default to '0'. Use with the `limit` parameter to manage pagination of results. |  |
**limit** | Option<**i32**> | The maximum number of changes to return in the response (default: 100; max: 500). Use with the `offset` parameter to manage pagination of results |  |
**sort** | Option<**String**> | Sort changes using options like:  - `action_timestamp` (timestamp of the change occurrence)    Sort either `asc` (ascending) or `desc` (descending). For example: `action_timestamp|asc`. The full list of allowed sorting options can be reviewed in our API documentation. |  |
**filter** | Option<**String**> | Filter changes using a query in Falcon Query Language (FQL).   Common filter options include:   - `host.host_name`  - `action_timestamp`   The full list of allowed filter parameters can be reviewed in our API documentation. |  |

### Return type

[**crate::models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

