# \HostsApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**entities_period_perform_action**](HostsApi.md#entities_period_perform_action) | **POST** /devices/entities/group-actions/v1 | Performs the specified action on the provided group IDs.
[**get_device_details_v2**](HostsApi.md#get_device_details_v2) | **GET** /devices/entities/devices/v2 | Get details on one or more hosts by providing host IDs as a query parameter.  Supports up to a maximum 100 IDs.
[**get_online_state_period_v1**](HostsApi.md#get_online_state_period_v1) | **GET** /devices/entities/online-state/v1 | Get the online status for one or more hosts by specifying each host’s unique ID. Successful requests return an HTTP 200 response and the status for each host identified by a `state` of `online`, `offline`, or `unknown` for each host, identified by host `id`.  Make a `GET` request to `/devices/queries/devices/v1` to get a list of host IDs.
[**perform_action_v2**](HostsApi.md#perform_action_v2) | **POST** /devices/entities/devices-actions/v2 | Take various actions on the hosts in your environment. Contain or lift containment on a host. Delete or restore a host.
[**post_device_details_v2**](HostsApi.md#post_device_details_v2) | **POST** /devices/entities/devices/v2 | Get details on one or more hosts by providing host IDs in a POST body.  Supports up to a maximum 5000 IDs.
[**query_device_login_history**](HostsApi.md#query_device_login_history) | **POST** /devices/combined/devices/login-history/v1 | Retrieve details about recent login sessions for a set of devices.
[**query_device_login_history_v2**](HostsApi.md#query_device_login_history_v2) | **POST** /devices/combined/devices/login-history/v2 | Retrieve details about recent interactive login sessions for a set of devices powered by the Host Timeline. A max of 10 device ids can be specified
[**query_devices_by_filter**](HostsApi.md#query_devices_by_filter) | **GET** /devices/queries/devices/v1 | Search for hosts in your environment by platform, hostname, IP, and other criteria.
[**query_devices_by_filter_scroll**](HostsApi.md#query_devices_by_filter_scroll) | **GET** /devices/queries/devices-scroll/v1 | Search for hosts in your environment by platform, hostname, IP, and other criteria with continuous pagination capability (based on offset pointer which expires after 2 minutes with no maximum limit)
[**query_get_network_address_history_v1**](HostsApi.md#query_get_network_address_history_v1) | **POST** /devices/combined/devices/network-address-history/v1 | Retrieve history of IP and MAC addresses of devices.
[**query_hidden_devices**](HostsApi.md#query_hidden_devices) | **GET** /devices/queries/devices-hidden/v1 | Retrieve hidden hosts that match the provided filter criteria.
[**update_device_tags**](HostsApi.md#update_device_tags) | **PATCH** /devices/entities/devices/tags/v1 | Append or remove one or more Falcon Grouping Tags on one or more hosts.  Tags must be of the form FalconGroupingTags/

## entities_period_perform_action

> models::DeviceapiPeriodGroupsResponseV1 entities_period_perform_action(ids, action_name, body, disable_hostname_check)
Performs the specified action on the provided group IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The group ids to act on | [required] |
**action_name** | **String** | The action to perform. | [required] |
**body** | [**MsaPeriodEntityActionRequest**](MsaPeriodEntityActionRequest.md) |  | [required] |
**disable_hostname_check** | Option<**bool**> | Bool to disable hostname check on add-member |  |[default to false]

### Return type

[**models::DeviceapiPeriodGroupsResponseV1**](deviceapi.GroupsResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_device_details_v2

> models::DeviceapiPeriodDeviceDetailsResponseSwagger get_device_details_v2(ids)
Get details on one or more hosts by providing host IDs as a query parameter.  Supports up to a maximum 100 IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The host agentIDs used to get details on | [required] |

### Return type

[**models::DeviceapiPeriodDeviceDetailsResponseSwagger**](deviceapi.DeviceDetailsResponseSwagger.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_online_state_period_v1

> models::StatePeriodOnlineStateRespV1 get_online_state_period_v1(ids)
Get the online status for one or more hosts by specifying each host’s unique ID. Successful requests return an HTTP 200 response and the status for each host identified by a `state` of `online`, `offline`, or `unknown` for each host, identified by host `id`.  Make a `GET` request to `/devices/queries/devices/v1` to get a list of host IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The unique ID of the host to get the online status of. | [required] |

### Return type

[**models::StatePeriodOnlineStateRespV1**](state.OnlineStateRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## perform_action_v2

> models::MsaPeriodReplyAffectedEntities perform_action_v2(action_name, body)
Take various actions on the hosts in your environment. Contain or lift containment on a host. Delete or restore a host.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_name** | **String** | Specify one of these actions:  - `contain` - This action contains the host, which stops any network communications to locations other than the CrowdStrike cloud and IPs specified in your [containment policy](https://falcon.crowdstrike.com/support/documentation/11/getting-started-guide#containmentpolicy) - `lift_containment`: This action lifts containment on the host, which returns its network communications to normal - `hide_host`: This action will delete a host. After the host is deleted, no new detections for that host will be reported via UI or APIs - `unhide_host`: This action will restore a host. Detection reporting will resume after the host is restored | [required] |
**body** | [**MsaPeriodEntityActionRequestV2**](MsaPeriodEntityActionRequestV2.md) | The host agent ID (AID) of the host you want to contain. Get an agent ID from a detection, the Falcon console, or the Streaming API.  Provide the ID in JSON format with the key `ids` and the value in square brackets, such as:   `\"ids\": [\"123456789\"]` | [required] |

### Return type

[**models::MsaPeriodReplyAffectedEntities**](msa.ReplyAffectedEntities.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_device_details_v2

> models::DeviceapiPeriodDeviceDetailsResponseSwagger post_device_details_v2(body)
Get details on one or more hosts by providing host IDs in a POST body.  Supports up to a maximum 5000 IDs.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DeviceapiPeriodDeviceDetailsResponseSwagger**](deviceapi.DeviceDetailsResponseSwagger.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_device_login_history

> models::DeviceapiPeriodLoginHistoryResponseV1 query_device_login_history(body)
Retrieve details about recent login sessions for a set of devices.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DeviceapiPeriodLoginHistoryResponseV1**](deviceapi.LoginHistoryResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_device_login_history_v2

> models::DeviceapiPeriodLoginHistoryResponseV1 query_device_login_history_v2(body)
Retrieve details about recent interactive login sessions for a set of devices powered by the Host Timeline. A max of 10 device ids can be specified

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DeviceapiPeriodLoginHistoryResponseV1**](deviceapi.LoginHistoryResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_devices_by_filter

> models::MsaPeriodQueryResponse query_devices_by_filter(offset, limit, sort, filter)
Search for hosts in your environment by platform, hostname, IP, and other criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by (e.g. status.desc or hostname.asc) |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_devices_by_filter_scroll

> models::DeviceapiPeriodDeviceResponse query_devices_by_filter_scroll(offset, limit, sort, filter)
Search for hosts in your environment by platform, hostname, IP, and other criteria with continuous pagination capability (based on offset pointer which expires after 2 minutes with no maximum limit)

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | The offset to page from, for the next result set |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by (e.g. status.desc or hostname.asc) |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |

### Return type

[**models::DeviceapiPeriodDeviceResponse**](deviceapi.DeviceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_get_network_address_history_v1

> models::DeviceapiPeriodNetworkAddressHistoryResponseV1 query_get_network_address_history_v1(body)
Retrieve history of IP and MAC addresses of devices.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MsaPeriodIdsRequest**](MsaPeriodIdsRequest.md) |  | [required] |

### Return type

[**models::DeviceapiPeriodNetworkAddressHistoryResponseV1**](deviceapi.NetworkAddressHistoryResponseV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_hidden_devices

> models::MsaPeriodQueryResponse query_hidden_devices(offset, limit, sort, filter)
Retrieve hidden hosts that match the provided filter criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by (e.g. status.desc or hostname.asc) |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_device_tags

> models::DeviceapiPeriodUpdateDeviceTagsSwaggerV1 update_device_tags(body)
Append or remove one or more Falcon Grouping Tags on one or more hosts.  Tags must be of the form FalconGroupingTags/

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DeviceapiPeriodUpdateDeviceTagsRequestV1**](DeviceapiPeriodUpdateDeviceTagsRequestV1.md) |  | [required] |

### Return type

[**models::DeviceapiPeriodUpdateDeviceTagsSwaggerV1**](deviceapi.UpdateDeviceTagsSwaggerV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
