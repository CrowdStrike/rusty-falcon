# \DiscoverIotApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_iot_hosts**](DiscoverIotApi.md#get_iot_hosts) | **GET** /discover/entities/iot-hosts/v1 | Get details on IoT assets by providing one or more IDs.
[**query_iot_hosts**](DiscoverIotApi.md#query_iot_hosts) | **GET** /discover/queries/iot-hosts/v1 | Search for IoT assets in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of asset IDs which match the filter criteria.
[**query_iot_hosts_v2**](DiscoverIotApi.md#query_iot_hosts_v2) | **GET** /discover/queries/iot-hosts/v2 | Search for IoT assets in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of asset IDs which match the filter criteria.

## get_iot_hosts

> models::DomainPeriodDiscoverApiioTHostEntitiesResponse get_iot_hosts(ids)
Get details on IoT assets by providing one or more IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | One or more asset IDs (max: 100). Find asset IDs with GET `/discover/queries/iot-hosts/v1` | [required] |

### Return type

[**models::DomainPeriodDiscoverApiioTHostEntitiesResponse**](domain.DiscoverAPIIoTHostEntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_iot_hosts

> models::MsaspecPeriodQueryResponse query_iot_hosts(offset, limit, sort, filter)
Search for IoT assets in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of asset IDs which match the filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | An offset used with the `limit` parameter to manage pagination of results. On your first request, don’t provide an `offset`. On subsequent requests, add previous `offset` with the previous `limit` to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of asset IDs to return in this response (min: 1, max: 100, default: 100). Use with the `offset` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort assets by their properties. A single sort field is allowed. Common sort options include:  <ul><li>hostname|asc</li><li>product_type_desc|desc</li></ul> |  |
**filter** | Option<**String**> | Filter assets using an FQL query. Common filter options include:<ul><li>entity_type:'managed'</li><li>product_type_desc:'Workstation'</li><li>platform_name:'Windows'</li><li>last_seen_timestamp:>'now-7d'</li></ul>    Available filter fields that support exact match: device_family, device_class, device_type, device_mode, business_criticality, line_of_business, virtual_zone, subnet, purdue_level, vlan, local_ip_addresses, mac_addresses, physical_connections_count, data_providers, local_ips_count, network_interfaces.local_ip, classification    Available filter fields that supports wildcard (*): device_family, device_class, device_type, device_mode, business_criticality, line_of_business, virtual_zone, subnet, purdue_level, vlan, local_ip_addresses, mac_addresses, data_providers    Available filter fields that supports range comparisons (>, <, >=, <=): physical_connections_count, local_ips_count    All filter fields and operations supports negation (!). |  |

### Return type

[**models::MsaspecPeriodQueryResponse**](msaspec.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_iot_hosts_v2

> models::DomainPeriodDiscoverApiResponse query_iot_hosts_v2(after, limit, sort, filter)
Search for IoT assets in your environment by providing an FQL (Falcon Query Language) filter and paging details. Returns a set of asset IDs which match the filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**String**> | A pagination token used with the `limit` parameter to manage pagination of results. On your first request, don't provide an `after` token. On subsequent requests, provide the `after` token from the previous response to continue from that place in the results. |  |
**limit** | Option<**i32**> | The number of asset IDs to return in this response (min: 1, max: 100, default: 100). Use with the `after` parameter to manage pagination of results. |  |
**sort** | Option<**String**> | Sort assets by their properties. A single sort field is allowed. Common sort options include:  <ul><li>hostname|asc</li><li>product_type_desc|desc</li></ul> |  |
**filter** | Option<**String**> | Filter assets using an FQL query. Common filter options include:<ul><li>entity_type:'managed'</li><li>product_type_desc:'Workstation'</li><li>platform_name:'Windows'</li><li>last_seen_timestamp:>'now-7d'</li></ul>    Available filter fields that support exact match: device_family, device_class, device_type, device_mode, business_criticality, line_of_business, virtual_zone, subnet, purdue_level, vlan, local_ip_addresses, mac_addresses, physical_connections_count, data_providers    Available filter fields that supports wildcard (*): device_family, device_class, device_type, device_mode, business_criticality, line_of_business, virtual_zone, subnet, purdue_level, vlan, local_ip_addresses, mac_addresses, data_providers    Available filter fields that supports range comparisons (>, <, >=, <=): physical_connections_count    All filter fields and operations supports negation (!). |  |

### Return type

[**models::DomainPeriodDiscoverApiResponse**](domain.DiscoverAPIResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
