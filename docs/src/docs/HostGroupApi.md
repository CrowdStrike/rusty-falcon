# \HostGroupApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_host_groups**](HostGroupApi.md#create_host_groups) | **POST** /devices/entities/host-groups/v1 | Create Host Groups by specifying details about the group to create
[**delete_host_groups**](HostGroupApi.md#delete_host_groups) | **DELETE** /devices/entities/host-groups/v1 | Delete a set of Host Groups by specifying their IDs
[**get_host_groups**](HostGroupApi.md#get_host_groups) | **GET** /devices/entities/host-groups/v1 | Retrieve a set of Host Groups by specifying their IDs
[**perform_group_action**](HostGroupApi.md#perform_group_action) | **POST** /devices/entities/host-group-actions/v1 | Perform the specified action on the Host Groups specified in the request
[**query_combined_group_members**](HostGroupApi.md#query_combined_group_members) | **GET** /devices/combined/host-group-members/v1 | Search for members of a Host Group in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
[**query_combined_host_groups**](HostGroupApi.md#query_combined_host_groups) | **GET** /devices/combined/host-groups/v1 | Search for Host Groups in your environment by providing an FQL filter and paging details. Returns a set of Host Groups which match the filter criteria
[**query_group_members**](HostGroupApi.md#query_group_members) | **GET** /devices/queries/host-group-members/v1 | Search for members of a Host Group in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
[**query_host_groups**](HostGroupApi.md#query_host_groups) | **GET** /devices/queries/host-groups/v1 | Search for Host Groups in your environment by providing an FQL filter and paging details. Returns a set of Host Group IDs which match the filter criteria
[**update_host_groups**](HostGroupApi.md#update_host_groups) | **PATCH** /devices/entities/host-groups/v1 | Update Host Groups by specifying the ID of the group and details to update

## create_host_groups

> models::HostGroupsPeriodRespV1 create_host_groups(body)
Create Host Groups by specifying details about the group to create

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**HostGroupsPeriodCreateGroupsReqV1**](HostGroupsPeriodCreateGroupsReqV1.md) |  | [required] |

### Return type

[**models::HostGroupsPeriodRespV1**](host_groups.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_host_groups

> models::MsaPeriodQueryResponse delete_host_groups(ids)
Delete a set of Host Groups by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Host Groups to delete | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_host_groups

> models::HostGroupsPeriodRespV1 get_host_groups(ids)
Retrieve a set of Host Groups by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Host Groups to return | [required] |

### Return type

[**models::HostGroupsPeriodRespV1**](host_groups.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## perform_group_action

> models::HostGroupsPeriodRespV1 perform_group_action(action_name, body, disable_hostname_check)
Perform the specified action on the Host Groups specified in the request

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_name** | **String** | The action to perform | [required] |
**body** | [**MsaPeriodEntityActionRequestV2**](MsaPeriodEntityActionRequestV2.md) |  | [required] |
**disable_hostname_check** | Option<**bool**> | Bool to disable hostname check on add-member |  |[default to false]

### Return type

[**models::HostGroupsPeriodRespV1**](host_groups.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_group_members

> models::HostGroupsPeriodMembersRespV1 query_combined_group_members(id, filter, offset, limit, sort)
Search for members of a Host Group in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Host Group to search for members of |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::HostGroupsPeriodMembersRespV1**](host_groups.MembersRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_host_groups

> models::HostGroupsPeriodRespV1 query_combined_host_groups(filter, offset, limit, sort)
Search for Host Groups in your environment by providing an FQL filter and paging details. Returns a set of Host Groups which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::HostGroupsPeriodRespV1**](host_groups.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_group_members

> models::MsaPeriodQueryResponse query_group_members(id, filter, offset, limit, sort)
Search for members of a Host Group in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Host Group to search for members of |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_host_groups

> models::MsaPeriodQueryResponse query_host_groups(filter, offset, limit, sort)
Search for Host Groups in your environment by providing an FQL filter and paging details. Returns a set of Host Group IDs which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_host_groups

> models::HostGroupsPeriodRespV1 update_host_groups(body)
Update Host Groups by specifying the ID of the group and details to update

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**HostGroupsPeriodUpdateGroupsReqV1**](HostGroupsPeriodUpdateGroupsReqV1.md) |  | [required] |

### Return type

[**models::HostGroupsPeriodRespV1**](host_groups.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
