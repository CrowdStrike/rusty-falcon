# \DeviceControlPoliciesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_device_control_policies**](DeviceControlPoliciesApi.md#create_device_control_policies) | **POST** /policy/entities/device-control/v1 | Create Device Control Policies by specifying details about the policy to create
[**delete_device_control_policies**](DeviceControlPoliciesApi.md#delete_device_control_policies) | **DELETE** /policy/entities/device-control/v1 | Delete a set of Device Control Policies by specifying their IDs
[**get_default_device_control_policies**](DeviceControlPoliciesApi.md#get_default_device_control_policies) | **GET** /policy/entities/default-device-control/v1 | Retrieve the configuration for a Default Device Control Policy
[**get_device_control_policies**](DeviceControlPoliciesApi.md#get_device_control_policies) | **GET** /policy/entities/device-control/v1 | Retrieve a set of Device Control Policies by specifying their IDs
[**perform_device_control_policies_action**](DeviceControlPoliciesApi.md#perform_device_control_policies_action) | **POST** /policy/entities/device-control-actions/v1 | Perform the specified action on the Device Control Policies specified in the request
[**query_combined_device_control_policies**](DeviceControlPoliciesApi.md#query_combined_device_control_policies) | **GET** /policy/combined/device-control/v1 | Search for Device Control Policies in your environment by providing an FQL filter and paging details. Returns a set of Device Control Policies which match the filter criteria
[**query_combined_device_control_policy_members**](DeviceControlPoliciesApi.md#query_combined_device_control_policy_members) | **GET** /policy/combined/device-control-members/v1 | Search for members of a Device Control Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
[**query_device_control_policies**](DeviceControlPoliciesApi.md#query_device_control_policies) | **GET** /policy/queries/device-control/v1 | Search for Device Control Policies in your environment by providing an FQL filter and paging details. Returns a set of Device Control Policy IDs which match the filter criteria
[**query_device_control_policy_members**](DeviceControlPoliciesApi.md#query_device_control_policy_members) | **GET** /policy/queries/device-control-members/v1 | Search for members of a Device Control Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
[**set_device_control_policies_precedence**](DeviceControlPoliciesApi.md#set_device_control_policies_precedence) | **POST** /policy/entities/device-control-precedence/v1 | Sets the precedence of Device Control Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence
[**update_default_device_control_policies**](DeviceControlPoliciesApi.md#update_default_device_control_policies) | **PATCH** /policy/entities/default-device-control/v1 | Update the configuration for a Default Device Control Policy
[**update_device_control_policies**](DeviceControlPoliciesApi.md#update_device_control_policies) | **PATCH** /policy/entities/device-control/v1 | Update Device Control Policies by specifying the ID of the policy and details to update

## create_device_control_policies

> models::DeviceControlPeriodRespV2 create_device_control_policies(body)
Create Device Control Policies by specifying details about the policy to create

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DeviceControlPeriodCreatePoliciesV1**](DeviceControlPeriodCreatePoliciesV1.md) |  | [required] |

### Return type

[**models::DeviceControlPeriodRespV2**](device_control.RespV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_device_control_policies

> models::MsaPeriodQueryResponse delete_device_control_policies(ids)
Delete a set of Device Control Policies by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Device Control Policies to delete | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_default_device_control_policies

> models::DeviceControlPeriodRespV1 get_default_device_control_policies()
Retrieve the configuration for a Default Device Control Policy

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DeviceControlPeriodRespV1**](device_control.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_device_control_policies

> models::DeviceControlPeriodRespV1 get_device_control_policies(ids)
Retrieve a set of Device Control Policies by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Device Control Policies to return | [required] |

### Return type

[**models::DeviceControlPeriodRespV1**](device_control.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## perform_device_control_policies_action

> models::DeviceControlPeriodRespV1 perform_device_control_policies_action(action_name, body)
Perform the specified action on the Device Control Policies specified in the request

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_name** | **String** | The action to perform | [required] |
**body** | [**MsaPeriodEntityActionRequestV2**](MsaPeriodEntityActionRequestV2.md) |  | [required] |

### Return type

[**models::DeviceControlPeriodRespV1**](device_control.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_device_control_policies

> models::DeviceControlPeriodRespV1 query_combined_device_control_policies(filter, offset, limit, sort)
Search for Device Control Policies in your environment by providing an FQL filter and paging details. Returns a set of Device Control Policies which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::DeviceControlPeriodRespV1**](device_control.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_device_control_policy_members

> models::BasePeriodPolicyMembersRespV1 query_combined_device_control_policy_members(id, filter, offset, limit, sort)
Search for members of a Device Control Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Device Control Policy to search for members of |  |
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::BasePeriodPolicyMembersRespV1**](base.PolicyMembersRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_device_control_policies

> models::MsaPeriodQueryResponse query_device_control_policies(filter, offset, limit, sort)
Search for Device Control Policies in your environment by providing an FQL filter and paging details. Returns a set of Device Control Policy IDs which match the filter criteria

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

## query_device_control_policy_members

> models::MsaPeriodQueryResponse query_device_control_policy_members(id, filter, offset, limit, sort)
Search for members of a Device Control Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Device Control Policy to search for members of |  |
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

## set_device_control_policies_precedence

> models::MsaPeriodQueryResponse set_device_control_policies_precedence(body)
Sets the precedence of Device Control Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BasePeriodSetPolicyPrecedenceReqV1**](BasePeriodSetPolicyPrecedenceReqV1.md) |  | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_default_device_control_policies

> models::DeviceControlPeriodRespV1 update_default_device_control_policies(body)
Update the configuration for a Default Device Control Policy

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DeviceControlPeriodReqUpdateDefaultDcPolicyV1**](DeviceControlPeriodReqUpdateDefaultDcPolicyV1.md) |  | [required] |

### Return type

[**models::DeviceControlPeriodRespV1**](device_control.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_device_control_policies

> models::DeviceControlPeriodRespV1 update_device_control_policies(body)
Update Device Control Policies by specifying the ID of the policy and details to update

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DeviceControlPeriodUpdatePoliciesReqV1**](DeviceControlPeriodUpdatePoliciesReqV1.md) |  | [required] |

### Return type

[**models::DeviceControlPeriodRespV1**](device_control.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
