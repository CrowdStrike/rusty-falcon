# \IocsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**devices_count**](IocsApi.md#devices_count) | **GET** /indicators/aggregates/devices-count/v1 | Number of hosts in your customer account that have observed a given custom IOC
[**devices_ran_on**](IocsApi.md#devices_ran_on) | **GET** /indicators/queries/devices/v1 | Find hosts that have observed a given custom IOC. For details about those hosts, use GET /devices/entities/devices/v1
[**entities_period_processes**](IocsApi.md#entities_period_processes) | **GET** /processes/entities/processes/v1 | For the provided ProcessID retrieve the process details
[**processes_ran_on**](IocsApi.md#processes_ran_on) | **GET** /indicators/queries/processes/v1 | Search for processes associated with a custom IOC

## devices_count

> models::IocapiPeriodMsaReplyIocDevicesCount devices_count(r#type, value)
Number of hosts in your customer account that have observed a given custom IOC

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  The type of the indicator. Valid types include:  sha256: A hex-encoded sha256 hash string. Length - min: 64, max: 64.  md5: A hex-encoded md5 hash string. Length - min 32, max: 32.  domain: A domain name. Length - min: 1, max: 200.  ipv4: An IPv4 address. Must be a valid IP address.  ipv6: An IPv6 address. Must be a valid IP address.  | [required] |
**value** | **String** | The string representation of the indicator | [required] |

### Return type

[**models::IocapiPeriodMsaReplyIocDevicesCount**](iocapi.MsaReplyIOCDevicesCount.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## devices_ran_on

> models::IocapiPeriodMsaReplyDevicesRanOn devices_ran_on(r#type, value, limit, offset)
Find hosts that have observed a given custom IOC. For details about those hosts, use GET /devices/entities/devices/v1

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  The type of the indicator. Valid types include:  sha256: A hex-encoded sha256 hash string. Length - min: 64, max: 64.  md5: A hex-encoded md5 hash string. Length - min 32, max: 32.  domain: A domain name. Length - min: 1, max: 200.  ipv4: An IPv4 address. Must be a valid IP address.  ipv6: An IPv6 address. Must be a valid IP address.  | [required] |
**value** | **String** | The string representation of the indicator | [required] |
**limit** | Option<**String**> | The first process to return, where 0 is the latest offset. Use with the offset parameter to manage pagination of results. |  |
**offset** | Option<**String**> | The first process to return, where 0 is the latest offset. Use with the limit parameter to manage pagination of results. |  |

### Return type

[**models::IocapiPeriodMsaReplyDevicesRanOn**](iocapi.MsaReplyDevicesRanOn.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## entities_period_processes

> models::ProcessesapiPeriodMsaProcessDetailResponse entities_period_processes(ids)
For the provided ProcessID retrieve the process details

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | ProcessID for the running process you want to lookup | [required] |

### Return type

[**models::ProcessesapiPeriodMsaProcessDetailResponse**](processesapi.MsaProcessDetailResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## processes_ran_on

> models::IocapiPeriodMsaReplyProcessesRanOn processes_ran_on(r#type, value, device_id, limit, offset)
Search for processes associated with a custom IOC

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  The type of the indicator. Valid types include:  sha256: A hex-encoded sha256 hash string. Length - min: 64, max: 64.  md5: A hex-encoded md5 hash string. Length - min 32, max: 32.  domain: A domain name. Length - min: 1, max: 200.  ipv4: An IPv4 address. Must be a valid IP address.  ipv6: An IPv6 address. Must be a valid IP address.  | [required] |
**value** | **String** | The string representation of the indicator | [required] |
**device_id** | **String** | Specify a host's ID to return only processes from that host. Get a host's ID from GET /devices/queries/devices/v1, the Falcon console, or the Streaming API. | [required] |
**limit** | Option<**String**> | The first process to return, where 0 is the latest offset. Use with the offset parameter to manage pagination of results. |  |
**offset** | Option<**String**> | The first process to return, where 0 is the latest offset. Use with the limit parameter to manage pagination of results. |  |

### Return type

[**models::IocapiPeriodMsaReplyProcessesRanOn**](iocapi.MsaReplyProcessesRanOn.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
