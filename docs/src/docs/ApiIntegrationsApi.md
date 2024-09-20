# \ApiIntegrationsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**execute_command**](ApiIntegrationsApi.md#execute_command) | **POST** /plugins/entities/execute/v1 | Execute a command.
[**get_combined_plugin_configs**](ApiIntegrationsApi.md#get_combined_plugin_configs) | **GET** /plugins/combined/configs/v1 | Queries for config resources and returns details

## execute_command

> models::DomainPeriodExecuteCommandResultsV1 execute_command(body)
Execute a command.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainPeriodExecuteCommandRequestV1**](DomainPeriodExecuteCommandRequestV1.md) |  | [required] |

### Return type

[**models::DomainPeriodExecuteCommandResultsV1**](domain.ExecuteCommandResultsV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_combined_plugin_configs

> models::DomainPeriodConfigsV1 get_combined_plugin_configs(filter, limit, offset, sort)
Queries for config resources and returns details

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter items using a query in Falcon Query Language (FQL). |  |
**limit** | Option<**i32**> | The number of items to return in this response (default: 100, max: 500). Use with the offset parameter to manage pagination of results. |  |[default to 100]
**offset** | Option<**i32**> | The first item to return, where 0 is the latest item. Use with the limit parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort items using their properties. |  |

### Return type

[**models::DomainPeriodConfigsV1**](domain.ConfigsV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
