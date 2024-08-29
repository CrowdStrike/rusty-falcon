# \PreventionPoliciesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_prevention_policies**](PreventionPoliciesApi.md#create_prevention_policies) | **POST** /policy/entities/prevention/v1 | Create Prevention Policies by specifying details about the policy to create
[**delete_prevention_policies**](PreventionPoliciesApi.md#delete_prevention_policies) | **DELETE** /policy/entities/prevention/v1 | Delete a set of Prevention Policies by specifying their IDs
[**get_prevention_policies**](PreventionPoliciesApi.md#get_prevention_policies) | **GET** /policy/entities/prevention/v1 | Retrieve a set of Prevention Policies by specifying their IDs
[**perform_prevention_policies_action**](PreventionPoliciesApi.md#perform_prevention_policies_action) | **POST** /policy/entities/prevention-actions/v1 | Perform the specified action on the Prevention Policies specified in the request
[**query_combined_prevention_policies**](PreventionPoliciesApi.md#query_combined_prevention_policies) | **GET** /policy/combined/prevention/v1 | Search for Prevention Policies in your environment by providing an FQL filter and paging details. Returns a set of Prevention Policies which match the filter criteria
[**query_combined_prevention_policy_members**](PreventionPoliciesApi.md#query_combined_prevention_policy_members) | **GET** /policy/combined/prevention-members/v1 | Search for members of a Prevention Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
[**query_prevention_policies**](PreventionPoliciesApi.md#query_prevention_policies) | **GET** /policy/queries/prevention/v1 | Search for Prevention Policies in your environment by providing an FQL filter and paging details. Returns a set of Prevention Policy IDs which match the filter criteria
[**query_prevention_policy_members**](PreventionPoliciesApi.md#query_prevention_policy_members) | **GET** /policy/queries/prevention-members/v1 | Search for members of a Prevention Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
[**set_prevention_policies_precedence**](PreventionPoliciesApi.md#set_prevention_policies_precedence) | **POST** /policy/entities/prevention-precedence/v1 | Sets the precedence of Prevention Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence
[**update_prevention_policies**](PreventionPoliciesApi.md#update_prevention_policies) | **PATCH** /policy/entities/prevention/v1 | Update Prevention Policies by specifying the ID of the policy and details to update

## create_prevention_policies

> models::PreventionPeriodRespV1 create_prevention_policies(body)
Create Prevention Policies by specifying details about the policy to create

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PreventionPeriodCreatePoliciesReqV1**](PreventionPeriodCreatePoliciesReqV1.md) |  | [required] |

### Return type

[**models::PreventionPeriodRespV1**](prevention.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_prevention_policies

> models::MsaPeriodQueryResponse delete_prevention_policies(ids)
Delete a set of Prevention Policies by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Prevention Policies to delete | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_prevention_policies

> models::PreventionPeriodRespV1 get_prevention_policies(ids)
Retrieve a set of Prevention Policies by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Prevention Policies to return | [required] |

### Return type

[**models::PreventionPeriodRespV1**](prevention.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## perform_prevention_policies_action

> models::PreventionPeriodRespV1 perform_prevention_policies_action(action_name, body)
Perform the specified action on the Prevention Policies specified in the request

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_name** | **String** | The action to perform | [required] |
**body** | [**MsaPeriodEntityActionRequestV2**](MsaPeriodEntityActionRequestV2.md) |  | [required] |

### Return type

[**models::PreventionPeriodRespV1**](prevention.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_prevention_policies

> models::PreventionPeriodRespV1 query_combined_prevention_policies(filter, offset, limit, sort)
Search for Prevention Policies in your environment by providing an FQL filter and paging details. Returns a set of Prevention Policies which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::PreventionPeriodRespV1**](prevention.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_prevention_policy_members

> models::BasePeriodPolicyMembersRespV1 query_combined_prevention_policy_members(id, filter, offset, limit, sort)
Search for members of a Prevention Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Prevention Policy to search for members of |  |
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

## query_prevention_policies

> models::MsaPeriodQueryResponse query_prevention_policies(filter, offset, limit, sort)
Search for Prevention Policies in your environment by providing an FQL filter and paging details. Returns a set of Prevention Policy IDs which match the filter criteria

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

## query_prevention_policy_members

> models::MsaPeriodQueryResponse query_prevention_policy_members(id, filter, offset, limit, sort)
Search for members of a Prevention Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Prevention Policy to search for members of |  |
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

## set_prevention_policies_precedence

> models::MsaPeriodQueryResponse set_prevention_policies_precedence(body)
Sets the precedence of Prevention Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence

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

## update_prevention_policies

> models::PreventionPeriodRespV1 update_prevention_policies(body)
Update Prevention Policies by specifying the ID of the policy and details to update

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PreventionPeriodUpdatePoliciesReqV1**](PreventionPeriodUpdatePoliciesReqV1.md) |  | [required] |

### Return type

[**models::PreventionPeriodRespV1**](prevention.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
