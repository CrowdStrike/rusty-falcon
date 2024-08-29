# \FirewallPoliciesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_firewall_policies**](FirewallPoliciesApi.md#create_firewall_policies) | **POST** /policy/entities/firewall/v1 | Create Firewall Policies by specifying details about the policy to create
[**delete_firewall_policies**](FirewallPoliciesApi.md#delete_firewall_policies) | **DELETE** /policy/entities/firewall/v1 | Delete a set of Firewall Policies by specifying their IDs
[**get_firewall_policies**](FirewallPoliciesApi.md#get_firewall_policies) | **GET** /policy/entities/firewall/v1 | Retrieve a set of Firewall Policies by specifying their IDs
[**perform_firewall_policies_action**](FirewallPoliciesApi.md#perform_firewall_policies_action) | **POST** /policy/entities/firewall-actions/v1 | Perform the specified action on the Firewall Policies specified in the request
[**query_combined_firewall_policies**](FirewallPoliciesApi.md#query_combined_firewall_policies) | **GET** /policy/combined/firewall/v1 | Search for Firewall Policies in your environment by providing an FQL filter and paging details. Returns a set of Firewall Policies which match the filter criteria
[**query_combined_firewall_policy_members**](FirewallPoliciesApi.md#query_combined_firewall_policy_members) | **GET** /policy/combined/firewall-members/v1 | Search for members of a Firewall Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
[**query_firewall_policies**](FirewallPoliciesApi.md#query_firewall_policies) | **GET** /policy/queries/firewall/v1 | Search for Firewall Policies in your environment by providing an FQL filter and paging details. Returns a set of Firewall Policy IDs which match the filter criteria
[**query_firewall_policy_members**](FirewallPoliciesApi.md#query_firewall_policy_members) | **GET** /policy/queries/firewall-members/v1 | Search for members of a Firewall Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
[**set_firewall_policies_precedence**](FirewallPoliciesApi.md#set_firewall_policies_precedence) | **POST** /policy/entities/firewall-precedence/v1 | Sets the precedence of Firewall Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence
[**update_firewall_policies**](FirewallPoliciesApi.md#update_firewall_policies) | **PATCH** /policy/entities/firewall/v1 | Update Firewall Policies by specifying the ID of the policy and details to update

## create_firewall_policies

> models::FirewallPeriodRespV1 create_firewall_policies(body, clone_id)
Create Firewall Policies by specifying details about the policy to create

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FirewallPeriodCreateFirewallPoliciesReqV1**](FirewallPeriodCreateFirewallPoliciesReqV1.md) |  | [required] |
**clone_id** | Option<**String**> | The policy ID to be cloned from |  |

### Return type

[**models::FirewallPeriodRespV1**](firewall.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_firewall_policies

> models::MsaPeriodQueryResponse delete_firewall_policies(ids)
Delete a set of Firewall Policies by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Firewall Policies to delete | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_firewall_policies

> models::FirewallPeriodRespV1 get_firewall_policies(ids)
Retrieve a set of Firewall Policies by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Firewall Policies to return | [required] |

### Return type

[**models::FirewallPeriodRespV1**](firewall.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## perform_firewall_policies_action

> models::FirewallPeriodRespV1 perform_firewall_policies_action(action_name, body)
Perform the specified action on the Firewall Policies specified in the request

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_name** | **String** | The action to perform | [required] |
**body** | [**MsaPeriodEntityActionRequestV2**](MsaPeriodEntityActionRequestV2.md) |  | [required] |

### Return type

[**models::FirewallPeriodRespV1**](firewall.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_firewall_policies

> models::FirewallPeriodRespV1 query_combined_firewall_policies(filter, offset, limit, sort)
Search for Firewall Policies in your environment by providing an FQL filter and paging details. Returns a set of Firewall Policies which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::FirewallPeriodRespV1**](firewall.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_firewall_policy_members

> models::BasePeriodPolicyMembersRespV1 query_combined_firewall_policy_members(id, filter, offset, limit, sort)
Search for members of a Firewall Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Firewall Policy to search for members of |  |
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

## query_firewall_policies

> models::MsaPeriodQueryResponse query_firewall_policies(filter, offset, limit, sort)
Search for Firewall Policies in your environment by providing an FQL filter and paging details. Returns a set of Firewall Policy IDs which match the filter criteria

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

## query_firewall_policy_members

> models::MsaPeriodQueryResponse query_firewall_policy_members(id, filter, offset, limit, sort)
Search for members of a Firewall Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Firewall Policy to search for members of |  |
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

## set_firewall_policies_precedence

> models::MsaPeriodQueryResponse set_firewall_policies_precedence(body)
Sets the precedence of Firewall Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence

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

## update_firewall_policies

> models::FirewallPeriodRespV1 update_firewall_policies(body)
Update Firewall Policies by specifying the ID of the policy and details to update

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FirewallPeriodUpdateFirewallPoliciesReqV1**](FirewallPeriodUpdateFirewallPoliciesReqV1.md) |  | [required] |

### Return type

[**models::FirewallPeriodRespV1**](firewall.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
