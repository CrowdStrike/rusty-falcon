# \ResponsePoliciesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_rt_response_policies**](ResponsePoliciesApi.md#create_rt_response_policies) | **POST** /policy/entities/response/v1 | Create Response Policies by specifying details about the policy to create
[**delete_rt_response_policies**](ResponsePoliciesApi.md#delete_rt_response_policies) | **DELETE** /policy/entities/response/v1 | Delete a set of Response Policies by specifying their IDs
[**get_rt_response_policies**](ResponsePoliciesApi.md#get_rt_response_policies) | **GET** /policy/entities/response/v1 | Retrieve a set of Response Policies by specifying their IDs
[**perform_rt_response_policies_action**](ResponsePoliciesApi.md#perform_rt_response_policies_action) | **POST** /policy/entities/response-actions/v1 | Perform the specified action on the Response Policies specified in the request
[**query_combined_rt_response_policies**](ResponsePoliciesApi.md#query_combined_rt_response_policies) | **GET** /policy/combined/response/v1 | Search for Response Policies in your environment by providing an FQL filter and paging details. Returns a set of Response Policies which match the filter criteria
[**query_combined_rt_response_policy_members**](ResponsePoliciesApi.md#query_combined_rt_response_policy_members) | **GET** /policy/combined/response-members/v1 | Search for members of a Response policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
[**query_rt_response_policies**](ResponsePoliciesApi.md#query_rt_response_policies) | **GET** /policy/queries/response/v1 | Search for Response Policies in your environment by providing an FQL filter with sort and/or paging details. This returns a set of Response Policy IDs that match the given criteria.
[**query_rt_response_policy_members**](ResponsePoliciesApi.md#query_rt_response_policy_members) | **GET** /policy/queries/response-members/v1 | Search for members of a Response policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
[**set_rt_response_policies_precedence**](ResponsePoliciesApi.md#set_rt_response_policies_precedence) | **POST** /policy/entities/response-precedence/v1 | Sets the precedence of Response Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence
[**update_rt_response_policies**](ResponsePoliciesApi.md#update_rt_response_policies) | **PATCH** /policy/entities/response/v1 | Update Response Policies by specifying the ID of the policy and details to update

## create_rt_response_policies

> models::RemoteResponsePeriodRespV1 create_rt_response_policies(body)
Create Response Policies by specifying details about the policy to create

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RemoteResponsePeriodCreatePoliciesV1**](RemoteResponsePeriodCreatePoliciesV1.md) |  | [required] |

### Return type

[**models::RemoteResponsePeriodRespV1**](remote_response.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_rt_response_policies

> models::MsaPeriodQueryResponse delete_rt_response_policies(ids)
Delete a set of Response Policies by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Response Policies to delete | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_rt_response_policies

> models::RemoteResponsePeriodRespV1 get_rt_response_policies(ids)
Retrieve a set of Response Policies by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the RTR Policies to return | [required] |

### Return type

[**models::RemoteResponsePeriodRespV1**](remote_response.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## perform_rt_response_policies_action

> models::RemoteResponsePeriodRespV1 perform_rt_response_policies_action(action_name, body)
Perform the specified action on the Response Policies specified in the request

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_name** | **String** | The action to perform | [required] |
**body** | [**MsaPeriodEntityActionRequestV2**](MsaPeriodEntityActionRequestV2.md) |  | [required] |

### Return type

[**models::RemoteResponsePeriodRespV1**](remote_response.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_rt_response_policies

> models::RemoteResponsePeriodRespV1 query_combined_rt_response_policies(filter, offset, limit, sort)
Search for Response Policies in your environment by providing an FQL filter and paging details. Returns a set of Response Policies which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::RemoteResponsePeriodRespV1**](remote_response.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_rt_response_policy_members

> models::BasePeriodPolicyMembersRespV1 query_combined_rt_response_policy_members(id, filter, offset, limit, sort)
Search for members of a Response policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Response policy to search for members of |  |
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

## query_rt_response_policies

> models::MsaPeriodQueryResponse query_rt_response_policies(filter, offset, limit, sort)
Search for Response Policies in your environment by providing an FQL filter with sort and/or paging details. This returns a set of Response Policy IDs that match the given criteria.

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to determine the results. |  |
**offset** | Option<**i32**> | The offset of the first record to retrieve from |  |
**limit** | Option<**i32**> | The maximum number of records to return [1-5000] |  |
**sort** | Option<**String**> | The property to sort results by |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_rt_response_policy_members

> models::MsaPeriodQueryResponse query_rt_response_policy_members(id, filter, offset, limit, sort)
Search for members of a Response policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Response policy to search for members of |  |
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

## set_rt_response_policies_precedence

> models::MsaPeriodQueryResponse set_rt_response_policies_precedence(body)
Sets the precedence of Response Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence

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

## update_rt_response_policies

> models::RemoteResponsePeriodRespV1 update_rt_response_policies(body)
Update Response Policies by specifying the ID of the policy and details to update

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RemoteResponsePeriodUpdatePoliciesReqV1**](RemoteResponsePeriodUpdatePoliciesReqV1.md) |  | [required] |

### Return type

[**models::RemoteResponsePeriodRespV1**](remote_response.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
