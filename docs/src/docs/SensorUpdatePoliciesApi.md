# \SensorUpdatePoliciesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sensor_update_policies**](SensorUpdatePoliciesApi.md#create_sensor_update_policies) | **POST** /policy/entities/sensor-update/v1 | Create Sensor Update Policies by specifying details about the policy to create
[**create_sensor_update_policies_v2**](SensorUpdatePoliciesApi.md#create_sensor_update_policies_v2) | **POST** /policy/entities/sensor-update/v2 | Create Sensor Update Policies by specifying details about the policy to create with additional support for uninstall protection
[**delete_sensor_update_policies**](SensorUpdatePoliciesApi.md#delete_sensor_update_policies) | **DELETE** /policy/entities/sensor-update/v1 | Delete a set of Sensor Update Policies by specifying their IDs
[**get_sensor_update_policies**](SensorUpdatePoliciesApi.md#get_sensor_update_policies) | **GET** /policy/entities/sensor-update/v1 | Retrieve a set of Sensor Update Policies by specifying their IDs
[**get_sensor_update_policies_v2**](SensorUpdatePoliciesApi.md#get_sensor_update_policies_v2) | **GET** /policy/entities/sensor-update/v2 | Retrieve a set of Sensor Update Policies with additional support for uninstall protection by specifying their IDs
[**perform_sensor_update_policies_action**](SensorUpdatePoliciesApi.md#perform_sensor_update_policies_action) | **POST** /policy/entities/sensor-update-actions/v1 | Perform the specified action on the Sensor Update Policies specified in the request
[**query_combined_sensor_update_builds**](SensorUpdatePoliciesApi.md#query_combined_sensor_update_builds) | **GET** /policy/combined/sensor-update-builds/v1 | Retrieve available builds for use with Sensor Update Policies
[**query_combined_sensor_update_kernels**](SensorUpdatePoliciesApi.md#query_combined_sensor_update_kernels) | **GET** /policy/combined/sensor-update-kernels/v1 | Retrieve kernel compatibility info for Sensor Update Builds
[**query_combined_sensor_update_policies**](SensorUpdatePoliciesApi.md#query_combined_sensor_update_policies) | **GET** /policy/combined/sensor-update/v1 | Search for Sensor Update Policies in your environment by providing an FQL filter and paging details. Returns a set of Sensor Update Policies which match the filter criteria
[**query_combined_sensor_update_policies_v2**](SensorUpdatePoliciesApi.md#query_combined_sensor_update_policies_v2) | **GET** /policy/combined/sensor-update/v2 | Search for Sensor Update Policies with additional support for uninstall protection in your environment by providing an FQL filter and paging details. Returns a set of Sensor Update Policies which match the filter criteria
[**query_combined_sensor_update_policy_members**](SensorUpdatePoliciesApi.md#query_combined_sensor_update_policy_members) | **GET** /policy/combined/sensor-update-members/v1 | Search for members of a Sensor Update Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria
[**query_sensor_update_kernels_distinct**](SensorUpdatePoliciesApi.md#query_sensor_update_kernels_distinct) | **GET** /policy/queries/sensor-update-kernels/{distinct-field}/v1 | Retrieve kernel compatibility info for Sensor Update Builds
[**query_sensor_update_policies**](SensorUpdatePoliciesApi.md#query_sensor_update_policies) | **GET** /policy/queries/sensor-update/v1 | Search for Sensor Update Policies in your environment by providing an FQL filter and paging details. Returns a set of Sensor Update Policy IDs which match the filter criteria
[**query_sensor_update_policy_members**](SensorUpdatePoliciesApi.md#query_sensor_update_policy_members) | **GET** /policy/queries/sensor-update-members/v1 | Search for members of a Sensor Update Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria
[**reveal_uninstall_token**](SensorUpdatePoliciesApi.md#reveal_uninstall_token) | **POST** /policy/combined/reveal-uninstall-token/v1 | Reveals an uninstall token for a specific device. To retrieve the bulk maintenance token pass the value 'MAINTENANCE' as the value for 'device_id'
[**set_sensor_update_policies_precedence**](SensorUpdatePoliciesApi.md#set_sensor_update_policies_precedence) | **POST** /policy/entities/sensor-update-precedence/v1 | Sets the precedence of Sensor Update Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence
[**update_sensor_update_policies**](SensorUpdatePoliciesApi.md#update_sensor_update_policies) | **PATCH** /policy/entities/sensor-update/v1 | Update Sensor Update Policies by specifying the ID of the policy and details to update
[**update_sensor_update_policies_v2**](SensorUpdatePoliciesApi.md#update_sensor_update_policies_v2) | **PATCH** /policy/entities/sensor-update/v2 | Update Sensor Update Policies by specifying the ID of the policy and details to update with additional support for uninstall protection

## create_sensor_update_policies

> models::SensorUpdatePeriodRespV1 create_sensor_update_policies(body)
Create Sensor Update Policies by specifying details about the policy to create

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SensorUpdatePeriodCreatePoliciesReqV1**](SensorUpdatePeriodCreatePoliciesReqV1.md) |  | [required] |

### Return type

[**models::SensorUpdatePeriodRespV1**](sensor_update.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_sensor_update_policies_v2

> models::SensorUpdatePeriodRespV2 create_sensor_update_policies_v2(body)
Create Sensor Update Policies by specifying details about the policy to create with additional support for uninstall protection

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SensorUpdatePeriodCreatePoliciesReqV2**](SensorUpdatePeriodCreatePoliciesReqV2.md) |  | [required] |

### Return type

[**models::SensorUpdatePeriodRespV2**](sensor_update.RespV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_sensor_update_policies

> models::MsaPeriodQueryResponse delete_sensor_update_policies(ids)
Delete a set of Sensor Update Policies by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Sensor Update Policies to delete | [required] |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sensor_update_policies

> models::SensorUpdatePeriodRespV1 get_sensor_update_policies(ids)
Retrieve a set of Sensor Update Policies by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Sensor Update Policies to return | [required] |

### Return type

[**models::SensorUpdatePeriodRespV1**](sensor_update.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_sensor_update_policies_v2

> models::SensorUpdatePeriodRespV2 get_sensor_update_policies_v2(ids)
Retrieve a set of Sensor Update Policies with additional support for uninstall protection by specifying their IDs

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) | The IDs of the Sensor Update Policies to return | [required] |

### Return type

[**models::SensorUpdatePeriodRespV2**](sensor_update.RespV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## perform_sensor_update_policies_action

> models::SensorUpdatePeriodRespV1 perform_sensor_update_policies_action(action_name, body)
Perform the specified action on the Sensor Update Policies specified in the request

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_name** | **String** | The action to perform | [required] |
**body** | [**MsaPeriodEntityActionRequestV2**](MsaPeriodEntityActionRequestV2.md) |  | [required] |

### Return type

[**models::SensorUpdatePeriodRespV1**](sensor_update.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_sensor_update_builds

> models::SensorUpdatePeriodBuildsRespV1 query_combined_sensor_update_builds(platform, stage)
Retrieve available builds for use with Sensor Update Policies

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<**String**> | The platform to return builds for |  |
**stage** | Option<[**Vec<String>**](String.md)> | The stages to return builds for |  |

### Return type

[**models::SensorUpdatePeriodBuildsRespV1**](sensor_update.BuildsRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_sensor_update_kernels

> models::SensorUpdatePeriodKernelsRespV1 query_combined_sensor_update_kernels(filter, offset, limit)
Retrieve kernel compatibility info for Sensor Update Builds

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |

### Return type

[**models::SensorUpdatePeriodKernelsRespV1**](sensor_update.KernelsRespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_sensor_update_policies

> models::SensorUpdatePeriodRespV1 query_combined_sensor_update_policies(filter, offset, limit, sort)
Search for Sensor Update Policies in your environment by providing an FQL filter and paging details. Returns a set of Sensor Update Policies which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::SensorUpdatePeriodRespV1**](sensor_update.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_sensor_update_policies_v2

> models::SensorUpdatePeriodRespV2 query_combined_sensor_update_policies_v2(filter, offset, limit, sort)
Search for Sensor Update Policies with additional support for uninstall protection in your environment by providing an FQL filter and paging details. Returns a set of Sensor Update Policies which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-5000] |  |
**sort** | Option<**String**> | The property to sort by |  |

### Return type

[**models::SensorUpdatePeriodRespV2**](sensor_update.RespV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_combined_sensor_update_policy_members

> models::BasePeriodPolicyMembersRespV1 query_combined_sensor_update_policy_members(id, filter, offset, limit, sort)
Search for members of a Sensor Update Policy in your environment by providing an FQL filter and paging details. Returns a set of host details which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Sensor Update Policy to search for members of |  |
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

## query_sensor_update_kernels_distinct

> models::MsaPeriodQueryResponse query_sensor_update_kernels_distinct(distinct_field, filter, offset, limit)
Retrieve kernel compatibility info for Sensor Update Builds

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**distinct_field** | **String** | The field name to get distinct values for | [required] |
**filter** | Option<**String**> | The filter expression that should be used to limit the results |  |
**offset** | Option<**i32**> | The offset to start retrieving records from |  |
**limit** | Option<**i32**> | The maximum records to return. [1-500] |  |

### Return type

[**models::MsaPeriodQueryResponse**](msa.QueryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## query_sensor_update_policies

> models::MsaPeriodQueryResponse query_sensor_update_policies(filter, offset, limit, sort)
Search for Sensor Update Policies in your environment by providing an FQL filter and paging details. Returns a set of Sensor Update Policy IDs which match the filter criteria

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

## query_sensor_update_policy_members

> models::MsaPeriodQueryResponse query_sensor_update_policy_members(id, filter, offset, limit, sort)
Search for members of a Sensor Update Policy in your environment by providing an FQL filter and paging details. Returns a set of Agent IDs which match the filter criteria

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Sensor Update Policy to search for members of |  |
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

## reveal_uninstall_token

> models::UninstallTokenPeriodRespV1 reveal_uninstall_token(body)
Reveals an uninstall token for a specific device. To retrieve the bulk maintenance token pass the value 'MAINTENANCE' as the value for 'device_id'

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UninstallTokenPeriodRevealUninstallTokenReqV1**](UninstallTokenPeriodRevealUninstallTokenReqV1.md) |  | [required] |

### Return type

[**models::UninstallTokenPeriodRespV1**](uninstall_token.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## set_sensor_update_policies_precedence

> models::MsaPeriodQueryResponse set_sensor_update_policies_precedence(body)
Sets the precedence of Sensor Update Policies based on the order of IDs specified in the request. The first ID specified will have the highest precedence and the last ID specified will have the lowest. You must specify all non-Default Policies for a platform when updating precedence

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

## update_sensor_update_policies

> models::SensorUpdatePeriodRespV1 update_sensor_update_policies(body)
Update Sensor Update Policies by specifying the ID of the policy and details to update

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SensorUpdatePeriodUpdatePoliciesReqV1**](SensorUpdatePeriodUpdatePoliciesReqV1.md) |  | [required] |

### Return type

[**models::SensorUpdatePeriodRespV1**](sensor_update.RespV1.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_sensor_update_policies_v2

> models::SensorUpdatePeriodRespV2 update_sensor_update_policies_v2(body)
Update Sensor Update Policies by specifying the ID of the policy and details to update with additional support for uninstall protection

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SensorUpdatePeriodUpdatePoliciesReqV2**](SensorUpdatePeriodUpdatePoliciesReqV2.md) |  | [required] |

### Return type

[**models::SensorUpdatePeriodRespV2**](sensor_update.RespV2.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
