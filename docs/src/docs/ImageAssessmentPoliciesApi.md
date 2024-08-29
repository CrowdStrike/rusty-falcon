# \ImageAssessmentPoliciesApi

All URIs are relative to *<https://api.crowdstrike.com>*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policies**](ImageAssessmentPoliciesApi.md#create_policies) | **POST** /container-security/entities/image-assessment-policies/v1 | Create Image Assessment policies
[**create_policy_groups**](ImageAssessmentPoliciesApi.md#create_policy_groups) | **POST** /container-security/entities/image-assessment-policy-groups/v1 | Create Image Assessment Policy Group entities
[**delete_policy**](ImageAssessmentPoliciesApi.md#delete_policy) | **DELETE** /container-security/entities/image-assessment-policies/v1 | Delete Image Assessment Policy by policy UUID
[**delete_policy_group**](ImageAssessmentPoliciesApi.md#delete_policy_group) | **DELETE** /container-security/entities/image-assessment-policy-groups/v1 | Delete Image Assessment Policy Group entities
[**read_policies**](ImageAssessmentPoliciesApi.md#read_policies) | **GET** /container-security/entities/image-assessment-policies/v1 | Get all Image Assessment policies
[**read_policy_exclusions**](ImageAssessmentPoliciesApi.md#read_policy_exclusions) | **GET** /container-security/entities/image-assessment-policy-exclusions/v1 | Retrieve Image Assessment Policy Exclusion entities
[**read_policy_groups**](ImageAssessmentPoliciesApi.md#read_policy_groups) | **GET** /container-security/entities/image-assessment-policy-groups/v1 | Retrieve Image Assessment Policy Group entities
[**update_policies**](ImageAssessmentPoliciesApi.md#update_policies) | **PATCH** /container-security/entities/image-assessment-policies/v1 | Update Image Assessment Policy entities
[**update_policy_exclusions**](ImageAssessmentPoliciesApi.md#update_policy_exclusions) | **POST** /container-security/entities/image-assessment-policy-exclusions/v1 | Update Image Assessment Policy Exclusion entities
[**update_policy_groups**](ImageAssessmentPoliciesApi.md#update_policy_groups) | **PATCH** /container-security/entities/image-assessment-policy-groups/v1 | Update Image Assessment Policy Group entities
[**update_policy_precedence**](ImageAssessmentPoliciesApi.md#update_policy_precedence) | **POST** /container-security/entities/image-assessment-policy-precedence/v1 | Update Image Assessment Policy precedence

## create_policies

> models::ModelsPeriodPolicyEntityResponse create_policies(body)
Create Image Assessment policies

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodCreatePolicyRequest**](ModelsPeriodCreatePolicyRequest.md) |  | [required] |

### Return type

[**models::ModelsPeriodPolicyEntityResponse**](models.PolicyEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## create_policy_groups

> models::ModelsPeriodPolicyGroupEntityResponse create_policy_groups(body)
Create Image Assessment Policy Group entities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodCreateImageGroupRequest**](ModelsPeriodCreateImageGroupRequest.md) |  | [required] |

### Return type

[**models::ModelsPeriodPolicyGroupEntityResponse**](models.PolicyGroupEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_policy

> models::CorePeriodEntitiesResponse delete_policy(id)
Delete Image Assessment Policy by policy UUID

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Image Assessment Policy entity UUID | [required] |

### Return type

[**models::CorePeriodEntitiesResponse**](core.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_policy_group

> models::CorePeriodEntitiesResponse delete_policy_group(id)
Delete Image Assessment Policy Group entities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Policy Image Group entity UUID | [required] |

### Return type

[**models::CorePeriodEntitiesResponse**](core.EntitiesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_policies

> models::ModelsPeriodPolicyEntityResponse read_policies()
Get all Image Assessment policies

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelsPeriodPolicyEntityResponse**](models.PolicyEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_policy_exclusions

> models::ModelsPeriodPolicyExclusionEntityResponse read_policy_exclusions()
Retrieve Image Assessment Policy Exclusion entities

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelsPeriodPolicyExclusionEntityResponse**](models.PolicyExclusionEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## read_policy_groups

> models::ModelsPeriodPolicyGroupEntityResponse read_policy_groups()
Retrieve Image Assessment Policy Group entities

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelsPeriodPolicyGroupEntityResponse**](models.PolicyGroupEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_policies

> models::ModelsPeriodPolicyEntityResponse update_policies(id, body)
Update Image Assessment Policy entities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Image Assessment Policy entity UUID | [required] |
**body** | [**ModelsPeriodPatchPolicyRequest**](ModelsPeriodPatchPolicyRequest.md) |  | [required] |

### Return type

[**models::ModelsPeriodPolicyEntityResponse**](models.PolicyEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_policy_exclusions

> models::ModelsPeriodPolicyExclusionEntityResponse update_policy_exclusions(body)
Update Image Assessment Policy Exclusion entities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodUpdateExclusionsRequest**](ModelsPeriodUpdateExclusionsRequest.md) |  | [required] |

### Return type

[**models::ModelsPeriodPolicyExclusionEntityResponse**](models.PolicyExclusionEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_policy_groups

> models::ModelsPeriodPolicyGroupEntityResponse update_policy_groups(id, body)
Update Image Assessment Policy Group entities

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Policy Image Group entity UUID | [required] |
**body** | [**ModelsPeriodPatchImageGroupRequest**](ModelsPeriodPatchImageGroupRequest.md) |  | [required] |

### Return type

[**models::ModelsPeriodPolicyGroupEntityResponse**](models.PolicyGroupEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_policy_precedence

> models::ModelsPeriodPolicyEntityResponse update_policy_precedence(body)
Update Image Assessment Policy precedence

### Parameters

Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ModelsPeriodApiPrecedenceRequest**](ModelsPeriodApiPrecedenceRequest.md) |  | [required] |

### Return type

[**models::ModelsPeriodPolicyEntityResponse**](models.PolicyEntityResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
