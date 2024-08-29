# \EventStreamsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_available_streams_o_auth2**](EventStreamsApi.md#list_available_streams_o_auth2) | **GET** /sensors/entities/datafeed/v2 | Discover all event streams in your environment
[**refresh_active_stream_session**](EventStreamsApi.md#refresh_active_stream_session) | **POST** /sensors/entities/datafeed-actions/v1/{partition} | Refresh an active event stream. Use the URL shown in a GET /sensors/entities/datafeed/v2 response.

## list_available_streams_o_auth2

> models::MainPeriodDiscoveryResponseV2 list_available_streams_o_auth2(app_id, format)
Discover all event streams in your environment

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | Label that identifies your connection. Max: 32 alphanumeric characters (a-z, A-Z, 0-9). | [required] |
**format** | Option<**String**> | Format for streaming events. Valid values: json, flatjson |  |

### Return type

[**models::MainPeriodDiscoveryResponseV2**](main.discoveryResponseV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## refresh_active_stream_session

> models::MsaPeriodReplyMetaOnly refresh_active_stream_session(action_name, app_id, partition)
Refresh an active event stream. Use the URL shown in a GET /sensors/entities/datafeed/v2 response.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_name** | **String** | Action name. Allowed value is refresh_active_stream_session. | [required] |
**app_id** | **String** | Label that identifies your connection. Max: 32 alphanumeric characters (a-z, A-Z, 0-9). | [required] |
**partition** | **i32** | Partition to request data for. | [required] |

### Return type

[**models::MsaPeriodReplyMetaOnly**](msa.ReplyMetaOnly.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
